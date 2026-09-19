#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

static uint64_t eval_emitted(unsigned op, uint64_t lhs, uint64_t rhs) {
    switch (op) {
    case 0:
        return (uint64_t)(int32_t)((uint32_t)lhs + (uint32_t)rhs);
    case 1:
        return (uint64_t)(int32_t)((uint32_t)lhs - (uint32_t)rhs);
    case 2:
        return (uint64_t)(int32_t)((uint32_t)lhs << ((uint32_t)rhs & 0x1fu));
    case 3:
        return (uint64_t)(int32_t)((uint32_t)lhs >> ((uint32_t)rhs & 0x1fu));
    case 4:
        return (uint64_t)(int32_t)((uint32_t)((int32_t)(uint32_t)lhs >> ((uint32_t)rhs & 0x1fu)));
    default:
        return 0;
    }
}

int main(void) {
    unsigned op;
    uint64_t lhs;
    uint64_t rhs;

    if (sizeof(uint32_t) != 4 || sizeof(int32_t) != 4 || (int32_t)UINT32_C(0xffffffff) != -1 ||
        ((int32_t)-2 >> 1) != -1) {
        fputs("unsupported C implementation\n", stderr);
        return 2;
    }

    while (scanf("%u %" SCNx64 " %" SCNx64, &op, &lhs, &rhs) == 3) {
        if (op > 4) {
            fputs("invalid operation\n", stderr);
            return 3;
        }
        printf("%016" PRIx64 "\n", eval_emitted(op, lhs, rhs));
    }
    if (!feof(stdin)) {
        fputs("invalid input\n", stderr);
        return 4;
    }
    return 0;
}
