#include <trueman/util/code.h>
#include <string.h>
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
    TrueMan_U8_t buf[7];
    memset(buf, 255, sizeof(buf));

    TrueMan_UTIL_CODE_UINTX_encode_be(buf, sizeof(buf), 0x0A1B2C3D4E5F6879);
    TST_assert_eq(buf[0], 0x1B);
    TST_assert_eq(buf[1], 0x2C);
    TST_assert_eq(buf[2], 0x3D);
    TST_assert_eq(buf[3], 0x4E);
    TST_assert_eq(buf[4], 0x5F);
    TST_assert_eq(buf[5], 0x68);
    TST_assert_eq(buf[6], 0x79);

    TrueMan_U64_t val = TrueMan_UTIL_CODE_UINTX_decode_le(buf, sizeof(buf));
    TST_assert_eq(val, 0x0079685F4E3D2C1B);

    TrueMan_UTIL_CODE_UINTX_encode_le(buf, sizeof(buf), 0x0A1B2C3D4E5F6879);
    TST_assert_eq(buf[0], 0x79);
    TST_assert_eq(buf[1], 0x68);
    TST_assert_eq(buf[2], 0x5F);
    TST_assert_eq(buf[3], 0x4E);
    TST_assert_eq(buf[4], 0x3D);
    TST_assert_eq(buf[5], 0x2C);
    TST_assert_eq(buf[6], 0x1B);

    val = TrueMan_UTIL_CODE_UINTX_decode_be(buf, sizeof(buf));
    TST_assert_eq(val, 0x0079685F4E3D2C1B);

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
