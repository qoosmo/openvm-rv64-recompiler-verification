use rv64w::{RANDOM_CASES_PER_OP, RANDOM_SEED, boundary_vectors, random_vectors, reference};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

const COMMIT: &str = "594b044e705891bcc2abd01578410da7dfcd1efe";
const UPSTREAM_SOURCE_SHA256: &str =
    "ad9fe1b1739243afbadc324605185e66d3f9d083cfa3e4392a258bf47713dae1";
const PRNG: &str = "SplitMix64";
const C_FLAGS: &str =
    "-std=c11 -O2 -Wall -Wextra -Werror -fsanitize=undefined -fno-sanitize-recover=undefined";

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ResultManifest {
    schema_version: u32,
    mission: String,
    result_class: String,
    equivalence_notion: String,
    formal_proof: bool,
    upstream_commit: String,
    upstream_source_sha256: String,
    prng: String,
    random_seed_hex: String,
    vector_corpus_sha256: String,
    boundary_vectors: usize,
    random_vectors_per_operation: usize,
    operations: usize,
    total_vectors: usize,
    mismatches: usize,
    counterexample_found: bool,
    compiler: String,
    compiler_flags: String,
    c_standard: String,
    undefined_behavior_sanitizer: bool,
    host_os: String,
    host_arch: String,
    elapsed_ms: u128,
    timeout_seconds: Option<u64>,
    interventions: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest_mode = match args.as_slice() {
        [] => None,
        [flag, path] if flag == "--manifest" => Some((false, PathBuf::from(path))),
        [flag, path] if flag == "--check-manifest" => Some((true, PathBuf::from(path))),
        _ => return Err("usage: code001 [--manifest PATH | --check-manifest PATH]".into()),
    };

    let started = Instant::now();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target = root.join("target/code001");
    fs::create_dir_all(&target)?;
    let executable = target.join("emitted_harness");
    check_c_transcription(&root.join("c/emitted_harness.c"))?;
    let compiler = compile_c(&root, &executable)?;

    let boundary = boundary_vectors();
    let boundary_count = boundary.len();
    compare_rust_models(&boundary)?;
    let random = random_vectors(RANDOM_SEED, RANDOM_CASES_PER_OP);
    compare_rust_models(&random)?;

    let mut all = boundary;
    all.extend(random);
    let corpus_sha256 = compare_compiled_c(&target, &executable, &all)?;

    let elapsed_ms = started.elapsed().as_millis();
    let summary = format!(
        "CODE-001 PASS: {} boundary + {} randomized vectors; Rust models and compiled C agree",
        boundary_count,
        RANDOM_CASES_PER_OP * 5
    );
    println!("{summary}");

    if let Some((check, path)) = manifest_mode {
        if check {
            check_manifest(&path, boundary_count, all.len(), &corpus_sha256)?;
        } else {
            write_manifest(
                &path,
                boundary_count,
                all.len(),
                elapsed_ms,
                &compiler,
                &corpus_sha256,
            )?;
        }
    }
    Ok(())
}

fn check_c_transcription(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let expressions = [
        "(uint64_t)(int32_t)((uint32_t)lhs + (uint32_t)rhs)",
        "(uint64_t)(int32_t)((uint32_t)lhs - (uint32_t)rhs)",
        "(uint64_t)(int32_t)((uint32_t)lhs << ((uint32_t)rhs & 0x1fu))",
        "(uint64_t)(int32_t)((uint32_t)lhs >> ((uint32_t)rhs & 0x1fu))",
        "(uint64_t)(int32_t)((uint32_t)((int32_t)(uint32_t)lhs >> ((uint32_t)rhs & 0x1fu)))",
    ];
    for expression in expressions {
        if !source.contains(expression) {
            return Err(format!(
                "C harness no longer contains pinned emitted expression: {expression}"
            )
            .into());
        }
    }
    Ok(())
}

fn compile_c(root: &Path, executable: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let version = Command::new("clang").arg("--version").output()?;
    if !version.status.success() {
        return Err("clang --version failed".into());
    }
    let status = Command::new("clang")
        .args(C_FLAGS.split_ascii_whitespace())
        .arg(root.join("c/emitted_harness.c"))
        .arg("-o")
        .arg(executable)
        .status()?;
    if !status.success() {
        return Err("Clang compilation failed".into());
    }
    Ok(String::from_utf8(version.stdout)?
        .lines()
        .next()
        .unwrap_or("clang")
        .to_owned())
}

