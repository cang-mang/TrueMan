use super::super::Ctx as _Ctx;
/*============================================================================*/
const TXT_0: &str = "";
/*----------------------------------------------------------------------------*/
const TXT_1: &str = "The quick brown fox jumps over the lazy dog";
/*----------------------------------------------------------------------------*/
const TXT_2: &str = "The quick brown fox jumps over the lazy dog.";
/*----------------------------------------------------------------------------*/
const TXT_3: &str = "MIIEQwYJKoZIhvcNAQcCoIIENDCCBDACAQExCzAJBgUrDgMCGgUAMAsGCSqGSIb3DQEHAaCCAskwggLFMIIBraADAgECAgQ5oQecMA0GCSqGSIb3DQEBCwUAMBMxETAPBgNVBAMTCG1wYWFzMTIzMB4XDTE2MTEwMzEzMjMzNFoXDTQxMTAyODEzMjMzNFowEzERMA8GA1UEAxMIbXBhYXMxMjMwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDNcuVBVv4Z+OGCg3nfrkLYf1Q7x+6gmZupGAh8DLews6NyfkXantLvYGz2KULNZimV8l6gzmHDHfmtJyOkgqXw+nl8dv4XQaZTRegtF3rnZt43Sc8ATBPGDjcxRkS4OU+NrkYvjCgy2Kqx77vQL44i552upjn2zeVuKPHPMa2e4brA3V5JkJFGI+UO6Kbi8UwpK6cZLOg2L6rjOMoo48ie2LcGSQ8R2WxSlxX2ShMbqIjiZUzuVIprvC1DdFHDJjnmwY9pO1glSV40CWSHYIYc18IayA0Tp3ntGXsbvmK3DZaSDoAXSSWpd0uplw89eDlxDJooMrdya4E38c1LHFDxAgMBAAGjITAfMB0GA1UdDgQWBBTv20iuBW5mZNCnVmLqaMDn1B9hazANBgkqhkiG9w0BAQsFAAOCAQEANS+tmgXPfqTkFnAlHMeypKhAa0sSTgpbe9gl9PzRBrqLxFxXeeJTF/FG+wDgLZYrJRl79jvnYY5glb0se7W6N0RYH1hgKlbZiWysVuoCxgzF9h6mpul/FFCRQyvg58CGrY01KaEmUWGAMZO8tH7rDsBvrRSpRx9xIiZE1EgnePTZeIABbY8R6SjM7Ar2mAgZ+Wtxtu0zCKsLkRGWrcFhzvYIedXltfK5s1roDeGWXgDRPVUL1bVc2af8ABnBjHWJsVPYS2etHJuRSPZQLQX3cgt7W2NlnWShy1NvGSZeHlrpaoSj0qc9DZ+o5h2jimmYuU8I6mHmoLZNjFytnl/JNTGCAUIwggE+AgEBMBswEzERMA8GA1UEAxMIbXBhYXMxMjMCBDmhB5wwCQYFKw4DAhoFADANBgkqhkiG9w0BAQEFAASCAQB/IFTYMVWU5HapWkhoPcQaY+NEkN0JhKqNJj0WWqgR8/56W3X/RRIc/PKnn9+WL7G7vJuXr3eLd8U/0NEDW37dOebchoSkSAfDLrilCMJUbSlAGNHjffwwkVX3K0JKiV+jW795l7bRSaP++esblJZFxsbhwFhTST1XCFfm5jLH/PAvJakLtZV/uMWtguc/EHm3rhs3MaxSeCTn4qhxRIVunDRYi5xFg1UKWsaOnIJyMngnXZVQ509Qbbd4rcR9IZ6a2cyosbj9ehAlwD/YM+WI64DJE+vBKhR7kjnZZQFkX/H+0hOuKountfiM3E+B87rRe2Ff4TSdA7262qZhvsYB";
/*----------------------------------------------------------------------------*/
fn tst_normal(key: &[u8], v_1: &mut [u8], v_2: &mut [u8]) {
    let mut ctx: super::Ctx = Default::default();
    ctx.update(key).r#final(v_1).update(key).r#final(v_2);
}
/*----------------------------------------------------------------------------*/
fn tst_byte_by_byte(key: &[u8], v_1: &mut [u8], v_2: &mut [u8]) {
    let mut ctx: super::Ctx = Default::default();

    let mut cur = 0_usize;
    while cur < key.len() {
        let k = &key[cur..=cur];
        ctx.update(k);
        cur += 1;
    }

    ctx.r#final(v_1);

    cur = 0_usize;
    while cur < key.len() {
        let k = &key[cur..=cur];
        ctx.update(k);
        cur += 1;
    }

    ctx.r#final(v_2);
}
/*----------------------------------------------------------------------------*/
fn tst_one(key: &[u8], val: &mut [u8]) {
    super::Ctx::one(key, val);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_0() {
    let key = TXT_0.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0xD4);
    assert_eq!(v_1[ 1], 0x1D);
    assert_eq!(v_1[ 2], 0x8C);
    assert_eq!(v_1[ 3], 0xD9);
    assert_eq!(v_1[ 4], 0x8F);
    assert_eq!(v_1[ 5], 0x00);
    assert_eq!(v_1[ 6], 0xB2);
    assert_eq!(v_1[ 7], 0x04);
    assert_eq!(v_1[ 8], 0xE9);
    assert_eq!(v_1[ 9], 0x80);
    assert_eq!(v_1[10], 0x09);
    assert_eq!(v_1[11], 0x98);
    assert_eq!(v_1[12], 0xEC);
    assert_eq!(v_1[13], 0xF8);
    assert_eq!(v_1[14], 0x42);
    assert_eq!(v_1[15], 0x7E);
    assert_eq!(v_2[ 0], 0xD4);
    assert_eq!(v_2[ 1], 0x1D);
    assert_eq!(v_2[ 2], 0x8C);
    assert_eq!(v_2[ 3], 0xD9);
    assert_eq!(v_2[ 4], 0x8F);
    assert_eq!(v_2[ 5], 0x00);
    assert_eq!(v_2[ 6], 0xB2);
    assert_eq!(v_2[ 7], 0x04);
    assert_eq!(v_2[ 8], 0xE9);
    assert_eq!(v_2[ 9], 0x80);
    assert_eq!(v_2[10], 0x09);
    assert_eq!(v_2[11], 0x98);
    assert_eq!(v_2[12], 0xEC);
    assert_eq!(v_2[13], 0xF8);
    assert_eq!(v_2[14], 0x42);
    assert_eq!(v_2[15], 0x7E);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0xD4);
    assert_eq!(v_1[ 1], 0x1D);
    assert_eq!(v_1[ 2], 0x8C);
    assert_eq!(v_1[ 3], 0xD9);
    assert_eq!(v_1[ 4], 0x8F);
    assert_eq!(v_1[ 5], 0x00);
    assert_eq!(v_1[ 6], 0xB2);
    assert_eq!(v_1[ 7], 0x04);
    assert_eq!(v_1[ 8], 0xE9);
    assert_eq!(v_1[ 9], 0x80);
    assert_eq!(v_1[10], 0x09);
    assert_eq!(v_1[11], 0x98);
    assert_eq!(v_1[12], 0xEC);
    assert_eq!(v_1[13], 0xF8);
    assert_eq!(v_1[14], 0x42);
    assert_eq!(v_1[15], 0x7E);
    assert_eq!(v_2[ 0], 0xD4);
    assert_eq!(v_2[ 1], 0x1D);
    assert_eq!(v_2[ 2], 0x8C);
    assert_eq!(v_2[ 3], 0xD9);
    assert_eq!(v_2[ 4], 0x8F);
    assert_eq!(v_2[ 5], 0x00);
    assert_eq!(v_2[ 6], 0xB2);
    assert_eq!(v_2[ 7], 0x04);
    assert_eq!(v_2[ 8], 0xE9);
    assert_eq!(v_2[ 9], 0x80);
    assert_eq!(v_2[10], 0x09);
    assert_eq!(v_2[11], 0x98);
    assert_eq!(v_2[12], 0xEC);
    assert_eq!(v_2[13], 0xF8);
    assert_eq!(v_2[14], 0x42);
    assert_eq!(v_2[15], 0x7E);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[ 0], 0xD4);
    assert_eq!(val[ 1], 0x1D);
    assert_eq!(val[ 2], 0x8C);
    assert_eq!(val[ 3], 0xD9);
    assert_eq!(val[ 4], 0x8F);
    assert_eq!(val[ 5], 0x00);
    assert_eq!(val[ 6], 0xB2);
    assert_eq!(val[ 7], 0x04);
    assert_eq!(val[ 8], 0xE9);
    assert_eq!(val[ 9], 0x80);
    assert_eq!(val[10], 0x09);
    assert_eq!(val[11], 0x98);
    assert_eq!(val[12], 0xEC);
    assert_eq!(val[13], 0xF8);
    assert_eq!(val[14], 0x42);
    assert_eq!(val[15], 0x7E);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_1() {
    let key = TXT_1.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0x9E);
    assert_eq!(v_1[ 1], 0x10);
    assert_eq!(v_1[ 2], 0x7D);
    assert_eq!(v_1[ 3], 0x9D);
    assert_eq!(v_1[ 4], 0x37);
    assert_eq!(v_1[ 5], 0x2B);
    assert_eq!(v_1[ 6], 0xB6);
    assert_eq!(v_1[ 7], 0x82);
    assert_eq!(v_1[ 8], 0x6B);
    assert_eq!(v_1[ 9], 0xD8);
    assert_eq!(v_1[10], 0x1D);
    assert_eq!(v_1[11], 0x35);
    assert_eq!(v_1[12], 0x42);
    assert_eq!(v_1[13], 0xA4);
    assert_eq!(v_1[14], 0x19);
    assert_eq!(v_1[15], 0xD6);
    assert_eq!(v_2[ 0], 0x9E);
    assert_eq!(v_2[ 1], 0x10);
    assert_eq!(v_2[ 2], 0x7D);
    assert_eq!(v_2[ 3], 0x9D);
    assert_eq!(v_2[ 4], 0x37);
    assert_eq!(v_2[ 5], 0x2B);
    assert_eq!(v_2[ 6], 0xB6);
    assert_eq!(v_2[ 7], 0x82);
    assert_eq!(v_2[ 8], 0x6B);
    assert_eq!(v_2[ 9], 0xD8);
    assert_eq!(v_2[10], 0x1D);
    assert_eq!(v_2[11], 0x35);
    assert_eq!(v_2[12], 0x42);
    assert_eq!(v_2[13], 0xA4);
    assert_eq!(v_2[14], 0x19);
    assert_eq!(v_2[15], 0xD6);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0x9E);
    assert_eq!(v_1[ 1], 0x10);
    assert_eq!(v_1[ 2], 0x7D);
    assert_eq!(v_1[ 3], 0x9D);
    assert_eq!(v_1[ 4], 0x37);
    assert_eq!(v_1[ 5], 0x2B);
    assert_eq!(v_1[ 6], 0xB6);
    assert_eq!(v_1[ 7], 0x82);
    assert_eq!(v_1[ 8], 0x6B);
    assert_eq!(v_1[ 9], 0xD8);
    assert_eq!(v_1[10], 0x1D);
    assert_eq!(v_1[11], 0x35);
    assert_eq!(v_1[12], 0x42);
    assert_eq!(v_1[13], 0xA4);
    assert_eq!(v_1[14], 0x19);
    assert_eq!(v_1[15], 0xD6);
    assert_eq!(v_2[ 0], 0x9E);
    assert_eq!(v_2[ 1], 0x10);
    assert_eq!(v_2[ 2], 0x7D);
    assert_eq!(v_2[ 3], 0x9D);
    assert_eq!(v_2[ 4], 0x37);
    assert_eq!(v_2[ 5], 0x2B);
    assert_eq!(v_2[ 6], 0xB6);
    assert_eq!(v_2[ 7], 0x82);
    assert_eq!(v_2[ 8], 0x6B);
    assert_eq!(v_2[ 9], 0xD8);
    assert_eq!(v_2[10], 0x1D);
    assert_eq!(v_2[11], 0x35);
    assert_eq!(v_2[12], 0x42);
    assert_eq!(v_2[13], 0xA4);
    assert_eq!(v_2[14], 0x19);
    assert_eq!(v_2[15], 0xD6);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[ 0], 0x9E);
    assert_eq!(val[ 1], 0x10);
    assert_eq!(val[ 2], 0x7D);
    assert_eq!(val[ 3], 0x9D);
    assert_eq!(val[ 4], 0x37);
    assert_eq!(val[ 5], 0x2B);
    assert_eq!(val[ 6], 0xB6);
    assert_eq!(val[ 7], 0x82);
    assert_eq!(val[ 8], 0x6B);
    assert_eq!(val[ 9], 0xD8);
    assert_eq!(val[10], 0x1D);
    assert_eq!(val[11], 0x35);
    assert_eq!(val[12], 0x42);
    assert_eq!(val[13], 0xA4);
    assert_eq!(val[14], 0x19);
    assert_eq!(val[15], 0xD6);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_2() {
    let key = TXT_2.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0xE4);
    assert_eq!(v_1[ 1], 0xD9);
    assert_eq!(v_1[ 2], 0x09);
    assert_eq!(v_1[ 3], 0xC2);
    assert_eq!(v_1[ 4], 0x90);
    assert_eq!(v_1[ 5], 0xD0);
    assert_eq!(v_1[ 6], 0xFB);
    assert_eq!(v_1[ 7], 0x1C);
    assert_eq!(v_1[ 8], 0xA0);
    assert_eq!(v_1[ 9], 0x68);
    assert_eq!(v_1[10], 0xFF);
    assert_eq!(v_1[11], 0xAD);
    assert_eq!(v_1[12], 0xDF);
    assert_eq!(v_1[13], 0x22);
    assert_eq!(v_1[14], 0xCB);
    assert_eq!(v_1[15], 0xD0);
    assert_eq!(v_2[ 0], 0xE4);
    assert_eq!(v_2[ 1], 0xD9);
    assert_eq!(v_2[ 2], 0x09);
    assert_eq!(v_2[ 3], 0xC2);
    assert_eq!(v_2[ 4], 0x90);
    assert_eq!(v_2[ 5], 0xD0);
    assert_eq!(v_2[ 6], 0xFB);
    assert_eq!(v_2[ 7], 0x1C);
    assert_eq!(v_2[ 8], 0xA0);
    assert_eq!(v_2[ 9], 0x68);
    assert_eq!(v_2[10], 0xFF);
    assert_eq!(v_2[11], 0xAD);
    assert_eq!(v_2[12], 0xDF);
    assert_eq!(v_2[13], 0x22);
    assert_eq!(v_2[14], 0xCB);
    assert_eq!(v_2[15], 0xD0);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0xE4);
    assert_eq!(v_1[ 1], 0xD9);
    assert_eq!(v_1[ 2], 0x09);
    assert_eq!(v_1[ 3], 0xC2);
    assert_eq!(v_1[ 4], 0x90);
    assert_eq!(v_1[ 5], 0xD0);
    assert_eq!(v_1[ 6], 0xFB);
    assert_eq!(v_1[ 7], 0x1C);
    assert_eq!(v_1[ 8], 0xA0);
    assert_eq!(v_1[ 9], 0x68);
    assert_eq!(v_1[10], 0xFF);
    assert_eq!(v_1[11], 0xAD);
    assert_eq!(v_1[12], 0xDF);
    assert_eq!(v_1[13], 0x22);
    assert_eq!(v_1[14], 0xCB);
    assert_eq!(v_1[15], 0xD0);
    assert_eq!(v_2[ 0], 0xE4);
    assert_eq!(v_2[ 1], 0xD9);
    assert_eq!(v_2[ 2], 0x09);
    assert_eq!(v_2[ 3], 0xC2);
    assert_eq!(v_2[ 4], 0x90);
    assert_eq!(v_2[ 5], 0xD0);
    assert_eq!(v_2[ 6], 0xFB);
    assert_eq!(v_2[ 7], 0x1C);
    assert_eq!(v_2[ 8], 0xA0);
    assert_eq!(v_2[ 9], 0x68);
    assert_eq!(v_2[10], 0xFF);
    assert_eq!(v_2[11], 0xAD);
    assert_eq!(v_2[12], 0xDF);
    assert_eq!(v_2[13], 0x22);
    assert_eq!(v_2[14], 0xCB);
    assert_eq!(v_2[15], 0xD0);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[ 0], 0xE4);
    assert_eq!(val[ 1], 0xD9);
    assert_eq!(val[ 2], 0x09);
    assert_eq!(val[ 3], 0xC2);
    assert_eq!(val[ 4], 0x90);
    assert_eq!(val[ 5], 0xD0);
    assert_eq!(val[ 6], 0xFB);
    assert_eq!(val[ 7], 0x1C);
    assert_eq!(val[ 8], 0xA0);
    assert_eq!(val[ 9], 0x68);
    assert_eq!(val[10], 0xFF);
    assert_eq!(val[11], 0xAD);
    assert_eq!(val[12], 0xDF);
    assert_eq!(val[13], 0x22);
    assert_eq!(val[14], 0xCB);
    assert_eq!(val[15], 0xD0);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_3() {
    let key = TXT_3.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0x42);
    assert_eq!(v_1[ 1], 0x24);
    assert_eq!(v_1[ 2], 0xB8);
    assert_eq!(v_1[ 3], 0x85);
    assert_eq!(v_1[ 4], 0x74);
    assert_eq!(v_1[ 5], 0x32);
    assert_eq!(v_1[ 6], 0x24);
    assert_eq!(v_1[ 7], 0x14);
    assert_eq!(v_1[ 8], 0x29);
    assert_eq!(v_1[ 9], 0x6C);
    assert_eq!(v_1[10], 0xFF);
    assert_eq!(v_1[11], 0xC6);
    assert_eq!(v_1[12], 0x14);
    assert_eq!(v_1[13], 0x00);
    assert_eq!(v_1[14], 0x1E);
    assert_eq!(v_1[15], 0x71);
    assert_eq!(v_2[ 0], 0x42);
    assert_eq!(v_2[ 1], 0x24);
    assert_eq!(v_2[ 2], 0xB8);
    assert_eq!(v_2[ 3], 0x85);
    assert_eq!(v_2[ 4], 0x74);
    assert_eq!(v_2[ 5], 0x32);
    assert_eq!(v_2[ 6], 0x24);
    assert_eq!(v_2[ 7], 0x14);
    assert_eq!(v_2[ 8], 0x29);
    assert_eq!(v_2[ 9], 0x6C);
    assert_eq!(v_2[10], 0xFF);
    assert_eq!(v_2[11], 0xC6);
    assert_eq!(v_2[12], 0x14);
    assert_eq!(v_2[13], 0x00);
    assert_eq!(v_2[14], 0x1E);
    assert_eq!(v_2[15], 0x71);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[ 0], 0x42);
    assert_eq!(v_1[ 1], 0x24);
    assert_eq!(v_1[ 2], 0xB8);
    assert_eq!(v_1[ 3], 0x85);
    assert_eq!(v_1[ 4], 0x74);
    assert_eq!(v_1[ 5], 0x32);
    assert_eq!(v_1[ 6], 0x24);
    assert_eq!(v_1[ 7], 0x14);
    assert_eq!(v_1[ 8], 0x29);
    assert_eq!(v_1[ 9], 0x6C);
    assert_eq!(v_1[10], 0xFF);
    assert_eq!(v_1[11], 0xC6);
    assert_eq!(v_1[12], 0x14);
    assert_eq!(v_1[13], 0x00);
    assert_eq!(v_1[14], 0x1E);
    assert_eq!(v_1[15], 0x71);
    assert_eq!(v_2[ 0], 0x42);
    assert_eq!(v_2[ 1], 0x24);
    assert_eq!(v_2[ 2], 0xB8);
    assert_eq!(v_2[ 3], 0x85);
    assert_eq!(v_2[ 4], 0x74);
    assert_eq!(v_2[ 5], 0x32);
    assert_eq!(v_2[ 6], 0x24);
    assert_eq!(v_2[ 7], 0x14);
    assert_eq!(v_2[ 8], 0x29);
    assert_eq!(v_2[ 9], 0x6C);
    assert_eq!(v_2[10], 0xFF);
    assert_eq!(v_2[11], 0xC6);
    assert_eq!(v_2[12], 0x14);
    assert_eq!(v_2[13], 0x00);
    assert_eq!(v_2[14], 0x1E);
    assert_eq!(v_2[15], 0x71);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[ 0], 0x42);
    assert_eq!(val[ 1], 0x24);
    assert_eq!(val[ 2], 0xB8);
    assert_eq!(val[ 3], 0x85);
    assert_eq!(val[ 4], 0x74);
    assert_eq!(val[ 5], 0x32);
    assert_eq!(val[ 6], 0x24);
    assert_eq!(val[ 7], 0x14);
    assert_eq!(val[ 8], 0x29);
    assert_eq!(val[ 9], 0x6C);
    assert_eq!(val[10], 0xFF);
    assert_eq!(val[11], 0xC6);
    assert_eq!(val[12], 0x14);
    assert_eq!(val[13], 0x00);
    assert_eq!(val[14], 0x1E);
    assert_eq!(val[15], 0x71);
}
