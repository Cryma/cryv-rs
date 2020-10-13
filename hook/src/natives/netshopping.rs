use crate::types::NativeVector3;

pub fn _net_gameserver_use_server_transactions() -> bool {
    let value = native!(bool, 0x7D2708796355B20B, native_parameters!());

    value
}

pub fn _net_gameserver_catalog_item_exists(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xBD4D7EAF8A30F637, native_parameters!(name.as_ptr()));

    value
}

pub fn _net_gameserver_catalog_item_exists_hash(hash: u32) -> bool {
    let value = native!(bool, 0x247F0F73A182EA0B, native_parameters!(hash));

    value
}

pub fn net_gameserver_get_price(itemHash: u32, categoryHash: u32, p2: bool) -> i32 {
    let value = native!(
        i32,
        0xC27009422FCCA88D,
        native_parameters!(itemHash, categoryHash, p2)
    );

    value
}

pub fn _net_gameserver_catalog_is_ready() -> bool {
    let value = native!(bool, 0x3C4487461E9B0DCB, native_parameters!());

    value
}

pub fn _net_gameserver_is_catalog_valid() -> bool {
    let value = native!(bool, 0x2B949A1E6AEC8F6A, native_parameters!());

    value
}

pub fn _0x85f6c9aba1de2bcf() -> u32 {
    let value = native!(u32, 0x85F6C9ABA1DE2BCF, native_parameters!());

    value
}

pub fn _0x357b152ef96c30b6() -> u32 {
    let value = native!(u32, 0x357B152EF96C30B6, native_parameters!());

    value
}

pub fn _net_gameserver_get_catalog_state(state: *mut i32) -> bool {
    let value = native!(bool, 0xCF38DAFBB49EDE5E, native_parameters!(state));

    value
}

pub fn _0xe3e5a7c64ca2c6ed() -> u32 {
    let value = native!(u32, 0xE3E5A7C64CA2C6ED, native_parameters!());

    value
}

pub fn _0x0395cb47b022e62c(p0: *mut i32) -> bool {
    let value = native!(bool, 0x0395CB47B022E62C, native_parameters!(p0));

    value
}

pub fn net_gameserver_start_session(charSlot: i32) -> bool {
    let value = native!(bool, 0xA135AC892A58FC07, native_parameters!(charSlot));

    value
}

pub fn _0x72eb7ba9b69bf6ab() -> bool {
    let value = native!(bool, 0x72EB7BA9B69BF6AB, native_parameters!());

    value
}

pub fn _0x170910093218c8b9(p0: *mut i32) -> bool {
    let value = native!(bool, 0x170910093218C8B9, native_parameters!(p0));

    value
}

pub fn _0xc13c38e47ea5df31(p0: *mut i32) -> bool {
    let value = native!(bool, 0xC13C38E47EA5DF31, native_parameters!(p0));

    value
}

pub fn net_gameserver_is_session_valid(charSlot: i32) -> bool {
    let value = native!(bool, 0xB24F0944DA203D9E, native_parameters!(charSlot));

    value
}

pub fn _0x74a0fd0688f1ee45(p0: i32) -> i32 {
    let value = native!(i32, 0x74A0FD0688F1EE45, native_parameters!(p0));

    value
}

pub fn net_gameserver_session_apply_received_data(charSlot: i32) -> bool {
    let value = native!(bool, 0x2F41D51BA3BCD1F1, native_parameters!(charSlot));

    value
}

pub fn net_gameserver_is_session_refresh_pending() -> bool {
    let value = native!(bool, 0x810E8431C0614BF9, native_parameters!());

    value
}

pub fn _net_gameserver_get_balance(inventory: bool, playerbalance: bool) -> bool {
    let value = native!(
        bool,
        0x35A1B3E1D1315CFA,
        native_parameters!(inventory, playerbalance)
    );

    value
}

pub fn _0x613f125ba3bd2eb9() -> bool {
    let value = native!(bool, 0x613F125BA3BD2EB9, native_parameters!());

    value
}

