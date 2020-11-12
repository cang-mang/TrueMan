#include <trueman/util/hash.h>
#include <stdio.h>

#define TST_assert_eq(x, y) do { \
    if((x) != (y)) { \
        printf( \
            "failure:\n" \
            "\t---- %s:%u (%s) ----\n" \
            "\t left: %llu\n" \
            "\tright: %llu\n", \
            __FILE__, (unsigned)__LINE__, __FUNCTION__, \
            (unsigned long long)(x), (unsigned long long)(y) \
        ); \
        return(-1); \
    } \
} while(0)

int main(void) {
    const char str[] = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^";

    TrueMan_U32_t x = TrueMan_UTIL_HASH_FNV1_x32(str, sizeof(str) - 1);
    TST_assert_eq(x, 4240981924ULL);

    TrueMan_U64_t y = TrueMan_UTIL_HASH_FNV1_x64(str, sizeof(str) - 1);
    TST_assert_eq(y, 7619294831832707076ULL);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
