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

    TrueMan_U32_t x = TrueMan_UTIL_HASH_XX_x32(str, sizeof(str) - 1, 0 - 1);
    TST_assert_eq(x, 2599008376ULL);

    TrueMan_U64_t y = TrueMan_UTIL_HASH_XX_x64(str, sizeof(str) - 1, 0 - 1);
    TST_assert_eq(y, 17226521755326120047ULL);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
