#include <trueman/crypto/hash.h>
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

static const char TST_txt_0[] = "";

static const char TST_txt_1[] = "The quick brown fox jumps over the lazy dog";

static const char TST_txt_2[] = "The quick brown fox jumps over the lazy dog.";

static const char TST_txt_3[] = "MIIEQwYJKoZIhvcNAQcCoIIENDCCBDACAQExCzAJBgUrDgMC"
                                "GgUAMAsGCSqGSIb3DQEHAaCCAskwggLFMIIBraADAgECAgQ5"
                                "oQecMA0GCSqGSIb3DQEBCwUAMBMxETAPBgNVBAMTCG1wYWFz"
                                "MTIzMB4XDTE2MTEwMzEzMjMzNFoXDTQxMTAyODEzMjMzNFow"
                                "EzERMA8GA1UEAxMIbXBhYXMxMjMwggEiMA0GCSqGSIb3DQEB"
                                "AQUAA4IBDwAwggEKAoIBAQDNcuVBVv4Z+OGCg3nfrkLYf1Q7"
                                "x+6gmZupGAh8DLews6NyfkXantLvYGz2KULNZimV8l6gzmHD"
                                "HfmtJyOkgqXw+nl8dv4XQaZTRegtF3rnZt43Sc8ATBPGDjcx"
                                "RkS4OU+NrkYvjCgy2Kqx77vQL44i552upjn2zeVuKPHPMa2e"
                                "4brA3V5JkJFGI+UO6Kbi8UwpK6cZLOg2L6rjOMoo48ie2LcG"
                                "SQ8R2WxSlxX2ShMbqIjiZUzuVIprvC1DdFHDJjnmwY9pO1gl"
                                "SV40CWSHYIYc18IayA0Tp3ntGXsbvmK3DZaSDoAXSSWpd0up"
                                "lw89eDlxDJooMrdya4E38c1LHFDxAgMBAAGjITAfMB0GA1Ud"
                                "DgQWBBTv20iuBW5mZNCnVmLqaMDn1B9hazANBgkqhkiG9w0B"
                                "AQsFAAOCAQEANS+tmgXPfqTkFnAlHMeypKhAa0sSTgpbe9gl"
                                "9PzRBrqLxFxXeeJTF/FG+wDgLZYrJRl79jvnYY5glb0se7W6"
                                "N0RYH1hgKlbZiWysVuoCxgzF9h6mpul/FFCRQyvg58CGrY01"
                                "KaEmUWGAMZO8tH7rDsBvrRSpRx9xIiZE1EgnePTZeIABbY8R"
                                "6SjM7Ar2mAgZ+Wtxtu0zCKsLkRGWrcFhzvYIedXltfK5s1ro"
                                "DeGWXgDRPVUL1bVc2af8ABnBjHWJsVPYS2etHJuRSPZQLQX3"
                                "cgt7W2NlnWShy1NvGSZeHlrpaoSj0qc9DZ+o5h2jimmYuU8I"
                                "6mHmoLZNjFytnl/JNTGCAUIwggE+AgEBMBswEzERMA8GA1UE"
                                "AxMIbXBhYXMxMjMCBDmhB5wwCQYFKw4DAhoFADANBgkqhkiG"
                                "9w0BAQEFAASCAQB/IFTYMVWU5HapWkhoPcQaY+NEkN0JhKqN"
                                "Jj0WWqgR8/56W3X/RRIc/PKnn9+WL7G7vJuXr3eLd8U/0NED"
                                "W37dOebchoSkSAfDLrilCMJUbSlAGNHjffwwkVX3K0JKiV+j"
                                "W795l7bRSaP++esblJZFxsbhwFhTST1XCFfm5jLH/PAvJakL"
                                "tZV/uMWtguc/EHm3rhs3MaxSeCTn4qhxRIVunDRYi5xFg1UK"
                                "WsaOnIJyMngnXZVQ509Qbbd4rcR9IZ6a2cyosbj9ehAlwD/Y"
                                "M+WI64DJE+vBKhR7kjnZZQFkX/H+0hOuKountfiM3E+B87rR"
                                "e2Ff4TSdA7262qZhvsYB";

static const char TST_txt_4[] = "In terms of practical security, a major concern "
                                "about these new attacks is that they might pave the "
                                "way to more efficient ones. Whether this is the case "
                                "is yet to be seen, but a migration to stronger "
                                "hashes is believed to be prudent. Some of the "
                                "applications that use cryptographic hashes, like "
                                "password storage, are only minimally affected by a "
                                "collision attack. Constructing a password that works "
                                "for a given account requires a preimage attack, as "
                                "well as access to the hash of the original password, "
                                "which may or may not be trivial. Reversing password "
                                "encryption (e.g. to obtain a password to try against "
                                "a user's account elsewhere) is not made possible by "
                                "the attacks. (However, even a secure password hash "
                                "can\'t prevent brute-force attacks on weak "
                                "passwords.)";

static const char TST_txt_5[] = "0";

typedef union TST_ctx {
    TrueMan_U8_t  b[2];
    TrueMan_U16_t s;
} TST_CTX_t;

