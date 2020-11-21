#include <trueman/util/code.h>
#include <cstring>
#include <iostream>

#define TST_assert_eq(x, y) do { \
    if((x) != (y)) { \
        std::cout << "failure:" << std::endl; \
        std::cout << "\t---- " << __FILE__ << ':' << __LINE__; \
        std::cout << " (" << __FUNCTION__ << ") ----" << std::endl; \
        std::cout << "\t left: " << (unsigned long long)(x) << std::endl; \
        std::cout << "\tright: " << (unsigned long long)(y) << std::endl; \
        return(-1); \
    } \
} while(0)

int main() {
    std::uint8_t buf[7];
    memset(buf, 255, sizeof(buf));

    TrueMan::util::code::uintx::encode_be(buf, sizeof(buf), 0x0A1B2C3D4E5F6879);
    TST_assert_eq(buf[0], 0x1B);
    TST_assert_eq(buf[1], 0x2C);
    TST_assert_eq(buf[2], 0x3D);
    TST_assert_eq(buf[3], 0x4E);
    TST_assert_eq(buf[4], 0x5F);
    TST_assert_eq(buf[5], 0x68);
    TST_assert_eq(buf[6], 0x79);

    std::uint64_t val = TrueMan::util::code::uintx::decode_le(buf, sizeof(buf));
    TST_assert_eq(val, 0x0079685F4E3D2C1B);

    TrueMan::util::code::uintx::encode_le(buf, sizeof(buf), 0x0A1B2C3D4E5F6879);
    TST_assert_eq(buf[0], 0x79);
    TST_assert_eq(buf[1], 0x68);
    TST_assert_eq(buf[2], 0x5F);
    TST_assert_eq(buf[3], 0x4E);
    TST_assert_eq(buf[4], 0x3D);
    TST_assert_eq(buf[5], 0x2C);
    TST_assert_eq(buf[6], 0x1B);

    val = TrueMan::util::code::uintx::decode_be(buf, sizeof(buf));
    TST_assert_eq(val, 0x0079685F4E3D2C1B);

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
