static DOSYEAR_TO_STR: [u32; 128] = [
    0x31393830, 0x31393831, 0x31393832, 0x31393833, 0x31393834, 0x31393835, 0x31393836, 0x31393837,
    0x31393838, 0x31393839, 0x31393930, 0x31393931, 0x31393932, 0x31393933, 0x31393934, 0x31393935,
    0x31393936, 0x31393937, 0x31393938, 0x31393939, 0x32303030, 0x32303031, 0x32303032, 0x32303033,
    0x32303034, 0x32303035, 0x32303036, 0x32303037, 0x32303038, 0x32303039, 0x32303130, 0x32303131,
    0x32303132, 0x32303133, 0x32303134, 0x32303135, 0x32303136, 0x32303137, 0x32303138, 0x32303139,
    0x32303230, 0x32303231, 0x32303232, 0x32303233, 0x32303234, 0x32303235, 0x32303236, 0x32303237,
    0x32303238, 0x32303239, 0x32303330, 0x32303331, 0x32303332, 0x32303333, 0x32303334, 0x32303335,
    0x32303336, 0x32303337, 0x32303338, 0x32303339, 0x32303430, 0x32303431, 0x32303432, 0x32303433,
    0x32303434, 0x32303435, 0x32303436, 0x32303437, 0x32303438, 0x32303439, 0x32303530, 0x32303531,
    0x32303532, 0x32303533, 0x32303534, 0x32303535, 0x32303536, 0x32303537, 0x32303538, 0x32303539,
    0x32303630, 0x32303631, 0x32303632, 0x32303633, 0x32303634, 0x32303635, 0x32303636, 0x32303637,
    0x32303638, 0x32303639, 0x32303730, 0x32303731, 0x32303732, 0x32303733, 0x32303734, 0x32303735,
    0x32303736, 0x32303737, 0x32303738, 0x32303739, 0x32303830, 0x32303831, 0x32303832, 0x32303833,
    0x32303834, 0x32303835, 0x32303836, 0x32303837, 0x32303838, 0x32303839, 0x32303930, 0x32303931,
    0x32303932, 0x32303933, 0x32303934, 0x32303935, 0x32303936, 0x32303937, 0x32303938, 0x32303939,
    0x32313030, 0x32313031, 0x32313032, 0x32313033, 0x32313034, 0x32313035, 0x32313036, 0x32313037,
];

pub fn dosyear_to_string128(dosyear: u8) -> u32 {
    let ix: u8 = dosyear & 0x7f;
    DOSYEAR_TO_STR[ix as usize]
}

#[cfg(target_family = "wasm")]
#[cfg(feature = "dosyear2str128")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosyear2str128(dosyear: u8) -> u32 {
    dosyear_to_string128(dosyear)
}

pub fn dosyear_to_string(year: u16) -> u32 {
    let bound: u16 = year & 0xfff; // 0, 1980, 2107, 4095
    let add: u16 = bound + 68; // 68, 2048, 2175, 4163
    let and: u16 = add & 0x7f; // 68, 0, 127, 67
    dosyear_to_string128(and as u8)
}

#[cfg(target_family = "wasm")]
#[cfg(feature = "dosyear2str")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosyear2str(year: u16) -> u32 {
    dosyear_to_string(year)
}

#[cfg(test)]
mod test_dosyear2str {

    mod dosyear_to_string {
        macro_rules! create_test_dosyear2str {
            ($fname: ident, $input: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let input: u16 = $input;
                    let expected: u32 = $expected;

                    let got: u32 = crate::dosyear_to_string(input);

                    assert_eq!(got, expected);
                }
            };
        }

        create_test_dosyear2str!(test1980, 1980, 0x31393830);
        create_test_dosyear2str!(test1981, 1981, 0x31393831);
        create_test_dosyear2str!(test1982, 1982, 0x31393832);
        create_test_dosyear2str!(test1983, 1983, 0x31393833);
        create_test_dosyear2str!(test1984, 1984, 0x31393834);
        create_test_dosyear2str!(test1985, 1985, 0x31393835);
        create_test_dosyear2str!(test1986, 1986, 0x31393836);
        create_test_dosyear2str!(test1987, 1987, 0x31393837);
        create_test_dosyear2str!(test1988, 1988, 0x31393838);
        create_test_dosyear2str!(test1989, 1989, 0x31393839);

        create_test_dosyear2str!(test1990, 1990, 0x31393930);
        create_test_dosyear2str!(test2000, 2000, 0x32303030);
        create_test_dosyear2str!(test2010, 2010, 0x32303130);
        create_test_dosyear2str!(test2020, 2020, 0x32303230);
        create_test_dosyear2str!(test2030, 2030, 0x32303330);
        create_test_dosyear2str!(test2040, 2040, 0x32303430);
        create_test_dosyear2str!(test2050, 2050, 0x32303530);
        create_test_dosyear2str!(test2060, 2060, 0x32303630);
        create_test_dosyear2str!(test2070, 2070, 0x32303730);
        create_test_dosyear2str!(test2080, 2080, 0x32303830);
        create_test_dosyear2str!(test2090, 2090, 0x32303930);

        create_test_dosyear2str!(test2100, 2100, 0x32313030);
        create_test_dosyear2str!(test2101, 2101, 0x32313031);
        create_test_dosyear2str!(test2102, 2102, 0x32313032);
        create_test_dosyear2str!(test2103, 2103, 0x32313033);
        create_test_dosyear2str!(test2104, 2104, 0x32313034);
        create_test_dosyear2str!(test2105, 2105, 0x32313035);
        create_test_dosyear2str!(test2106, 2106, 0x32313036);
        create_test_dosyear2str!(test2107, 2107, 0x32313037);
    }
}