static void TST_csum(const void *ptr, TrueMan_USIZE_t len, TrueMan_U8_t val[2]) {
    const TrueMan_U8_t *buf = (const TrueMan_U8_t *)ptr;

    TST_CTX_t     ctx;
    TrueMan_U32_t sum = 0;

    while(len > 1) {
        ctx.b[0] = *(buf++);
        ctx.b[1] = *(buf++);
        sum += ctx.s;
        len -= 2;
    }

    if(len) {
        sum += *buf;
    }

    while(sum >> 16) {
        sum = (sum >> 16) + (sum & 0x0000FFFF);
    }

    ctx.s = (TrueMan_U16_t)(~sum);
    val[0] = ctx.b[0];
    val[1] = ctx.b[1];
}

static void TST_normal(
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t v_1[2], TrueMan_U8_t v_2[2]
) {
    TrueMan_CRYPTO_HASH_CSUM_CTX_t ctx;
    TrueMan_CRYPTO_HASH_CSUM_final(
        TrueMan_CRYPTO_HASH_CSUM_update(
            TrueMan_CRYPTO_HASH_CSUM_final(
                TrueMan_CRYPTO_HASH_CSUM_update(
                    TrueMan_CRYPTO_HASH_CSUM_init(&ctx), txt, len
                ),
                v_1
            ),
            txt,
            len
        ),
        v_2
    );
}

static void TST_byte_by_byte(
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t v_1[2], TrueMan_U8_t v_2[2]
) {
    TrueMan_CRYPTO_HASH_CSUM_CTX_t ctx;
    TrueMan_CRYPTO_HASH_CSUM_init(&ctx);

    TrueMan_USIZE_t cur = 0;
    for( ; cur < len; ++cur) {
        TrueMan_CRYPTO_HASH_CSUM_update(&ctx, txt + cur, 1);
    }

    TrueMan_CRYPTO_HASH_CSUM_final(&ctx, v_1);

    for(cur = 0; cur < len; ++cur) {
        TrueMan_CRYPTO_HASH_CSUM_update(&ctx, txt + cur, 1);
    }

    TrueMan_CRYPTO_HASH_CSUM_final(&ctx, v_2);
}

static void TST_one(
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t val[2]
) {
    TrueMan_CRYPTO_HASH_CSUM_one(txt, len, val);
}

static void TST_old(
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t val[2]
) {
    TST_csum(txt, len, val);
}

static int TST_exe(const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t const sum[2]) {
    TrueMan_U8_t v_1[2] = { 0, 0 };
    TrueMan_U8_t v_2[2] = { 0, 0 };

    TST_normal(txt, len, v_1, v_2);
    TST_assert_eq(v_1[0], sum[0]);
    TST_assert_eq(v_1[1], sum[1]);
    TST_assert_eq(v_2[0], sum[0]);
    TST_assert_eq(v_2[1], sum[1]);

    v_1[0] = 0;
    v_1[1] = 0;
    v_2[0] = 0;
    v_2[1] = 0;

    TST_byte_by_byte(txt, len, v_1, v_2);
    TST_assert_eq(v_1[0], sum[0]);
    TST_assert_eq(v_1[1], sum[1]);
    TST_assert_eq(v_2[0], sum[0]);
    TST_assert_eq(v_2[1], sum[1]);

    v_1[0] = 0;
    v_1[1] = 0;
    v_2[0] = 0;
    v_2[1] = 0;

    TST_one(txt, len, v_1);
    TST_old(txt, len, v_2);
    TST_assert_eq(v_1[0], sum[0]);
    TST_assert_eq(v_1[1], sum[1]);
    TST_assert_eq(v_2[0], sum[0]);
    TST_assert_eq(v_2[1], sum[1]);

    return(0);
}

int main(void) {
    printf("len_ctx=%zu\n", TrueMan_CRYPTO_HASH_CSUM_op.len_ctx);
    printf("len_blk=%zu\n", TrueMan_CRYPTO_HASH_CSUM_op.len_blk);
    printf("len_dgt=%zu\n", TrueMan_CRYPTO_HASH_CSUM_op.len_dgt);

    TrueMan_U8_t val[2] = { 0xFF, 0xFF };
    if(TST_exe(TST_txt_0, sizeof(TST_txt_0) - 1, val)) {
        printf("\"%s\"\n", TST_txt_0);
        return(-1);
    }

    val[0] = 0x72;
    val[1] = 0xA4;
    if(TST_exe(TST_txt_1, sizeof(TST_txt_1) - 1, val)) {
        printf("\"%s\"\n", TST_txt_1);
        return(-1);
    }

    val[0] = 0x72;
    val[1] = 0x76;
    if(TST_exe(TST_txt_2, sizeof(TST_txt_2) - 1, val)) {
        printf("\"%s\"\n", TST_txt_2);
        return(-1);
    }

    val[0] = 0x77;
    val[1] = 0x70;
    if(TST_exe(TST_txt_3, sizeof(TST_txt_3) - 1, val)) {
        printf("\"%s\"\n", TST_txt_3);
        return(-1);
    }

    val[0] = 0x05;
    val[1] = 0x03;
    if(TST_exe(TST_txt_4, sizeof(TST_txt_4) - 1, val)) {
        printf("\"%s\"\n", TST_txt_4);
        return(-1);
    }

    val[0] = 0xCF;
    val[1] = 0xFF;
    if(TST_exe(TST_txt_5, sizeof(TST_txt_5) - 1, val)) {
        printf("\"%s\"\n", TST_txt_5);
        return(-1);
    }

    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
