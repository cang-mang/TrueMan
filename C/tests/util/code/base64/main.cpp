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
        'Q', 'u', 'o', 't', 'e', 'd', '-', 'p',
        'r', 'i', 'n', 't', 'a', 'b', 'l', 'e'
    };

    std::uint8_t d_0[128];
    std::memset(d_0, 255, sizeof(d_0));

    std::uintptr_t l_0 = TrueMan::util::code::base64::encode(src, sizeof(src), d_0);

    TST_assert_eq(l_0, 24);

    TST_assert_eq(d_0[ 0], 'U');
    TST_assert_eq(d_0[ 1], 'X');
    TST_assert_eq(d_0[ 2], 'V');
    TST_assert_eq(d_0[ 3], 'v');
    TST_assert_eq(d_0[ 4], 'd');
    TST_assert_eq(d_0[ 5], 'G');
    TST_assert_eq(d_0[ 6], 'V');
    TST_assert_eq(d_0[ 7], 'k');
    TST_assert_eq(d_0[ 8], 'L');
    TST_assert_eq(d_0[ 9], 'X');
    TST_assert_eq(d_0[10], 'B');
    TST_assert_eq(d_0[11], 'y');
    TST_assert_eq(d_0[12], 'a');
    TST_assert_eq(d_0[13], 'W');
    TST_assert_eq(d_0[14], '5');
    TST_assert_eq(d_0[15], '0');
    TST_assert_eq(d_0[16], 'Y');
    TST_assert_eq(d_0[17], 'W');
    TST_assert_eq(d_0[18], 'J');
    TST_assert_eq(d_0[19], 's');
    TST_assert_eq(d_0[20], 'Z');
    TST_assert_eq(d_0[21], 'Q');
    TST_assert_eq(d_0[22], '=');
    TST_assert_eq(d_0[23], '=');

    std::uint8_t d_1[128];
    std::memset(d_1, 255, sizeof(d_1));

    std::intptr_t l_1 = TrueMan::util::code::base64::decode(d_0, l_0, d_1);
    if(l_1 < 0) {
        TST_assert_eq(0, 1);
    }

    TST_assert_eq(l_1, 16);

    TST_assert_eq(d_1[ 0], 'Q');
    TST_assert_eq(d_1[ 1], 'u');
    TST_assert_eq(d_1[ 2], 'o');
    TST_assert_eq(d_1[ 3], 't');
    TST_assert_eq(d_1[ 4], 'e');
    TST_assert_eq(d_1[ 5], 'd');
    TST_assert_eq(d_1[ 6], '-');
    TST_assert_eq(d_1[ 7], 'p');
    TST_assert_eq(d_1[ 8], 'r');
    TST_assert_eq(d_1[ 9], 'i');
    TST_assert_eq(d_1[10], 'n');
    TST_assert_eq(d_1[11], 't');
    TST_assert_eq(d_1[12], 'a');
    TST_assert_eq(d_1[13], 'b');
    TST_assert_eq(d_1[14], 'l');
    TST_assert_eq(d_1[15], 'e');

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
