use crate::memory;

#[repr(C)]
struct ScriptHeader {
    padding1: [u8; 16],
    code_blocks_offset: *mut *mut u8,
    padding2: [u8; 4],
    code_length: i32,
    padding3: [u8; 4],
    local_count: i32,
    padding4: [u8; 4],
    native_count: i32,
    local_offset: *mut i64,
    padding5: [u8; 8],
    native_offset: *mut i64,
    padding6: [u8; 16],
    name_hash: i32,
    padding7: [u8; 4],
    name: *mut i8,
    strings_offset: *mut *mut i8,
    string_size: i32,
    padding8: [u8; 12],
}

impl ScriptHeader {
    fn code_page_count(&self) -> i32 {
        (self.code_length + 0x3FFF) >> 14
    }

    fn get_code_page_size(&self, page: i32) -> i32 {
        if page < 0 || page >= self.code_page_count() {
            return 0;
        }

        if page == self.code_page_count() - 1 {
            return self.code_length & 0x3FFF;
        }

        0x4000
    }

    fn get_code_page_address(&self, page: i32) -> *mut u8 {
        unsafe { *self.code_blocks_offset.offset(page as isize) }
    }
}

#[repr(C)]
struct ScriptTableItem {
    header: *mut ScriptHeader,
    padding: [u8; 4],
    hash: i32,
}

impl ScriptTableItem {
    fn is_loaded(&self) -> bool {
        self.header != std::ptr::null_mut()
    }
}

#[repr(C)]
struct ScriptTable {
    table_ptr: *mut ScriptTableItem,
    padding: [u8; 16],
    count: i32,
}

impl ScriptTable {
    fn find_script(&self, hash: i32) -> *mut ScriptTableItem {
        if self.table_ptr == std::ptr::null_mut() {
            return std::ptr::null_mut();
        }

        for i in 0..self.count {
            if unsafe { (*self.table_ptr.offset(i as isize)).hash } == hash {
                return unsafe { self.table_ptr.offset(i as isize) };
            }
        }

        std::ptr::null_mut()
    }
}

#[repr(C)]
struct GlobalTable {
    global_base_ptr: *mut *mut i64,
}

impl GlobalTable {
    fn address_of(&self, index: i32) -> *mut i64 {
        unsafe {
            (*self.global_base_ptr.offset((index >> 18 & 0x3F) as isize))
                .offset((index & 0x3FFFF) as isize)
        }
    }

    fn is_initialized(&self) -> bool {
        unsafe {
            self.global_base_ptr != std::ptr::null_mut()
                && *self.global_base_ptr != std::ptr::null_mut()
        }
    }
}

pub unsafe fn patch_multiplayer_vehicles() {
    let mut global_table = GlobalTable {
        global_base_ptr: std::ptr::null_mut(),
    };

    let global_base_pointer =
        memory::get_pattern("4C 8D 05 ? ? ? ? 4D 8B 08 4D 85 C9 74 11".to_owned(), 0);
    let global_base_pointer =
        global_base_pointer.add(((*(global_base_pointer.add(0x3) as *mut u32)) + 0x7) as usize);
    global_table.global_base_ptr = global_base_pointer as *mut *mut i64;

    let script_table_pointer =
        memory::get_pattern("48 03 15 ? ? ? ? 4C 23 C2 49 8B 08".to_owned(), 0);
    let script_table_pointer =
        script_table_pointer.add(((*(script_table_pointer.add(0x3) as *mut u32)) + 0x7) as usize);

    let script_table = script_table_pointer as *mut ScriptTable;

    while global_table.is_initialized() == false {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let script_table_item = (*script_table).find_script(0x39DA738B);
    if script_table_item == std::ptr::null_mut() {
        return;
    }

    while (*script_table_item).is_loaded() == false {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let script_header = (*script_table_item).header;
    for i in 0..(*script_header).code_page_count() {
        let size = (*script_header).get_code_page_size(i);
        if size > 0 {
            let code_page_address =
                (*script_header).get_code_page_address(i) as *mut std::ffi::c_void;

            let address = memory::get_pattern_in_memory_region(
                "2D ? ? 00 00 2C 01 ? ? 56 04 00 6E 2E ? 01 5F ? ? ? ? 04 00 6E 2E ? 01".to_owned(),
                code_page_address,
                size,
            );

            if address != std::ptr::null_mut() {
                let global_index = (*(address.add(17) as *mut i32)) & 0xFFFFFF;
                (*global_table.address_of(global_index)) = 1;
            }
        }
    }
}