pub fn _net_gameserver_get_transaction_manager_data(p0: *mut i32, p1: *mut bool) -> bool {
    let value = native!(bool, 0x897433D292B44130, native_parameters!(p0, p1));

    value
}

pub fn net_gameserver_basket_start(
    transactionId: *mut i32,
    categoryHash: u32,
    actionHash: u32,
    flags: i32,
) -> bool {
    let value = native!(
        bool,
        0x279F08B1A4B29B7E,
        native_parameters!(transactionId, categoryHash, actionHash, flags)
    );

    value
}

pub fn _net_gameserver_basket_delete() -> bool {
    let value = native!(bool, 0xFA336E7F40C0A0D0, native_parameters!());

    value
}

pub fn net_gameserver_basket_end() -> bool {
    let value = native!(bool, 0xA65568121DF2EA26, native_parameters!());

    value
}

pub fn net_gameserver_basket_add_item(itemData: *mut u32, quantity: i32) -> bool {
    let value = native!(
        bool,
        0xF30980718C8ED876,
        native_parameters!(itemData, quantity)
    );

    value
}

pub fn net_gameserver_basket_is_full() -> bool {
    let value = native!(bool, 0x27F76CC6C55AD30E, native_parameters!());

    value
}

pub fn net_gameserver_basket_apply_server_data(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xE1A0450ED46A7812, native_parameters!(p0, p1));

    value
}

pub fn net_gameserver_checkout_start(transactionId: i32) -> bool {
    let value = native!(bool, 0x39BE7CEA8D9CC8E6, native_parameters!(transactionId));

    value
}

pub fn net_gameserver_begin_service(
    transactionId: *mut i32,
    categoryHash: u32,
    itemHash: u32,
    actionTypeHash: u32,
    value: i32,
    flags: i32,
) -> bool {
    let value = native!(
        bool,
        0x3C5FD37B5499582E,
        native_parameters!(
            transactionId,
            categoryHash,
            itemHash,
            actionTypeHash,
            value,
            flags
        )
    );

    value
}

pub fn net_gameserver_end_service(transactionId: i32) -> bool {
    let value = native!(bool, 0xE2A99A9B524BEFFF, native_parameters!(transactionId));

    value
}

pub fn _net_gameserver_delete_character_slot(slot: i32, transfer: bool, reason: u32) -> bool {
    let value = native!(
        bool,
        0x51F1A8E48C3D2F6D,
        native_parameters!(slot, transfer, reason)
    );

    value
}

pub fn _net_gameserver_delete_character_slot_get_status() -> i32 {
    let value = native!(i32, 0x0A6D923DFFC9BD89, native_parameters!());

    value
}

pub fn net_gameserver_delete_set_telemetry_nonce_seed() -> bool {
    let value = native!(bool, 0x112CEF1615A1139F, native_parameters!());

    value
}

pub fn _net_gameserver_transfer_bank_to_wallet(charSlot: i32, amount: i32) -> bool {
    let value = native!(
        bool,
        0xD47A2C1BA117471D,
        native_parameters!(charSlot, amount)
    );

    value
}

pub fn _net_gameserver_transfer_wallet_to_bank(charSlot: i32, amount: i32) -> bool {
    let value = native!(
        bool,
        0xC2F7FE5309181C7D,
        native_parameters!(charSlot, amount)
    );

    value
}

pub fn _net_gameserver_transfer_cash_get_status() -> i32 {
    let value = native!(i32, 0x23789E777D14CE44, native_parameters!());

    value
}

pub fn _net_gameserver_transfer_cash_get_status_2() -> i32 {
    let value = native!(i32, 0x350AA5EBC03D3BD2, native_parameters!());

    value
}

pub fn _net_gameserver_transfer_cash_set_telemetry_nonce_seed() -> bool {
    let value = native!(bool, 0x498C1E05CE5F7877, native_parameters!());

    value
}

pub fn net_gameserver_set_telemetry_nonce_seed(p0: i32) -> bool {
    let value = native!(bool, 0x9507D4271988E1AE, native_parameters!(p0));

    value
}
