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
const TXT_4: &str = "In terms of practical security, a major concern about these new attacks is that they might pave the way to more efficient ones. Whether this is the case is yet to be seen, but a migration to stronger hashes is believed to be prudent. Some of the applications that use cryptographic hashes, like password storage, are only minimally affected by a collision attack. Constructing a password that works for a given account requires a preimage attack, as well as access to the hash of the original password, which may or may not be trivial. Reversing password encryption (e.g. to obtain a password to try against a user's account elsewhere) is not made possible by the attacks. (However, even a secure password hash can\'t prevent brute-force attacks on weak passwords.)";
/*----------------------------------------------------------------------------*/
const TXT_5: &str = "0";
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
    assert_eq!(v_1[0], 0xFF);
    assert_eq!(v_1[1], 0xFF);
    assert_eq!(v_2[0], 0xFF);
    assert_eq!(v_2[1], 0xFF);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0xFF);
    assert_eq!(v_1[1], 0xFF);
    assert_eq!(v_2[0], 0xFF);
    assert_eq!(v_2[1], 0xFF);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[0], 0xFF);
    assert_eq!(val[1], 0xFF);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_1() {
    let key = TXT_1.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x72);
    assert_eq!(v_1[1], 0xA4);
    assert_eq!(v_2[0], 0x72);
    assert_eq!(v_2[1], 0xA4);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x72);
    assert_eq!(v_1[1], 0xA4);
    assert_eq!(v_2[0], 0x72);
    assert_eq!(v_2[1], 0xA4);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[0], 0x72);
    assert_eq!(val[1], 0xA4);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_2() {
    let key = TXT_2.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x72);
    assert_eq!(v_1[1], 0x76);
    assert_eq!(v_2[0], 0x72);
    assert_eq!(v_2[1], 0x76);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x72);
    assert_eq!(v_1[1], 0x76);
    assert_eq!(v_2[0], 0x72);
    assert_eq!(v_2[1], 0x76);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[0], 0x72);
    assert_eq!(val[1], 0x76);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_3() {
    let key = TXT_3.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x77);
    assert_eq!(v_1[1], 0x70);
    assert_eq!(v_2[0], 0x77);
    assert_eq!(v_2[1], 0x70);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x77);
    assert_eq!(v_1[1], 0x70);
    assert_eq!(v_2[0], 0x77);
    assert_eq!(v_2[1], 0x70);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[0], 0x77);
    assert_eq!(val[1], 0x70);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_4() {
    let key = TXT_4.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x05);
    assert_eq!(v_1[1], 0x03);
    assert_eq!(v_2[0], 0x05);
    assert_eq!(v_2[1], 0x03);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0x05);
    assert_eq!(v_1[1], 0x03);
    assert_eq!(v_2[0], 0x05);
    assert_eq!(v_2[1], 0x03);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[0], 0x05);
    assert_eq!(val[1], 0x03);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_5() {
    let key = TXT_5.as_bytes();

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_normal(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0xCF);
    assert_eq!(v_1[1], 0xFF);
    assert_eq!(v_2[0], 0xCF);
    assert_eq!(v_2[1], 0xFF);

    let mut v_1: [u8; super::Ctx::LEN_DGT] = Default::default();
    let mut v_2: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_byte_by_byte(key, &mut v_1, &mut v_2);
    assert_eq!(v_1[0], 0xCF);
    assert_eq!(v_1[1], 0xFF);
    assert_eq!(v_2[0], 0xCF);
    assert_eq!(v_2[1], 0xFF);

    let mut val: [u8; super::Ctx::LEN_DGT] = Default::default();

    tst_one(key, &mut val);
    assert_eq!(val[0], 0xCF);
    assert_eq!(val[1], 0xFF);
}
