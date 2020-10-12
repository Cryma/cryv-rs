use crate::types::NativeVector3;

pub fn create_itemset(p0: bool) -> i32 {
    let value = native!(i32, 0x35AD299F50D91B24, native_parameters!(p0));

    value
}

pub fn destroy_itemset(p0: u32) -> () {
    let value = native!((), 0xDE18220B1C183EDA, native_parameters!(p0));

    value
}

pub fn is_itemset_valid(p0: u32) -> bool {
    let value = native!(bool, 0xB1B1EA596344DFAB, native_parameters!(p0));

    value
}

pub fn add_to_itemset(p0: u32, p1: u32) -> bool {
    let value = native!(bool, 0xE3945201F14637DD, native_parameters!(p0, p1));

    value
}

pub fn remove_from_itemset(p0: u32, p1: u32) -> () {
    let value = native!((), 0x25E68244B0177686, native_parameters!(p0, p1));

    value
}

pub fn get_itemset_size(x: u32) -> u32 {
    let value = native!(u32, 0xD9127E83ABF7C631, native_parameters!(x));

    value
}

pub fn get_indexed_item_in_itemset(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x7A197E2521EE2BAB, native_parameters!(p0, p1));

    value
}

pub fn is_in_itemset(p0: u32, p1: u32) -> bool {
    let value = native!(bool, 0x2D0FC594D1E9C107, native_parameters!(p0, p1));

    value
}

pub fn clean_itemset(p0: u32) -> () {
    let value = native!((), 0x41BC0D722FC04221, native_parameters!(p0));

    value
}
