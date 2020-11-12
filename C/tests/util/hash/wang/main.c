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
    TrueMan_U64_t x = (TrueMan_U64_t)9876543210123456789ULL;
    TrueMan_U64_t y = TrueMan_UTIL_HASH_WANG_direct(x);
    TST_assert_eq(y, 14255830987705448333ULL);

    x = TrueMan_UTIL_HASH_WANG_inverse(y);
    TST_assert_eq(x, 9876543210123456789ULL);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
