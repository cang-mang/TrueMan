#include <trueman/util/hash.h>
#include <stdio.h>

#define TST_assert_eq(x, y) do { \
    if((x) != (y)) { \
        printf( \
            "failure:\n" \
            "\t---- %s:%u (%s) ----\n" \
            "\t left: %u\n" \
            "\tright: %u\n", \
            __FILE__, (unsigned)__LINE__, __FUNCTION__, \
            (unsigned)(x), (unsigned)(y) \
        ); \
        return(-1); \
    } \
} while(0)

int main(void) {
    const char str[] = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^";

    TrueMan_U32_t h = TrueMan_UTIL_HASH_BKDR_x_0(
        str, sizeof(str) - 1, 131, 0
    );
    TST_assert_eq(h, 3255416723);

    h = TrueMan_UTIL_HASH_BKDR_time33(str, sizeof(str) - 1, 0);
    TST_assert_eq(h, 2622491663);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
