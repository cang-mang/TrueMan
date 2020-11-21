#include <trueman/util/code.h>
#include <string.h>
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
    const TrueMan_U8_t src[] = {
        0xE6, 0xB1, 0x89, 0xE5, 0xAD, 0x97
    };

    TrueMan_U16_t d_0[128];
    memset(d_0, 255, sizeof(d_0));

    TrueMan_ISIZE_t l_0 = TrueMan_UTIL_CODE_UCS2_decode_utf8(src, sizeof(src), d_0);
    if(l_0 < 0) {
        TST_assert_eq(0, 1);
    }

    TST_assert_eq(l_0, 2);

    TST_assert_eq(d_0[0], 0x6C49);
    TST_assert_eq(d_0[1], 0x5B57);

    TrueMan_U8_t d_1[128];
    memset(d_1, 255, sizeof(d_1));

    TrueMan_USIZE_t l_1 = TrueMan_UTIL_CODE_UCS2_encode_utf8(d_0, l_0, d_1);

    TST_assert_eq(l_1, 6);

    TST_assert_eq(d_1[0], src[0]);
    TST_assert_eq(d_1[1], src[1]);
    TST_assert_eq(d_1[2], src[2]);
    TST_assert_eq(d_1[3], src[3]);
    TST_assert_eq(d_1[4], src[4]);
    TST_assert_eq(d_1[5], src[5]);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
