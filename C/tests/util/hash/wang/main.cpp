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
    std::uint64_t x = (std::uint64_t)9876543210123456789ULL;
    std::uint64_t y = TrueMan::util::hash::wang::direct(x);
    TST_assert_eq(y, 14255830987705448333ULL);

    x = TrueMan::util::hash::wang::inverse(y);
    TST_assert_eq(x, 9876543210123456789ULL);

    std::cout << "OK " << '(' << __FUNCTION__ << ')' << std::endl;
    return(0);
}