fn compare_rust_models(vectors: &[(rv64w::Op, u64, u64)]) -> Result<(), String> {
    for &(op, lhs, rhs) in vectors {
        let expected = reference(op, lhs, rhs);
        let actual = rv64w::emitted_shape(op, lhs, rhs);
        if expected != actual {
            return Err(format!(
                "Rust model mismatch: {} lhs={lhs:#018x} rhs={rhs:#018x} reference={expected:#018x} emitted={actual:#018x}",
                op.name()
            ));
        }
    }
    Ok(())
}

fn compare_compiled_c(
    target: &Path,
    executable: &Path,
    vectors: &[(rv64w::Op, u64, u64)],
) -> Result<String, Box<dyn std::error::Error>> {
    let input_path = target.join("vectors.txt");
    let mut input = File::create(&input_path)?;
    for &(op, lhs, rhs) in vectors {
        writeln!(input, "{} {lhs:016x} {rhs:016x}", op.id())?;
    }
    drop(input);
    let input_bytes = fs::read(&input_path)?;
    let corpus_sha256 = format!("{:x}", Sha256::digest(&input_bytes));

    let output = Command::new(executable)
        .stdin(Stdio::from(File::open(input_path)?))
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "compiled C harness failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    let mut lines = stdout.lines();
    for &(op, lhs, rhs) in vectors {
        let line = lines.next().ok_or("compiled C returned too few results")?;
        let actual = u64::from_str_radix(line, 16)?;
        let expected = reference(op, lhs, rhs);
        if actual != expected {
            return Err(format!(
                "C mismatch: {} lhs={lhs:#018x} rhs={rhs:#018x} reference={expected:#018x} C={actual:#018x}",
                op.name()
            )
            .into());
        }
    }
    if lines.next().is_some() {
        return Err("compiled C returned too many results".into());
    }
    Ok(corpus_sha256)
}

fn write_manifest(
    path: &Path,
    boundary_count: usize,
    total_count: usize,
    elapsed_ms: u128,
    compiler: &str,
    corpus_sha256: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        return Err(format!("refusing to overwrite result artifact {}", path.display()).into());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let manifest = ResultManifest {
        schema_version: 1,
        mission: "CODE-001".to_owned(),
        result_class: "PASS_SIMULATION".to_owned(),
        equivalence_notion: "concrete differential agreement over recorded vectors".to_owned(),
        formal_proof: false,
        upstream_commit: COMMIT.to_owned(),
        upstream_source_sha256: UPSTREAM_SOURCE_SHA256.to_owned(),
        prng: PRNG.to_owned(),
        random_seed_hex: format!("0x{RANDOM_SEED:016x}"),
        vector_corpus_sha256: corpus_sha256.to_owned(),
        boundary_vectors: boundary_count,
        random_vectors_per_operation: RANDOM_CASES_PER_OP,
        operations: 5,
        total_vectors: total_count,
        mismatches: 0,
        counterexample_found: false,
        compiler: compiler.to_owned(),
        compiler_flags: C_FLAGS.to_owned(),
        c_standard: "C11".to_owned(),
        undefined_behavior_sanitizer: true,
        host_os: std::env::consts::OS.to_owned(),
        host_arch: std::env::consts::ARCH.to_owned(),
        elapsed_ms,
        timeout_seconds: None,
        interventions: 0,
    };
    fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(&manifest)?),
    )?;
    Ok(())
}

fn check_manifest(
    path: &Path,
    boundary_count: usize,
    total_count: usize,
    corpus_sha256: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let manifest: ResultManifest = serde_json::from_str(&text)?;
    let valid = manifest.schema_version == 1
        && manifest.mission == "CODE-001"
        && manifest.result_class == "PASS_SIMULATION"
        && !manifest.formal_proof
        && manifest.upstream_commit == COMMIT
        && manifest.upstream_source_sha256 == UPSTREAM_SOURCE_SHA256
        && manifest.prng == PRNG
        && manifest.random_seed_hex == format!("0x{RANDOM_SEED:016x}")
        && manifest.vector_corpus_sha256 == corpus_sha256
        && manifest.boundary_vectors == boundary_count
        && manifest.random_vectors_per_operation == RANDOM_CASES_PER_OP
        && manifest.operations == 5
        && manifest.total_vectors == total_count
        && manifest.mismatches == 0
        && !manifest.counterexample_found
        && !manifest.compiler.is_empty()
        && manifest.compiler_flags == C_FLAGS
        && manifest.c_standard == "C11"
        && manifest.undefined_behavior_sanitizer
        && manifest.timeout_seconds.is_none()
        && manifest.interventions == 0;
    if !valid {
        return Err(format!("manifest {} is inconsistent", path.display()).into());
    }
    println!("manifest {} is consistent", path.display());
    Ok(())
}
