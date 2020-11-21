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
        0xF0, 0xAA, 0x9A, 0xA5, 0xF0, 0xA4, 0x89, 0xB9
    };

    TrueMan_U32_t d_0[128];
    memset(d_0, 255, sizeof(d_0));

    TrueMan_ISIZE_t l_0 = TrueMan_UTIL_CODE_UCS4_decode_utf8(src, sizeof(src), d_0);
    if(l_0 < 0) {
        TST_assert_eq(0, 1);
    }

    TST_assert_eq(l_0, 2);

    TST_assert_eq(d_0[0], 0x0002A6A5);
    TST_assert_eq(d_0[1], 0x00024279);

    TrueMan_U8_t d_1[128];
    memset(d_1, 255, sizeof(d_1));

    TrueMan_USIZE_t l_1 = TrueMan_UTIL_CODE_UCS4_encode_utf8(d_0, l_0, d_1);

    TST_assert_eq(l_1, 8);

    TST_assert_eq(d_1[0], src[0]);
    TST_assert_eq(d_1[1], src[1]);
    TST_assert_eq(d_1[2], src[2]);
    TST_assert_eq(d_1[3], src[3]);
    TST_assert_eq(d_1[4], src[4]);
    TST_assert_eq(d_1[5], src[5]);
    TST_assert_eq(d_1[6], src[6]);
    TST_assert_eq(d_1[7], src[7]);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
