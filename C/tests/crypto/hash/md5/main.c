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

static void TST_normal(
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t v_1[16], TrueMan_U8_t v_2[16]
) {
    TrueMan_CRYPTO_HASH_MD5_CTX_t ctx;
    TrueMan_CRYPTO_HASH_MD5_final(
        TrueMan_CRYPTO_HASH_MD5_update(
            TrueMan_CRYPTO_HASH_MD5_final(
                TrueMan_CRYPTO_HASH_MD5_update(
                    TrueMan_CRYPTO_HASH_MD5_init(&ctx), txt, len
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
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t v_1[16], TrueMan_U8_t v_2[16]
) {
    TrueMan_CRYPTO_HASH_MD5_CTX_t ctx;
    TrueMan_CRYPTO_HASH_MD5_init(&ctx);

    TrueMan_USIZE_t cur = 0;
    for( ; cur < len; ++cur) {
        TrueMan_CRYPTO_HASH_MD5_update(&ctx, txt + cur, 1);
    }

    TrueMan_CRYPTO_HASH_MD5_final(&ctx, v_1);

    for(cur = 0; cur < len; ++cur) {
        TrueMan_CRYPTO_HASH_MD5_update(&ctx, txt + cur, 1);
    }

    TrueMan_CRYPTO_HASH_MD5_final(&ctx, v_2);
}

static void TST_one(
    const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t val[16]
) {
    TrueMan_CRYPTO_HASH_MD5_one(txt, len, val);
}

static int TST_exe(const char *txt, TrueMan_USIZE_t len, TrueMan_U8_t const sum[16]) {
    TrueMan_U8_t v_1[16] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
    TrueMan_U8_t v_2[16] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };

    TST_normal(txt, len, v_1, v_2);
    TST_assert_eq(v_1[ 0], sum[ 0]);
    TST_assert_eq(v_1[ 1], sum[ 1]);
    TST_assert_eq(v_1[ 2], sum[ 2]);
    TST_assert_eq(v_1[ 3], sum[ 3]);
    TST_assert_eq(v_1[ 4], sum[ 4]);
    TST_assert_eq(v_1[ 5], sum[ 5]);
    TST_assert_eq(v_1[ 6], sum[ 6]);
    TST_assert_eq(v_1[ 7], sum[ 7]);
    TST_assert_eq(v_1[ 8], sum[ 8]);
    TST_assert_eq(v_1[ 9], sum[ 9]);
    TST_assert_eq(v_1[10], sum[10]);
    TST_assert_eq(v_1[11], sum[11]);
    TST_assert_eq(v_1[12], sum[12]);
    TST_assert_eq(v_1[13], sum[13]);
    TST_assert_eq(v_1[14], sum[14]);
    TST_assert_eq(v_1[15], sum[15]);
    TST_assert_eq(v_2[ 0], sum[ 0]);
    TST_assert_eq(v_2[ 1], sum[ 1]);
    TST_assert_eq(v_2[ 2], sum[ 2]);
    TST_assert_eq(v_2[ 3], sum[ 3]);
    TST_assert_eq(v_2[ 4], sum[ 4]);
    TST_assert_eq(v_2[ 5], sum[ 5]);
    TST_assert_eq(v_2[ 6], sum[ 6]);
    TST_assert_eq(v_2[ 7], sum[ 7]);
    TST_assert_eq(v_2[ 8], sum[ 8]);
    TST_assert_eq(v_2[ 9], sum[ 9]);
    TST_assert_eq(v_2[10], sum[10]);
    TST_assert_eq(v_2[11], sum[11]);
    TST_assert_eq(v_2[12], sum[12]);
    TST_assert_eq(v_2[13], sum[13]);
    TST_assert_eq(v_2[14], sum[14]);
    TST_assert_eq(v_2[15], sum[15]);

    v_1[ 0] = 0;
    v_1[ 1] = 0;
    v_1[ 2] = 0;
    v_1[ 3] = 0;
    v_1[ 4] = 0;
    v_1[ 5] = 0;
    v_1[ 6] = 0;
    v_1[ 7] = 0;
    v_1[ 8] = 0;
    v_1[ 9] = 0;
    v_1[10] = 0;
    v_1[11] = 0;
    v_1[12] = 0;
    v_1[13] = 0;
    v_1[14] = 0;
    v_1[15] = 0;
    v_2[ 0] = 0;
    v_2[ 1] = 0;
    v_2[ 2] = 0;
    v_2[ 3] = 0;
    v_2[ 4] = 0;
    v_2[ 5] = 0;
    v_2[ 6] = 0;
    v_2[ 7] = 0;
    v_2[ 8] = 0;
    v_2[ 9] = 0;
    v_2[10] = 0;
    v_2[11] = 0;
    v_2[12] = 0;
    v_2[13] = 0;
    v_2[14] = 0;
    v_2[15] = 0;

    TST_byte_by_byte(txt, len, v_1, v_2);
    TST_assert_eq(v_1[ 0], sum[ 0]);
    TST_assert_eq(v_1[ 1], sum[ 1]);
    TST_assert_eq(v_1[ 2], sum[ 2]);
    TST_assert_eq(v_1[ 3], sum[ 3]);
    TST_assert_eq(v_1[ 4], sum[ 4]);
    TST_assert_eq(v_1[ 5], sum[ 5]);
    TST_assert_eq(v_1[ 6], sum[ 6]);
    TST_assert_eq(v_1[ 7], sum[ 7]);
    TST_assert_eq(v_1[ 8], sum[ 8]);
    TST_assert_eq(v_1[ 9], sum[ 9]);
    TST_assert_eq(v_1[10], sum[10]);
    TST_assert_eq(v_1[11], sum[11]);
    TST_assert_eq(v_1[12], sum[12]);
    TST_assert_eq(v_1[13], sum[13]);
    TST_assert_eq(v_1[14], sum[14]);
    TST_assert_eq(v_1[15], sum[15]);
    TST_assert_eq(v_2[ 0], sum[ 0]);
    TST_assert_eq(v_2[ 1], sum[ 1]);
    TST_assert_eq(v_2[ 2], sum[ 2]);
    TST_assert_eq(v_2[ 3], sum[ 3]);
    TST_assert_eq(v_2[ 4], sum[ 4]);
    TST_assert_eq(v_2[ 5], sum[ 5]);
    TST_assert_eq(v_2[ 6], sum[ 6]);
    TST_assert_eq(v_2[ 7], sum[ 7]);
    TST_assert_eq(v_2[ 8], sum[ 8]);
    TST_assert_eq(v_2[ 9], sum[ 9]);
    TST_assert_eq(v_2[10], sum[10]);
    TST_assert_eq(v_2[11], sum[11]);
    TST_assert_eq(v_2[12], sum[12]);
    TST_assert_eq(v_2[13], sum[13]);
    TST_assert_eq(v_2[14], sum[14]);
    TST_assert_eq(v_2[15], sum[15]);

    v_1[ 0] = 0;
    v_1[ 1] = 0;
    v_1[ 2] = 0;
    v_1[ 3] = 0;
    v_1[ 4] = 0;
    v_1[ 5] = 0;
    v_1[ 6] = 0;
    v_1[ 7] = 0;
    v_1[ 8] = 0;
    v_1[ 9] = 0;
    v_1[10] = 0;
    v_1[11] = 0;
    v_1[12] = 0;
    v_1[13] = 0;
    v_1[14] = 0;
    v_1[15] = 0;

    TST_one(txt, len, v_1);
    TST_assert_eq(v_1[ 0], sum[ 0]);
    TST_assert_eq(v_1[ 1], sum[ 1]);
    TST_assert_eq(v_1[ 2], sum[ 2]);
    TST_assert_eq(v_1[ 3], sum[ 3]);
    TST_assert_eq(v_1[ 4], sum[ 4]);
    TST_assert_eq(v_1[ 5], sum[ 5]);
    TST_assert_eq(v_1[ 6], sum[ 6]);
    TST_assert_eq(v_1[ 7], sum[ 7]);
    TST_assert_eq(v_1[ 8], sum[ 8]);
    TST_assert_eq(v_1[ 9], sum[ 9]);
    TST_assert_eq(v_1[10], sum[10]);
    TST_assert_eq(v_1[11], sum[11]);
    TST_assert_eq(v_1[12], sum[12]);
    TST_assert_eq(v_1[13], sum[13]);
    TST_assert_eq(v_1[14], sum[14]);
    TST_assert_eq(v_1[15], sum[15]);

    return(0);
}

int main(void) {
    printf("len_ctx=%zu\n", TrueMan_CRYPTO_HASH_MD5_op.len_ctx);
    printf("len_blk=%zu\n", TrueMan_CRYPTO_HASH_MD5_op.len_blk);
    printf("len_dgt=%zu\n", TrueMan_CRYPTO_HASH_MD5_op.len_dgt);
    do {
        TrueMan_U8_t val[16] = {
            0xD4, 0x1D, 0x8C, 0xD9, 0x8F, 0x00, 0xB2, 0x04,
            0xE9, 0x80, 0x09, 0x98, 0xEC, 0xF8, 0x42, 0x7E
        };
        if(TST_exe(TST_txt_0, sizeof(TST_txt_0) - 1, val)) {
            printf("\"%s\"\n", TST_txt_0);
            return(-1);
        }
    } while(0);
    do {
        TrueMan_U8_t val[16] = {
            0x9E, 0x10, 0x7D, 0x9D, 0x37, 0x2B, 0xB6, 0x82,
            0x6B, 0xD8, 0x1D, 0x35, 0x42, 0xA4, 0x19, 0xD6
        };
        if(TST_exe(TST_txt_1, sizeof(TST_txt_1) - 1, val)) {
            printf("\"%s\"\n", TST_txt_1);
            return(-1);
        }
    } while(0);
    do {
        TrueMan_U8_t val[16] = {
            0xE4, 0xD9, 0x09, 0xC2, 0x90, 0xD0, 0xFB, 0x1C,
            0xA0, 0x68, 0xFF, 0xAD, 0xDF, 0x22, 0xCB, 0xD0
        };
        if(TST_exe(TST_txt_2, sizeof(TST_txt_2) - 1, val)) {
            printf("\"%s\"\n", TST_txt_2);
            return(-1);
        }
    } while(0);
    do {
        TrueMan_U8_t val[16] = {
            0x42, 0x24, 0xB8, 0x85, 0x74, 0x32, 0x24, 0x14,
            0x29, 0x6C, 0xFF, 0xC6, 0x14, 0x00, 0x1E, 0x71
        };
        if(TST_exe(TST_txt_3, sizeof(TST_txt_3) - 1, val)) {
            printf("\"%s\"\n", TST_txt_3);
            return(-1);
        }
    } while(0);
    printf("OK (%s)\n", __FUNCTION__);
    return(0);
}
