#include <trueman/util/code.h>
#include <cstring>
#include <iostream>

#define TST_assert_eq(x, y) do { \
    if((x) != (y)) { \
        std::cout << "failure:" << std::endl; \
        std::cout << "\t---- " << __FILE__ << ':' << __LINE__; \
        std::cout << " (" << __FUNCTION__ << ") ----" << std::endl; \
        std::cout << "\t left: " << (x) << std::endl; \
        std::cout << "\tright: " << (y) << std::endl; \
        return(-1); \
    } \
} while(0)

int main() {
    const std::uint8_t src[] = {
        0x04, 0x40, 0x40, 0x09, 0xd3, 0x1a, 0xfc, 0x28,
        0x45, 0x2a, 0x8b, 0x4d, 0x9e, 0x64, 0x71, 0x04
    };

    std::uint8_t d_0[128];
    std::memset(d_0, 255, sizeof(d_0));

    std::uintptr_t l_0 = TrueMan::util::code::base16::encode(src, sizeof(src), d_0, 1);

    TST_assert_eq(l_0, 32);

    TST_assert_eq(d_0[ 0], '0');
    TST_assert_eq(d_0[ 1], '4');
    TST_assert_eq(d_0[ 2], '4');
    TST_assert_eq(d_0[ 3], '0');
    TST_assert_eq(d_0[ 4], '4');
    TST_assert_eq(d_0[ 5], '0');
    TST_assert_eq(d_0[ 6], '0');
    TST_assert_eq(d_0[ 7], '9');
    TST_assert_eq(d_0[ 8], 'D');
    TST_assert_eq(d_0[ 9], '3');
    TST_assert_eq(d_0[10], '1');
    TST_assert_eq(d_0[11], 'A');
    TST_assert_eq(d_0[12], 'F');
    TST_assert_eq(d_0[13], 'C');
    TST_assert_eq(d_0[14], '2');
    TST_assert_eq(d_0[15], '8');
    TST_assert_eq(d_0[16], '4');
    TST_assert_eq(d_0[17], '5');
    TST_assert_eq(d_0[18], '2');
    TST_assert_eq(d_0[19], 'A');
    TST_assert_eq(d_0[20], '8');
    TST_assert_eq(d_0[21], 'B');
    TST_assert_eq(d_0[22], '4');
    TST_assert_eq(d_0[23], 'D');
    TST_assert_eq(d_0[24], '9');
    TST_assert_eq(d_0[25], 'E');
    TST_assert_eq(d_0[26], '6');
    TST_assert_eq(d_0[27], '4');
    TST_assert_eq(d_0[28], '7');
    TST_assert_eq(d_0[29], '1');
    TST_assert_eq(d_0[30], '0');
    TST_assert_eq(d_0[31], '4');

    std::uint8_t d_1[128];
    std::memset(d_1, 255, sizeof(d_1));

    std::uintptr_t l_1 = TrueMan::util::code::base16::decode(d_0, l_0, d_1);
    if(l_1 < 0) {
        TST_assert_eq(0, 1);
    }

    TST_assert_eq(l_1, 16);

    TST_assert_eq(d_1[ 0], 0x04);
    TST_assert_eq(d_1[ 1], 0x40);
    TST_assert_eq(d_1[ 2], 0x40);
    TST_assert_eq(d_1[ 3], 0x09);
    TST_assert_eq(d_1[ 4], 0xD3);
    TST_assert_eq(d_1[ 5], 0x1A);
    TST_assert_eq(d_1[ 6], 0xFC);
    TST_assert_eq(d_1[ 7], 0x28);
    TST_assert_eq(d_1[ 8], 0x45);
    TST_assert_eq(d_1[ 9], 0x2A);
    TST_assert_eq(d_1[10], 0x8B);
    TST_assert_eq(d_1[11], 0x4D);
    TST_assert_eq(d_1[12], 0x9E);
    TST_assert_eq(d_1[13], 0x64);
    TST_assert_eq(d_1[14], 0x71);
    TST_assert_eq(d_1[15], 0x04);

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
