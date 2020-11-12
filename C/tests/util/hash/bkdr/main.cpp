#include <trueman/util/hash.h>
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
    const char str[] = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^";

    std::uint32_t h = TrueMan::util::hash::bkdr::x_0(
        str, sizeof(str) - 1, 131, 0
    );
    TST_assert_eq(h, 3255416723);

    h = TrueMan::util::hash::bkdr::time33(str, sizeof(str) - 1, 0);
    TST_assert_eq(h, 2622491663);

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
