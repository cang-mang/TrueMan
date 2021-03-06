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

    std::uint32_t x = TrueMan::util::hash::fnv1::x32(str, sizeof(str) - 1);
    TST_assert_eq(x, 4240981924ULL);

    std::uint64_t y = TrueMan::util::hash::fnv1::x64(str, sizeof(str) - 1);
    TST_assert_eq(y, 7619294831832707076ULL);

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
