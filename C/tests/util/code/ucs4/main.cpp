#include <trueman/util/code.h>
#include <cstring>
#include <iostream>

#define TST_assert_eq(x, y) do { \
    if((x) != (y)) { \
        std::cout << "failure:" << std::endl; \
        std::cout << "\t---- " << __FILE__ << ':' << __LINE__; \
        std::cout << " (" << __FUNCTION__ << ") ----" << std::endl; \
        std::cout << "\t left: " << (unsigned)(x) << std::endl; \
        std::cout << "\tright: " << (unsigned)(y) << std::endl; \
        return(-1); \
    } \
} while(0)

int main() {
    const std::uint8_t src[] = {
        0xF0, 0xAA, 0x9A, 0xA5, 0xF0, 0xA4, 0x89, 0xB9
    };

    std::uint32_t d_0[128];
    std::memset(d_0, 255, sizeof(d_0));

    std::intptr_t l_0 = TrueMan::util::code::ucs4::decode_utf8(src, sizeof(src), d_0);
    if(l_0 < 0) {
        TST_assert_eq(0, 1);
    }

    TST_assert_eq(l_0, 2);

    TST_assert_eq(d_0[0], 0x0002A6A5);
    TST_assert_eq(d_0[1], 0x00024279);

    std::uint8_t d_1[128];
    std::memset(d_1, 255, sizeof(d_1));

    std::uintptr_t l_1 = TrueMan::util::code::ucs4::encode_utf8(d_0, l_0, d_1);

    TST_assert_eq(l_1, 8);

    TST_assert_eq(d_1[0], src[0]);
    TST_assert_eq(d_1[1], src[1]);
    TST_assert_eq(d_1[2], src[2]);
    TST_assert_eq(d_1[3], src[3]);
    TST_assert_eq(d_1[4], src[4]);
    TST_assert_eq(d_1[5], src[5]);
    TST_assert_eq(d_1[6], src[6]);
    TST_assert_eq(d_1[7], src[7]);

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
