# REVIEW-001 — Cross-discipline review

## Formal agent reviews coding output

Check whether the executable reference model matches the theorem statement and whether
the C-harness assumptions are accurately documented.

## Coding agent reviews formal output

Check whether the Lean emitter model matches the pinned OpenVM source expression shape
and test edge cases against the executable harness.

## Gate

Any mismatch becomes an explicit issue before integration. Do not paper over a mismatch
by weakening the theorem or changing the reference model.
