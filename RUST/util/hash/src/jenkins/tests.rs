#[test]
fn exe_u32() {
    let key = &[
        0x00000000_u32, 0x77073096_u32, 0xee0e612c_u32, 0x990951ba_u32,
        0x076dc419_u32, 0x706af48f_u32, 0xe963a535_u32, 0x9e6495a3_u32,
        0x0edb8832_u32, 0x79dcb8a4_u32, 0xe0d5e91e_u32, 0x97d2d988_u32,
        0x09b64c2b_u32, 0x7eb17cbd_u32, 0xe7b82d07_u32, 0x90bf1d91_u32,
        0x1db71064_u32, 0x6ab020f2_u32, 0xf3b97148_u32, 0x84be41de_u32,
        0x1adad47d_u32, 0x6ddde4eb_u32, 0xf4d4b551_u32, 0x83d385c7_u32,
        0x136c9856_u32, 0x646ba8c0_u32, 0xfd62f97a_u32, 0x8a65c9ec_u32,
        0x14015c4f_u32, 0x63066cd9_u32, 0xfa0f3d63_u32, 0x8d080df5_u32,
        0x3b6e20c8_u32, 0x4c69105e_u32, 0xd56041e4_u32, 0xa2677172_u32,
        0x3c03e4d1_u32, 0x4b04d447_u32, 0xd20d85fd_u32, 0xa50ab56b_u32,
        0x35b5a8fa_u32, 0x42b2986c_u32, 0xdbbbc9d6_u32, 0xacbcf940_u32,
        0x32d86ce3_u32, 0x45df5c75_u32, 0xdcd60dcf_u32, 0xabd13d59_u32,
        0x26d930ac_u32, 0x51de003a_u32, 0xc8d75180_u32, 0xbfd06116_u32,
        0x21b4f4b5_u32, 0x56b3c423_u32, 0xcfba9599_u32, 0xb8bda50f_u32,
        0x2802b89e_u32, 0x5f058808_u32, 0xc60cd9b2_u32, 0xb10be924_u32,
        0x2f6f7c87_u32, 0x58684c11_u32, 0xc1611dab_u32, 0xb6662d3d_u32,
        0x76dc4190_u32, 0x01db7106_u32, 0x98d220bc_u32, 0xefd5102a_u32,
        0x71b18589_u32, 0x06b6b51f_u32, 0x9fbfe4a5_u32, 0xe8b8d433_u32,
        0x7807c9a2_u32, 0x0f00f934_u32, 0x9609a88e_u32, 0xe10e9818_u32,
        0x7f6a0dbb_u32, 0x086d3d2d_u32, 0x91646c97_u32, 0xe6635c01_u32,
        0x6b6b51f4_u32, 0x1c6c6162_u32, 0x856530d8_u32, 0xf262004e_u32,
        0x6c0695ed_u32, 0x1b01a57b_u32, 0x8208f4c1_u32, 0xf50fc457_u32,
        0x65b0d9c6_u32, 0x12b7e950_u32, 0x8bbeb8ea_u32, 0xfcb9887c_u32,
        0x62dd1ddf_u32, 0x15da2d49_u32, 0x8cd37cf3_u32, 0xfbd44c65_u32,
        0x4db26158_u32, 0x3ab551ce_u32, 0xa3bc0074_u32, 0xd4bb30e2_u32,
        0x4adfa541_u32, 0x3dd895d7_u32, 0xa4d1c46d_u32, 0xd3d6f4fb_u32,
        0x4369e96a_u32, 0x346ed9fc_u32, 0xad678846_u32, 0xda60b8d0_u32,
        0x44042d73_u32, 0x33031de5_u32, 0xaa0a4c5f_u32, 0xdd0d7cc9_u32,
        0x5005713c_u32, 0x270241aa_u32, 0xbe0b1010_u32, 0xc90c2086_u32,
        0x5768b525_u32, 0x206f85b3_u32, 0xb966d409_u32, 0xce61e49f_u32,
        0x5edef90e_u32, 0x29d9c998_u32, 0xb0d09822_u32, 0xc7d7a8b4_u32,
        0x59b33d17_u32, 0x2eb40d81_u32, 0xb7bd5c3b_u32, 0xc0ba6cad_u32,
        0xedb88320_u32, 0x9abfb3b6_u32, 0x03b6e20c_u32, 0x74b1d29a_u32,
        0xead54739_u32, 0x9dd277af_u32, 0x04db2615_u32, 0x73dc1683_u32,
        0xe3630b12_u32, 0x94643b84_u32, 0x0d6d6a3e_u32, 0x7a6a5aa8_u32,
        0xe40ecf0b_u32, 0x9309ff9d_u32, 0x0a00ae27_u32, 0x7d079eb1_u32,
        0xf00f9344_u32, 0x8708a3d2_u32, 0x1e01f268_u32, 0x6906c2fe_u32,
        0xf762575d_u32, 0x806567cb_u32, 0x196c3671_u32, 0x6e6b06e7_u32,
        0xfed41b76_u32, 0x89d32be0_u32, 0x10da7a5a_u32, 0x67dd4acc_u32,
        0xf9b9df6f_u32, 0x8ebeeff9_u32, 0x17b7be43_u32, 0x60b08ed5_u32,
        0xd6d6a3e8_u32, 0xa1d1937e_u32, 0x38d8c2c4_u32, 0x4fdff252_u32,
        0xd1bb67f1_u32, 0xa6bc5767_u32, 0x3fb506dd_u32, 0x48b2364b_u32,
        0xd80d2bda_u32, 0xaf0a1b4c_u32, 0x36034af6_u32, 0x41047a60_u32,
        0xdf60efc3_u32, 0xa867df55_u32, 0x316e8eef_u32, 0x4669be79_u32,
        0xcb61b38c_u32, 0xbc66831a_u32, 0x256fd2a0_u32, 0x5268e236_u32,
        0xcc0c7795_u32, 0xbb0b4703_u32, 0x220216b9_u32, 0x5505262f_u32,
        0xc5ba3bbe_u32, 0xb2bd0b28_u32, 0x2bb45a92_u32, 0x5cb36a04_u32,
        0xc2d7ffa7_u32, 0xb5d0cf31_u32, 0x2cd99e8b_u32, 0x5bdeae1d_u32,
        0x9b64c2b0_u32, 0xec63f226_u32, 0x756aa39c_u32, 0x026d930a_u32,
        0x9c0906a9_u32, 0xeb0e363f_u32, 0x72076785_u32, 0x05005713_u32,
        0x95bf4a82_u32, 0xe2b87a14_u32, 0x7bb12bae_u32, 0x0cb61b38_u32,
        0x92d28e9b_u32, 0xe5d5be0d_u32, 0x7cdcefb7_u32, 0x0bdbdf21_u32,
        0x86d3d2d4_u32, 0xf1d4e242_u32, 0x68ddb3f8_u32, 0x1fda836e_u32,
        0x81be16cd_u32, 0xf6b9265b_u32, 0x6fb077e1_u32, 0x18b74777_u32,
        0x88085ae6_u32, 0xff0f6a70_u32, 0x66063bca_u32, 0x11010b5c_u32,
        0x8f659eff_u32, 0xf862ae69_u32, 0x616bffd3_u32, 0x166ccf45_u32,
        0xa00ae278_u32, 0xd70dd2ee_u32, 0x4e048354_u32, 0x3903b3c2_u32,
        0xa7672661_u32, 0xd06016f7_u32, 0x4969474d_u32, 0x3e6e77db_u32,
        0xaed16a4a_u32, 0xd9d65adc_u32, 0x40df0b66_u32, 0x37d83bf0_u32,
        0xa9bcae53_u32, 0xdebb9ec5_u32, 0x47b2cf7f_u32, 0x30b5ffe9_u32,
        0xbdbdf21c_u32, 0xcabac28a_u32, 0x53b39330_u32, 0x24b4a3a6_u32,
        0xbad03605_u32, 0xcdd70693_u32, 0x54de5729_u32, 0x23d967bf_u32,
        0xb3667a2e_u32, 0xc4614ab8_u32, 0x5d681b02_u32, 0x2a6f2b94_u32,
        0xb40bbe37_u32, 0xc30c8ea1_u32, 0x5a05df1b_u32, 0x2d02ef8d_u32
    ];

    let x = super::x_n(key, 1234567890_u32);
    assert_eq!(x, 2446089946);

    let x = super::x_3(key[10], key[20], key[30], key[40]);
    assert_eq!(x, 3025208094);

    let x = super::x_2(key[11], key[22], key[33]);
    assert_eq!(x, 3579518633);

    let x = super::x_1(key[44], key[55]);
    assert_eq!(x, 74525844);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_u8() {
    let key = &[
        0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x96_u8, 0x30_u8, 0x07_u8, 0x77_u8,
        0x2c_u8, 0x61_u8, 0x0e_u8, 0xee_u8, 0xba_u8, 0x51_u8, 0x09_u8, 0x99_u8,
        0x19_u8, 0xc4_u8, 0x6d_u8, 0x07_u8, 0x8f_u8, 0xf4_u8, 0x6a_u8, 0x70_u8,
        0x35_u8, 0xa5_u8, 0x63_u8, 0xe9_u8, 0xa3_u8, 0x95_u8, 0x64_u8, 0x9e_u8,
        0x32_u8, 0x88_u8, 0xdb_u8, 0x0e_u8, 0xa4_u8, 0xb8_u8, 0xdc_u8, 0x79_u8,
        0x1e_u8, 0xe9_u8, 0xd5_u8, 0xe0_u8, 0x88_u8, 0xd9_u8, 0xd2_u8, 0x97_u8,
        0x2b_u8, 0x4c_u8, 0xb6_u8, 0x09_u8, 0xbd_u8, 0x7c_u8, 0xb1_u8, 0x7e_u8,
        0x07_u8, 0x2d_u8, 0xb8_u8, 0xe7_u8, 0x91_u8, 0x1d_u8, 0xbf_u8, 0x90_u8,
        0x64_u8, 0x10_u8, 0xb7_u8, 0x1d_u8, 0xf2_u8, 0x20_u8, 0xb0_u8, 0x6a_u8,
        0x48_u8, 0x71_u8, 0xb9_u8, 0xf3_u8, 0xde_u8, 0x41_u8, 0xbe_u8, 0x84_u8,
        0x7d_u8, 0xd4_u8, 0xda_u8, 0x1a_u8, 0xeb_u8, 0xe4_u8, 0xdd_u8, 0x6d_u8,
        0x51_u8, 0xb5_u8, 0xd4_u8, 0xf4_u8, 0xc7_u8, 0x85_u8, 0xd3_u8, 0x83_u8,
        0x56_u8, 0x98_u8, 0x6c_u8, 0x13_u8, 0xc0_u8, 0xa8_u8, 0x6b_u8, 0x64_u8,
        0x7a_u8, 0xf9_u8, 0x62_u8, 0xfd_u8, 0xec_u8, 0xc9_u8, 0x65_u8, 0x8a_u8,
        0x4f_u8, 0x5c_u8, 0x01_u8, 0x14_u8, 0xd9_u8, 0x6c_u8, 0x06_u8, 0x63_u8,
        0x63_u8, 0x3d_u8, 0x0f_u8, 0xfa_u8, 0xf5_u8, 0x0d_u8, 0x08_u8, 0x8d_u8,
        0xc8_u8, 0x20_u8, 0x6e_u8, 0x3b_u8, 0x5e_u8, 0x10_u8, 0x69_u8, 0x4c_u8,
        0xe4_u8, 0x41_u8, 0x60_u8, 0xd5_u8, 0x72_u8, 0x71_u8, 0x67_u8, 0xa2_u8,
        0xd1_u8, 0xe4_u8, 0x03_u8, 0x3c_u8, 0x47_u8, 0xd4_u8, 0x04_u8, 0x4b_u8,
        0xfd_u8, 0x85_u8, 0x0d_u8, 0xd2_u8, 0x6b_u8, 0xb5_u8, 0x0a_u8, 0xa5_u8,
        0xfa_u8, 0xa8_u8, 0xb5_u8, 0x35_u8, 0x6c_u8, 0x98_u8, 0xb2_u8, 0x42_u8,
        0xd6_u8, 0xc9_u8, 0xbb_u8, 0xdb_u8, 0x40_u8, 0xf9_u8, 0xbc_u8, 0xac_u8,
        0xe3_u8, 0x6c_u8, 0xd8_u8, 0x32_u8, 0x75_u8, 0x5c_u8, 0xdf_u8, 0x45_u8,
        0xcf_u8, 0x0d_u8, 0xd6_u8, 0xdc_u8, 0x59_u8, 0x3d_u8, 0xd1_u8, 0xab_u8,
        0xac_u8, 0x30_u8, 0xd9_u8, 0x26_u8, 0x3a_u8, 0x00_u8, 0xde_u8, 0x51_u8,
        0x80_u8, 0x51_u8, 0xd7_u8, 0xc8_u8, 0x16_u8, 0x61_u8, 0xd0_u8, 0xbf_u8,
        0xb5_u8, 0xf4_u8, 0xb4_u8, 0x21_u8, 0x23_u8, 0xc4_u8, 0xb3_u8, 0x56_u8,
        0x99_u8, 0x95_u8, 0xba_u8, 0xcf_u8, 0x0f_u8, 0xa5_u8, 0xbd_u8, 0xb8_u8,
        0x9e_u8, 0xb8_u8, 0x02_u8, 0x28_u8, 0x08_u8, 0x88_u8, 0x05_u8, 0x5f_u8,
        0xb2_u8, 0xd9_u8, 0x0c_u8, 0xc6_u8, 0x24_u8, 0xe9_u8, 0x0b_u8, 0xb1_u8,
        0x87_u8, 0x7c_u8, 0x6f_u8, 0x2f_u8, 0x11_u8, 0x4c_u8, 0x68_u8, 0x58_u8,
        0xab_u8, 0x1d_u8, 0x61_u8, 0xc1_u8, 0x3d_u8, 0x2d_u8, 0x66_u8, 0xb6_u8,
        0x90_u8, 0x41_u8, 0xdc_u8, 0x76_u8, 0x06_u8, 0x71_u8, 0xdb_u8, 0x01_u8,
        0xbc_u8, 0x20_u8, 0xd2_u8, 0x98_u8, 0x2a_u8, 0x10_u8, 0xd5_u8, 0xef_u8,
        0x89_u8, 0x85_u8, 0xb1_u8, 0x71_u8, 0x1f_u8, 0xb5_u8, 0xb6_u8, 0x06_u8,
        0xa5_u8, 0xe4_u8, 0xbf_u8, 0x9f_u8, 0x33_u8, 0xd4_u8, 0xb8_u8, 0xe8_u8,
        0xa2_u8, 0xc9_u8, 0x07_u8, 0x78_u8, 0x34_u8, 0xf9_u8, 0x00_u8, 0x0f_u8,
        0x8e_u8, 0xa8_u8, 0x09_u8, 0x96_u8, 0x18_u8, 0x98_u8, 0x0e_u8, 0xe1_u8,
        0xbb_u8, 0x0d_u8, 0x6a_u8, 0x7f_u8, 0x2d_u8, 0x3d_u8, 0x6d_u8, 0x08_u8,
        0x97_u8, 0x6c_u8, 0x64_u8, 0x91_u8, 0x01_u8, 0x5c_u8, 0x63_u8, 0xe6_u8,
        0xf4_u8, 0x51_u8, 0x6b_u8, 0x6b_u8, 0x62_u8, 0x61_u8, 0x6c_u8, 0x1c_u8,
        0xd8_u8, 0x30_u8, 0x65_u8, 0x85_u8, 0x4e_u8, 0x00_u8, 0x62_u8, 0xf2_u8,
        0xed_u8, 0x95_u8, 0x06_u8, 0x6c_u8, 0x7b_u8, 0xa5_u8, 0x01_u8, 0x1b_u8,
        0xc1_u8, 0xf4_u8, 0x08_u8, 0x82_u8, 0x57_u8, 0xc4_u8, 0x0f_u8, 0xf5_u8,
        0xc6_u8, 0xd9_u8, 0xb0_u8, 0x65_u8, 0x50_u8, 0xe9_u8, 0xb7_u8, 0x12_u8,
        0xea_u8, 0xb8_u8, 0xbe_u8, 0x8b_u8, 0x7c_u8, 0x88_u8, 0xb9_u8, 0xfc_u8,
        0xdf_u8, 0x1d_u8, 0xdd_u8, 0x62_u8, 0x49_u8, 0x2d_u8, 0xda_u8, 0x15_u8,
        0xf3_u8, 0x7c_u8, 0xd3_u8, 0x8c_u8, 0x65_u8, 0x4c_u8, 0xd4_u8, 0xfb_u8,
        0x58_u8, 0x61_u8, 0xb2_u8, 0x4d_u8, 0xce_u8, 0x51_u8, 0xb5_u8, 0x3a_u8,
        0x74_u8, 0x00_u8, 0xbc_u8, 0xa3_u8, 0xe2_u8, 0x30_u8, 0xbb_u8, 0xd4_u8,
        0x41_u8, 0xa5_u8, 0xdf_u8, 0x4a_u8, 0xd7_u8, 0x95_u8, 0xd8_u8, 0x3d_u8,
        0x6d_u8, 0xc4_u8, 0xd1_u8, 0xa4_u8, 0xfb_u8, 0xf4_u8, 0xd6_u8, 0xd3_u8,
        0x6a_u8, 0xe9_u8, 0x69_u8, 0x43_u8, 0xfc_u8, 0xd9_u8, 0x6e_u8, 0x34_u8,
        0x46_u8, 0x88_u8, 0x67_u8, 0xad_u8, 0xd0_u8, 0xb8_u8, 0x60_u8, 0xda_u8,
        0x73_u8, 0x2d_u8, 0x04_u8, 0x44_u8, 0xe5_u8, 0x1d_u8, 0x03_u8, 0x33_u8,
        0x5f_u8, 0x4c_u8, 0x0a_u8, 0xaa_u8, 0xc9_u8, 0x7c_u8, 0x0d_u8, 0xdd_u8,
        0x3c_u8, 0x71_u8, 0x05_u8, 0x50_u8, 0xaa_u8, 0x41_u8, 0x02_u8, 0x27_u8,
        0x10_u8, 0x10_u8, 0x0b_u8, 0xbe_u8, 0x86_u8, 0x20_u8, 0x0c_u8, 0xc9_u8,
        0x25_u8, 0xb5_u8, 0x68_u8, 0x57_u8, 0xb3_u8, 0x85_u8, 0x6f_u8, 0x20_u8,
        0x09_u8, 0xd4_u8, 0x66_u8, 0xb9_u8, 0x9f_u8, 0xe4_u8, 0x61_u8, 0xce_u8,
        0x0e_u8, 0xf9_u8, 0xde_u8, 0x5e_u8, 0x98_u8, 0xc9_u8, 0xd9_u8, 0x29_u8,
        0x22_u8, 0x98_u8, 0xd0_u8, 0xb0_u8, 0xb4_u8, 0xa8_u8, 0xd7_u8, 0xc7_u8,
        0x17_u8, 0x3d_u8, 0xb3_u8, 0x59_u8, 0x81_u8, 0x0d_u8, 0xb4_u8, 0x2e_u8,
        0x3b_u8, 0x5c_u8, 0xbd_u8, 0xb7_u8, 0xad_u8, 0x6c_u8, 0xba_u8, 0xc0_u8,
        0x20_u8, 0x83_u8, 0xb8_u8, 0xed_u8, 0xb6_u8, 0xb3_u8, 0xbf_u8, 0x9a_u8,
        0x0c_u8, 0xe2_u8, 0xb6_u8, 0x03_u8, 0x9a_u8, 0xd2_u8, 0xb1_u8, 0x74_u8,
        0x39_u8, 0x47_u8, 0xd5_u8, 0xea_u8, 0xaf_u8, 0x77_u8, 0xd2_u8, 0x9d_u8,
        0x15_u8, 0x26_u8, 0xdb_u8, 0x04_u8, 0x83_u8, 0x16_u8, 0xdc_u8, 0x73_u8,
        0x12_u8, 0x0b_u8, 0x63_u8, 0xe3_u8, 0x84_u8, 0x3b_u8, 0x64_u8, 0x94_u8,
        0x3e_u8, 0x6a_u8, 0x6d_u8, 0x0d_u8, 0xa8_u8, 0x5a_u8, 0x6a_u8, 0x7a_u8,
        0x0b_u8, 0xcf_u8, 0x0e_u8, 0xe4_u8, 0x9d_u8, 0xff_u8, 0x09_u8, 0x93_u8,
        0x27_u8, 0xae_u8, 0x00_u8, 0x0a_u8, 0xb1_u8, 0x9e_u8, 0x07_u8, 0x7d_u8,
        0x44_u8, 0x93_u8, 0x0f_u8, 0xf0_u8, 0xd2_u8, 0xa3_u8, 0x08_u8, 0x87_u8,
        0x68_u8, 0xf2_u8, 0x01_u8, 0x1e_u8, 0xfe_u8, 0xc2_u8, 0x06_u8, 0x69_u8,
        0x5d_u8, 0x57_u8, 0x62_u8, 0xf7_u8, 0xcb_u8, 0x67_u8, 0x65_u8, 0x80_u8,
        0x71_u8, 0x36_u8, 0x6c_u8, 0x19_u8, 0xe7_u8, 0x06_u8, 0x6b_u8, 0x6e_u8,
        0x76_u8, 0x1b_u8, 0xd4_u8, 0xfe_u8, 0xe0_u8, 0x2b_u8, 0xd3_u8, 0x89_u8,
        0x5a_u8, 0x7a_u8, 0xda_u8, 0x10_u8, 0xcc_u8, 0x4a_u8, 0xdd_u8, 0x67_u8,
        0x6f_u8, 0xdf_u8, 0xb9_u8, 0xf9_u8, 0xf9_u8, 0xef_u8, 0xbe_u8, 0x8e_u8,
        0x43_u8, 0xbe_u8, 0xb7_u8, 0x17_u8, 0xd5_u8, 0x8e_u8, 0xb0_u8, 0x60_u8,
        0xe8_u8, 0xa3_u8, 0xd6_u8, 0xd6_u8, 0x7e_u8, 0x93_u8, 0xd1_u8, 0xa1_u8,
        0xc4_u8, 0xc2_u8, 0xd8_u8, 0x38_u8, 0x52_u8, 0xf2_u8, 0xdf_u8, 0x4f_u8,
        0xf1_u8, 0x67_u8, 0xbb_u8, 0xd1_u8, 0x67_u8, 0x57_u8, 0xbc_u8, 0xa6_u8,
        0xdd_u8, 0x06_u8, 0xb5_u8, 0x3f_u8, 0x4b_u8, 0x36_u8, 0xb2_u8, 0x48_u8,
        0xda_u8, 0x2b_u8, 0x0d_u8, 0xd8_u8, 0x4c_u8, 0x1b_u8, 0x0a_u8, 0xaf_u8,
        0xf6_u8, 0x4a_u8, 0x03_u8, 0x36_u8, 0x60_u8, 0x7a_u8, 0x04_u8, 0x41_u8,
        0xc3_u8, 0xef_u8, 0x60_u8, 0xdf_u8, 0x55_u8, 0xdf_u8, 0x67_u8, 0xa8_u8,
        0xef_u8, 0x8e_u8, 0x6e_u8, 0x31_u8, 0x79_u8, 0xbe_u8, 0x69_u8, 0x46_u8,
        0x8c_u8, 0xb3_u8, 0x61_u8, 0xcb_u8, 0x1a_u8, 0x83_u8, 0x66_u8, 0xbc_u8,
        0xa0_u8, 0xd2_u8, 0x6f_u8, 0x25_u8, 0x36_u8, 0xe2_u8, 0x68_u8, 0x52_u8,
        0x95_u8, 0x77_u8, 0x0c_u8, 0xcc_u8, 0x03_u8, 0x47_u8, 0x0b_u8, 0xbb_u8,
        0xb9_u8, 0x16_u8, 0x02_u8, 0x22_u8, 0x2f_u8, 0x26_u8, 0x05_u8, 0x55_u8,
        0xbe_u8, 0x3b_u8, 0xba_u8, 0xc5_u8, 0x28_u8, 0x0b_u8, 0xbd_u8, 0xb2_u8,
        0x92_u8, 0x5a_u8, 0xb4_u8, 0x2b_u8, 0x04_u8, 0x6a_u8, 0xb3_u8, 0x5c_u8,
        0xa7_u8, 0xff_u8, 0xd7_u8, 0xc2_u8, 0x31_u8, 0xcf_u8, 0xd0_u8, 0xb5_u8,
        0x8b_u8, 0x9e_u8, 0xd9_u8, 0x2c_u8, 0x1d_u8, 0xae_u8, 0xde_u8, 0x5b_u8,
        0xb0_u8, 0xc2_u8, 0x64_u8, 0x9b_u8, 0x26_u8, 0xf2_u8, 0x63_u8, 0xec_u8,
        0x9c_u8, 0xa3_u8, 0x6a_u8, 0x75_u8, 0x0a_u8, 0x93_u8, 0x6d_u8, 0x02_u8,
        0xa9_u8, 0x06_u8, 0x09_u8, 0x9c_u8, 0x3f_u8, 0x36_u8, 0x0e_u8, 0xeb_u8,
        0x85_u8, 0x67_u8, 0x07_u8, 0x72_u8, 0x13_u8, 0x57_u8, 0x00_u8, 0x05_u8,
        0x82_u8, 0x4a_u8, 0xbf_u8, 0x95_u8, 0x14_u8, 0x7a_u8, 0xb8_u8, 0xe2_u8,
        0xae_u8, 0x2b_u8, 0xb1_u8, 0x7b_u8, 0x38_u8, 0x1b_u8, 0xb6_u8, 0x0c_u8,
        0x9b_u8, 0x8e_u8, 0xd2_u8, 0x92_u8, 0x0d_u8, 0xbe_u8, 0xd5_u8, 0xe5_u8,
        0xb7_u8, 0xef_u8, 0xdc_u8, 0x7c_u8, 0x21_u8, 0xdf_u8, 0xdb_u8, 0x0b_u8,
        0xd4_u8, 0xd2_u8, 0xd3_u8, 0x86_u8, 0x42_u8, 0xe2_u8, 0xd4_u8, 0xf1_u8,
        0xf8_u8, 0xb3_u8, 0xdd_u8, 0x68_u8, 0x6e_u8, 0x83_u8, 0xda_u8, 0x1f_u8,
        0xcd_u8, 0x16_u8, 0xbe_u8, 0x81_u8, 0x5b_u8, 0x26_u8, 0xb9_u8, 0xf6_u8,
        0xe1_u8, 0x77_u8, 0xb0_u8, 0x6f_u8, 0x77_u8, 0x47_u8, 0xb7_u8, 0x18_u8,
        0xe6_u8, 0x5a_u8, 0x08_u8, 0x88_u8, 0x70_u8, 0x6a_u8, 0x0f_u8, 0xff_u8,
        0xca_u8, 0x3b_u8, 0x06_u8, 0x66_u8, 0x5c_u8, 0x0b_u8, 0x01_u8, 0x11_u8,
        0xff_u8, 0x9e_u8, 0x65_u8, 0x8f_u8, 0x69_u8, 0xae_u8, 0x62_u8, 0xf8_u8,
        0xd3_u8, 0xff_u8, 0x6b_u8, 0x61_u8, 0x45_u8, 0xcf_u8, 0x6c_u8, 0x16_u8,
        0x78_u8, 0xe2_u8, 0x0a_u8, 0xa0_u8, 0xee_u8, 0xd2_u8, 0x0d_u8, 0xd7_u8,
        0x54_u8, 0x83_u8, 0x04_u8, 0x4e_u8, 0xc2_u8, 0xb3_u8, 0x03_u8, 0x39_u8,
        0x61_u8, 0x26_u8, 0x67_u8, 0xa7_u8, 0xf7_u8, 0x16_u8, 0x60_u8, 0xd0_u8,
        0x4d_u8, 0x47_u8, 0x69_u8, 0x49_u8, 0xdb_u8, 0x77_u8, 0x6e_u8, 0x3e_u8,
        0x4a_u8, 0x6a_u8, 0xd1_u8, 0xae_u8, 0xdc_u8, 0x5a_u8, 0xd6_u8, 0xd9_u8,
        0x66_u8, 0x0b_u8, 0xdf_u8, 0x40_u8, 0xf0_u8, 0x3b_u8, 0xd8_u8, 0x37_u8,
        0x53_u8, 0xae_u8, 0xbc_u8, 0xa9_u8, 0xc5_u8, 0x9e_u8, 0xbb_u8, 0xde_u8,
        0x7f_u8, 0xcf_u8, 0xb2_u8, 0x47_u8, 0xe9_u8, 0xff_u8, 0xb5_u8, 0x30_u8,
        0x1c_u8, 0xf2_u8, 0xbd_u8, 0xbd_u8, 0x8a_u8, 0xc2_u8, 0xba_u8, 0xca_u8,
        0x30_u8, 0x93_u8, 0xb3_u8, 0x53_u8, 0xa6_u8, 0xa3_u8, 0xb4_u8, 0x24_u8,
        0x05_u8, 0x36_u8, 0xd0_u8, 0xba_u8, 0x93_u8, 0x06_u8, 0xd7_u8, 0xcd_u8,
        0x29_u8, 0x57_u8, 0xde_u8, 0x54_u8, 0xbf_u8, 0x67_u8, 0xd9_u8, 0x23_u8,
        0x2e_u8, 0x7a_u8, 0x66_u8, 0xb3_u8, 0xb8_u8, 0x4a_u8, 0x61_u8, 0xc4_u8,
        0x02_u8, 0x1b_u8, 0x68_u8, 0x5d_u8, 0x94_u8, 0x2b_u8, 0x6f_u8, 0x2a_u8,
        0x37_u8, 0xbe_u8, 0x0b_u8, 0xb4_u8, 0xa1_u8, 0x8e_u8, 0x0c_u8, 0xc3_u8,
        0x1b_u8, 0xdf_u8, 0x05_u8, 0x5a_u8, 0x8d_u8, 0xef_u8, 0x02_u8, 0x2d_u8
    ];

    let x = super::x_0(key, 1234567890_u32);
    assert_eq!(x, 2446089946);
}
