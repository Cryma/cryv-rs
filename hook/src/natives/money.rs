use crate::types::NativeVector3;

pub fn network_initialize_cash(wallet: i32, bank: i32) -> () {
    let value = native!((), 0x3DA5ECD1A56CBA6D, native_parameters!(wallet, bank));

    value
}

pub fn network_delete_character(characterSlot: i32, p1: bool, p2: bool) -> () {
    let value = native!(
        (),
        0x05A50AF38947EB8D,
        native_parameters!(characterSlot, p1, p2)
    );

    value
}

pub fn _network_manual_delete_character(characterSlot: i32) -> () {
    let value = native!((), 0x821418C727FCACD7, native_parameters!(characterSlot));

    value
}

pub fn _network_get_is_high_earner() -> bool {
    let value = native!(bool, 0xFB2456B2040A6A67, native_parameters!());

    value
}

pub fn network_clear_character_wallet(characterSlot: i32) -> () {
    let value = native!((), 0xA921DED15FDF28F5, native_parameters!(characterSlot));

    value
}

pub fn network_give_player_jobshare_cash(amount: i32, networkHandle: *mut i32) -> () {
    let value = native!(
        (),
        0xFB18DF9CB95E0105,
        native_parameters!(amount, networkHandle)
    );

    value
}

pub fn network_receive_player_jobshare_cash(value: i32, networkHandle: *mut i32) -> () {
    let value = native!(
        (),
        0x56A3B51944C50598,
        native_parameters!(value, networkHandle)
    );

    value
}

pub fn network_can_share_job_cash() -> bool {
    let value = native!(bool, 0x1C2473301B1C66BA, native_parameters!());

    value
}

pub fn network_refund_cash(
    index: i32,
    context: &std::ffi::CString,
    reason: &std::ffi::CString,
    unk: bool,
) -> () {
    let value = native!(
        (),
        0xF9C812CD7C46E817,
        native_parameters!(index, context.as_ptr(), reason.as_ptr(), unk)
    );

    value
}

pub fn _network_deduct_cash(
    amount: i32,
    p1: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: bool,
    p4: bool,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x18B7AE224B087E26,
        native_parameters!(amount, p1.as_ptr(), p2.as_ptr(), p3, p4, p5)
    );

    value
}

pub fn network_money_can_bet(amount: i32, p1: bool, p2: bool) -> bool {
    let value = native!(bool, 0x81404F3DC124FE5B, native_parameters!(amount, p1, p2));

    value
}

pub fn network_can_bet(amount: i32) -> bool {
    let value = native!(bool, 0x3A54E33660DED67F, native_parameters!(amount));

    value
}

pub fn network_can_buy_lottery_ticket(cost: i32) -> bool {
    let value = native!(bool, 0xC62DD18375C99130, native_parameters!(cost));

    value
}

pub fn _network_casino_can_use_gambling_type(hash: u32) -> bool {
    let value = native!(bool, 0x158C16F5E4CF41F8, native_parameters!(hash));

    value
}

pub fn _network_casino_can_purchase_chips_with_pvc() -> bool {
    let value = native!(bool, 0x394DCDB9E836B7A9, native_parameters!());

    value
}

pub fn _network_casino_can_gamble(p0: u32) -> bool {
    let value = native!(bool, 0xF62F6D9528358FE4, native_parameters!(p0));

    value
}

pub fn _network_casino_can_purchase_chips_with_pvc_2() -> bool {
    let value = native!(bool, 0x8968D4D8C6C40C11, native_parameters!());

    value
}

pub fn _network_casino_purchase_chips(p0: i32, p1: i32) -> bool {
    let value = native!(bool, 0x3BD101471C7F9EEC, native_parameters!(p0, p1));

    value
}

pub fn _network_casino_sell_chips(p0: i32, p1: i32) -> bool {
    let value = native!(bool, 0xED44897CB336F480, native_parameters!(p0, p1));

    value
}

pub fn _0xcd0f5b5d932ae473() -> () {
    let value = native!((), 0xCD0F5B5D932AE473, native_parameters!());

    value
}

pub fn _can_pay_goon(p0: i32, p1: i32, amount: i32, p3: *mut i32) -> bool {
    let value = native!(
        bool,
        0x9777734DAD16992F,
        native_parameters!(p0, p1, amount, p3)
    );

    value
}

pub fn _network_earn_from_cashing_out(amount: i32) -> () {
    let value = native!((), 0xEFE9C9A1651B81E6, native_parameters!(amount));

    value
}

pub fn network_earn_from_pickup(amount: i32) -> () {
    let value = native!((), 0xED1517D3AF17C698, native_parameters!(amount));

    value
}

pub fn _network_earn_from_gang_pickup(amount: i32) -> () {
    let value = native!((), 0xA03D4ACE0A3284CE, native_parameters!(amount));

    value
}

pub fn _network_earn_from_assassinate_target_killed(amount: i32) -> () {
    let value = native!((), 0xFA700D8A9905F78A, native_parameters!(amount));

    value
}

pub fn _network_earn_from_armour_truck(amount: i32) -> () {
    let value = native!((), 0xF514621E8EA463D0, native_parameters!(amount));

    value
}

pub fn network_earn_from_crate_drop(amount: i32) -> () {
    let value = native!((), 0xB1CC1B9EC3007A2A, native_parameters!(amount));

    value
}

pub fn network_earn_from_betting(amount: i32, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x827A5BA1A44ACA6D,
        native_parameters!(amount, p1.as_ptr())
    );

    value
}

pub fn network_earn_from_job(amount: i32, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xB2CC4836834E8A98,
        native_parameters!(amount, p1.as_ptr())
    );

    value
}

pub fn _network_earn_from_job_x2(amount: i32, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xDEBBF584665411D0,
        native_parameters!(amount, p1.as_ptr())
    );

    value
}

pub fn _network_earn_from_premium_job(amount: i32, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xC8407624CEF2354B,
        native_parameters!(amount, p1.as_ptr())
    );

    value
}

pub fn network_earn_from_bend_job(amount: i32, heistHash: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x61326EE6DF15B0CA,
        native_parameters!(amount, heistHash.as_ptr())
    );

    value
}

pub fn network_earn_from_challenge_win(p0: u32, p1: *mut u32, p2: bool) -> () {
    let value = native!((), 0x2B171E6B2F64D8DF, native_parameters!(p0, p1, p2));

    value
}

pub fn network_earn_from_bounty(amount: i32, networkHandle: *mut i32, p2: *mut u32, p3: u32) -> () {
    let value = native!(
        (),
        0x131BB5DA15453ACF,
        native_parameters!(amount, networkHandle, p2, p3)
    );

    value
}

pub fn network_earn_from_import_export(amount: i32, modelHash: u32) -> () {
    let value = native!(
        (),
        0xF92A014A634442D6,
        native_parameters!(amount, modelHash)
    );

    value
}

pub fn network_earn_from_holdups(amount: i32) -> () {
    let value = native!((), 0x45B8154E077D9E4D, native_parameters!(amount));

    value
}

pub fn network_earn_from_property(amount: i32, propertyName: u32) -> () {
    let value = native!(
        (),
        0x849648349D77F5C5,
        native_parameters!(amount, propertyName)
    );

    value
}

pub fn network_earn_from_ai_target_kill(p0: u32, p1: u32) -> () {
    let value = native!((), 0x515B4A22E4D3C6D7, native_parameters!(p0, p1));

    value
}

pub fn network_earn_from_not_badsport(amount: i32) -> () {
    let value = native!((), 0x4337511FA8221D36, native_parameters!(amount));

    value
}

pub fn network_earn_from_rockstar(amount: i32) -> () {
    let value = native!((), 0x02CE1D6AC0FC73EA, native_parameters!(amount));

    value
}

pub fn network_earn_from_vehicle(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0xB539BD8A4C1EECF8,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn network_earn_from_personal_vehicle(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0x3F4D00167E41E0AD,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn network_earn_from_daily_objectives(p0: i32, p1: &std::ffi::CString, p2: i32) -> () {
    let value = native!(
        (),
        0x6EA318C91C1A8786,
        native_parameters!(p0, p1.as_ptr(), p2)
    );

    value
}

pub fn network_earn_from_ambient_job(p0: i32, p1: &std::ffi::CString, p2: *mut u32) -> () {
    let value = native!(
        (),
        0xFB6DB092FBAE29E6,
        native_parameters!(p0, p1.as_ptr(), p2)
    );

    value
}

pub fn network_earn_from_job_bonus(p0: u32, p1: *mut u32, p2: *mut u32) -> () {
    let value = native!((), 0x6816FB4416760775, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_from_criminal_mastermind_bonus(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xFA009A62990671D4, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_job_bonus_heist_award(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x9D4FDBB035229669, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_job_bonus_first_time_bonus(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x11B0A20C493F7E36, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_goon(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xCDA1C62BE2777802, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_boss(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x08B0CA7A6AB3AC32, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_boss_agency(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0CB1BE0633C024A8, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_earn_from_warehouse(amount: i32, id: i32) -> () {
    let value = native!((), 0x3E4ADAFF1830F146, native_parameters!(amount, id));

    value
}

pub fn _network_earn_from_contraband(amount: i32, p1: u32) -> () {
    let value = native!((), 0xECA658CE2A4E5A72, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_destroying_contraband(p0: u32) -> () {
    let value = native!((), 0x84C0116D012E8FC2, native_parameters!(p0));

    value
}

pub fn _0x6b7e4fb50d5f3d65(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x6B7E4FB50D5F3D65,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0x31ba138f6304fb9f(p0: u32, p1: u32) -> () {
    let value = native!((), 0x31BA138F6304FB9F, native_parameters!(p0, p1));

    value
}

pub fn _0x55a1e095db052fa5(p0: u32, p1: u32) -> () {
    let value = native!((), 0x55A1E095DB052FA5, native_parameters!(p0, p1));

    value
}

pub fn _network_earn_from_business_product(amount: i32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0x8586789730B10CAF,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_earn_from_vehicle_export(amount: i32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xEDEAD9A91EC768B3, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_earn_from_smuggling(amount: i32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0xDEE612F2D71B0308,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_earn_bounty_hunter_reward(p0: u32) -> () {
    let value = native!((), 0xF6B170F9A02E9E87, native_parameters!(p0));

    value
}

pub fn _network_earn_from_business_battle(p0: u32) -> () {
    let value = native!((), 0x42FCE14F50F27291, native_parameters!(p0));

    value
}

pub fn _network_earn_from_club_management_participation(p0: u32) -> () {
    let value = native!((), 0xA75EAC69F59E96E7, native_parameters!(p0));

    value
}

pub fn _network_earn_from_fmbb_phonecall_mission(p0: u32) -> () {
    let value = native!((), 0xC5156361F26E2212, native_parameters!(p0));

    value
}

pub fn _network_earn_from_business_hub_sell(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x0B39CF0D53F1C883, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_earn_from_fmbb_boss_work(p0: u32) -> () {
    let value = native!((), 0x1FDA0AA679C9919B, native_parameters!(p0));

    value
}

pub fn _network_earn_fmbb_wage_bonus(p0: u32) -> () {
    let value = native!((), 0xFFFBA1B1F7C0B6F4, native_parameters!(p0));

    value
}

pub fn network_can_spend_money(p0: u32, p1: bool, p2: bool, p3: bool, p4: u32, p5: u32) -> bool {
    let value = native!(
        bool,
        0xAB3CAA6B422164DA,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _network_can_spend_money_2(
    p0: u32,
    p1: bool,
    p2: bool,
    p3: bool,
    p4: *mut u32,
    p5: u32,
    p6: u32,
) -> bool {
    let value = native!(
        bool,
        0x7303E27CC6532080,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn network_buy_item(
    amount: i32,
    item: u32,
    p2: u32,
    p3: u32,
    p4: bool,
    item_name: &std::ffi::CString,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: bool,
) -> () {
    let value = native!(
        (),
        0xF0077C797F66A355,
        native_parameters!(amount, item, p2, p3, p4, item_name.as_ptr(), p6, p7, p8, p9)
    );

    value
}

pub fn network_spent_taxi(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x17C3A7D31EAE39F9, native_parameters!(amount, p1, p2));

    value
}

pub fn network_pay_employee_wage(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x5FD5ED82CBBE9989, native_parameters!(p0, p1, p2));

    value
}

pub fn network_pay_utility_bill(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xAFE08B35EC0C9EAE, native_parameters!(amount, p1, p2));

    value
}

pub fn network_pay_match_entry_fee(
    amount: i32,
    matchId: &std::ffi::CString,
    p2: bool,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x9346E14F2AF74D46,
        native_parameters!(amount, matchId.as_ptr(), p2, p3)
    );

    value
}

pub fn network_spent_betting(
    amount: i32,
    p1: i32,
    matchId: &std::ffi::CString,
    p3: bool,
    p4: bool,
) -> () {
    let value = native!(
        (),
        0x1C436FD11FFA692F,
        native_parameters!(amount, p1, matchId.as_ptr(), p3, p4)
    );

    value
}

pub fn _network_spent_wager(p0: u32, p1: u32, amount: i32) -> () {
    let value = native!((), 0xD99DB210089617FE, native_parameters!(p0, p1, amount));

    value
}

pub fn network_spent_in_stripclub(p0: u32, p1: bool, p2: u32, p3: bool) -> () {
    let value = native!((), 0xEE99784E4467689C, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_buy_healthcare(cost: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xD9B067E55253E3DD, native_parameters!(cost, p1, p2));

    value
}

pub fn network_buy_airstrike(cost: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x763B4BD305338F19, native_parameters!(cost, p1, p2));

    value
}

pub fn network_buy_backup_gang(p0: i32, p1: i32, p2: bool, p3: bool) -> () {
    let value = native!((), 0xA3EDDAA42411D3B9, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_buy_heli_strike(cost: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x81AA4610E3FD3A69, native_parameters!(cost, p1, p2));

    value
}

pub fn network_spent_ammo_drop(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xB162DC95C0A3317B, native_parameters!(p0, p1, p2));

    value
}

pub fn network_buy_bounty(amount: i32, victim: i32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x7B718E197453F2D9,
        native_parameters!(amount, victim, p2, p3)
    );

    value
}

pub fn network_buy_property(cost: i32, propertyName: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x650A08A280870AF6,
        native_parameters!(cost, propertyName, p2, p3)
    );

    value
}

pub fn network_buy_smokes(p0: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x75AF80E61248EEBD, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_heli_pickup(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x7BF1D73DB2ECA492, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_boat_pickup(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x524EE43A37232C00, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_bull_shark(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xA6DD8458CE24012C, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_cash_drop(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x289016EC778D60E0, native_parameters!(amount, p1, p2));

    value
}

pub fn network_spent_hire_mugger(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xE404BFB981665BF0, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_robbed_by_mugger(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x995A65F15F581359, native_parameters!(amount, p1, p2));

    value
}

pub fn network_spent_hire_mercenary(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xE7B80E2BF9D80BD6, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_buy_wantedlevel(p0: u32, p1: *mut u32, p2: bool, p3: bool) -> () {
    let value = native!((), 0xE1B13771A843C4F6, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_spent_buy_offtheradar(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xA628A745E2275C5D, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_buy_reveal_players(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x6E176F1B18BC0637, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_carwash(p0: u32, p1: u32, p2: u32, p3: bool, p4: bool) -> () {
    let value = native!(
        (),
        0xEC03C719DB2F4306,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn network_spent_cinema(p0: u32, p1: u32, p2: bool, p3: bool) -> () {
    let value = native!((), 0x6B38ECB05A63A685, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_spent_telescope(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x7FE61782AD94CC09, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_holdups(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xD9B86B9872039763, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_buy_passive_mode(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x6D3A430D1A809179, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_bank_interest(p0: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xCA230C9682556CF1, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_prostitutes(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xB21B89501CFAC79E, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_arrest_bail(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x812F5488B1B2A299, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_pay_vehicle_insurance_premium(
    amount: i32,
    vehicleModel: u32,
    networkHandle: *mut i32,
    notBankrupt: bool,
    hasTheMoney: bool,
) -> () {
    let value = native!(
        (),
        0x9FF28D88C766E3E8,
        native_parameters!(
            amount,
            vehicleModel,
            networkHandle,
            notBankrupt,
            hasTheMoney
        )
    );

    value
}

pub fn network_spent_call_player(p0: u32, p1: *mut u32, p2: bool, p3: bool) -> () {
    let value = native!((), 0xACDE7185B374177C, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_spent_bounty(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x29B260B84947DFCC, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_from_rockstar(p0: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x6A445B64ED7ABEB5, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x9b5016a6433a68c5() -> u32 {
    let value = native!(u32, 0x9B5016A6433A68C5, native_parameters!());

    value
}

pub fn process_cash_gift(p0: *mut i32, p1: *mut i32, p2: &std::ffi::CString) -> String {
    let value = native!(
        *const i8,
        0x20194D48EAEC9A41,
        native_parameters!(p0, p1, p2.as_ptr())
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _0xcd4d66b43b1dd28d(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xCD4D66B43B1DD28D, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_player_healthcare(p0: i32, p1: i32, p2: bool, p3: bool) -> () {
    let value = native!((), 0x7C99101F7FCE2EE5, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_spent_no_cops(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xD5BB406F4E04019F, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_request_job(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x8204DA7934DF3155, native_parameters!(p0, p1, p2));

    value
}

pub fn network_spent_request_heist(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x9D26502BB97BFE62, native_parameters!(p0, p1, p2));

    value
}

pub fn network_buy_lottery_ticket(p0: i32, p1: i32, p2: bool, p3: bool) -> () {
    let value = native!((), 0xD987F2489969668C, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_buy_fairground_ride(amount: i32, p1: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x8A7B3952DD64D2B5,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _0x7c4fccd2e4deb394() -> bool {
    let value = native!(bool, 0x7C4FCCD2E4DEB394, native_parameters!());

    value
}

pub fn _network_spent_job_skip(amount: i32, matchId: &std::ffi::CString, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x28F174A67B8D0C2F,
        native_parameters!(amount, matchId.as_ptr(), p2, p3)
    );

    value
}

pub fn _network_spent_boss(amount: i32, p1: bool, p2: bool) -> bool {
    let value = native!(bool, 0xFFBE02CD385356BD, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_pay_goon(p0: i32, p1: i32, amount: i32) -> () {
    let value = native!((), 0x08A1B82B91900682, native_parameters!(p0, p1, amount));

    value
}

pub fn _network_spent_pay_boss(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xDBC966A01C02BCA7, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_spent_move_yacht(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xE7DF4E0545DFB56E, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_rename_organization(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xFC4EE00A7B3BFB76, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_buy_contraband(p0: i32, p1: i32, p2: u32, p3: bool, p4: bool) -> () {
    let value = native!(
        (),
        0x30FD873ECE50E9F6,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _network_spent_vip_utility_charges(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x5182A339A3474510, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x112209ce0290c03a(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x112209CE0290C03A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xed5fd7af10f5e262(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xED5FD7AF10F5E262, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x0d30eb83668e63c5(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0D30EB83668E63C5, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_pa_service_dancer(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xB49ECA122467D05F, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xe23adc6fcb1f29ae(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xE23ADC6FCB1F29AE, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_spent_pa_service_heli_pickup(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0FE8E1FCD2B86B33, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x69ef772b192614c1(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x69EF772B192614C1, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x8e243837643d9583(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x8E243837643D9583, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xbd0efb25cca8f97a(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xBD0EFB25CCA8F97A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xa95f667a755725da(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xA95F667A755725DA, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_purchase_warehouse(amount: i32, data: *mut u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x33981D6804E62F49,
        native_parameters!(amount, data, p2, p3)
    );

    value
}

pub fn _0x4128464231e3ca0b(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x4128464231E3CA0B, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x2fab6614ce22e196(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x2FAB6614CE22E196, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_order_warehouse_vehicle(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x05F04155A226FBBF, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_order_bodyguard_vehicle(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xE8B0B270B6E7C76E, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_jukebox(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x5BCDE0F640C773D2, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x998e18ceb44487fc(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x998E18CEB44487FC, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xfa07759e6fddd7cf(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xFA07759E6FDDD7CF, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x6fd97159fe3c971a(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x6FD97159FE3C971A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x675d19c6067cae08(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x675D19C6067CAE08, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xa51b086b0b2c0f7a(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xA51B086B0B2C0F7A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_ba_service(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xD7CCCBA28C4ECAF0,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _network_spent_business(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0035BB914316F1E3, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x5f456788b05faeac(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x5F456788B05FAEAC, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_spent_vehicle_export_mods(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: u32,
) -> () {
    let value = native!(
        (),
        0xA75CCF58A60A5FD1,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn _0xb4c2ec463672474e(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xB4C2EC463672474E, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x2afc2d19b50797f2(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x2AFC2D19B50797F2, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_import_export_repair(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xC1952F3773BA18FE, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_spent_purchase_hangar(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xCCB339CC970452DA, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_upgrade_hangar(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x615EB504B0788DAF, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_hangar_utility_charges(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xB18AC2ECBB15CB6A, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_hangar_staff_charges(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xB1F1346FD57685D7, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_buy_truck(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xAC272C0AE01B4BD8, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_upgrade_truck(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x365E877C61D6988B, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_buy_bunker(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x12D148D26538D0F9, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_upgrade_bunker(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0C82D21A77C22D49, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_earn_from_sell_bunker(amount: i32, bunkerHash: u32) -> () {
    let value = native!(
        (),
        0x9251B6ABF2D0A5B4,
        native_parameters!(amount, bunkerHash)
    );

    value
}

pub fn _network_spent_ballistic_equipment(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x5D97630A8A0EF123, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_earn_from_rdr_bonus(amount: i32, p1: u32) -> () {
    let value = native!((), 0x7A5349B773584675, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_wage_payment(amount: i32) -> () {
    let value = native!((), 0x35F8DA0E8A31EF1B, native_parameters!(amount));

    value
}

pub fn _network_earn_from_wage_payment_bonus(amount: i32) -> () {
    let value = native!((), 0x005ACA7100BD101D, native_parameters!(amount));

    value
}

pub fn _network_spent_buy_base(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x4EA3F425C7744D21, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_upgrade_base(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x3DD3F33A5D55EA6F, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_buy_tiltrotor(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0CCE73BC7A11E885, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_upgrade_tiltrotor(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x165E135D6DFA2907, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_employ_assassins(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x5BBBD92186E1F1C5, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_gangops_cannon(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x771ADB0E7635B7BF, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_gangops_start_mission(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xDA947AE8880D5C18, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_casino_heist_skip_mission(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x487009DD91D93429, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_earn_from_sell_base(amount: i32, baseNameHash: u32) -> () {
    let value = native!(
        (),
        0x0E1E2FF3F4EC11AA,
        native_parameters!(amount, baseNameHash)
    );

    value
}

pub fn _network_earn_from_target_refund(amount: i32, p1: i32) -> () {
    let value = native!((), 0x5B669CF2299A271F, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_gangops_wages(amount: i32, p1: i32) -> () {
    let value = native!((), 0x2DCB19ABAB0380A8, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_gangops_wages_bonus(amount: i32, p1: i32) -> () {
    let value = native!((), 0x15BB2A5C757EB91F, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_dar_challenge(amount: i32, p1: u32) -> () {
    let value = native!((), 0xCAC672087B4A24AB, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_doomsday_finale_bonus(amount: i32, vehicleHash: u32) -> () {
    let value = native!(
        (),
        0x128A747F4A230952,
        native_parameters!(amount, vehicleHash)
    );

    value
}

pub fn _network_earn_from_gangops_awards(amount: i32, unk: &std::ffi::CString, p2: u32) -> () {
    let value = native!(
        (),
        0xA9A31475F530DFDA,
        native_parameters!(amount, unk.as_ptr(), p2)
    );

    value
}

pub fn _network_earn_from_gangops_elite(amount: i32, unk: &std::ffi::CString, actIndex: i32) -> () {
    let value = native!(
        (),
        0x2597A0D4A4FC2C77,
        native_parameters!(amount, unk.as_ptr(), actIndex)
    );

    value
}

pub fn _network_rival_delivery_completed(earnedMoney: i32) -> () {
    let value = native!((), 0x1B882107C23A9022, native_parameters!(earnedMoney));

    value
}

pub fn _network_spent_gangops_start_strand(type_esc: i32, amount: i32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0xA19EC0786E326E06,
        native_parameters!(type_esc, amount, p2, p3)
    );

    value
}

pub fn _network_spent_gangops_trip_skip(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x5ECE6FD7B4EC8D6A, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_earn_from_gangops_jobs_prep_participation(amount: i32) -> () {
    let value = native!((), 0xED26584F6BDCBBFD, native_parameters!(amount));

    value
}

pub fn _network_earn_from_gangops_jobs_setup(amount: i32, unk: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xA9160796D47A2CF8,
        native_parameters!(amount, unk.as_ptr())
    );

    value
}

pub fn _network_earn_from_gangops_jobs_finale(amount: i32, unk: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x1C121FC9545E0D52,
        native_parameters!(amount, unk.as_ptr())
    );

    value
}

pub fn _0x2a7cec72c3443bcc(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x2A7CEC72C3443BCC, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xe0f82d68c7039158(p0: u32) -> () {
    let value = native!((), 0xE0F82D68C7039158, native_parameters!(p0));

    value
}

pub fn _0xb4deae67f35e2acd(p0: u32) -> () {
    let value = native!((), 0xB4DEAE67F35E2ACD, native_parameters!(p0));

    value
}

pub fn _network_earn_from_bb_event_bonus(amount: i32) -> () {
    let value = native!((), 0xFDD8D2440DAF1590, native_parameters!(amount));

    value
}

pub fn _0x2a93c46aab1eacc9(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x2A93C46AAB1EACC9, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x226c284c830d0ca8(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x226C284C830D0CA8, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_earn_from_hacker_truck_mission(p0: u32, amount: i32, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0xE8815FE993896AD3,
        native_parameters!(p0, amount, p2, p3)
    );

    value
}

pub fn _0xed76d195e6e3bf7f(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xED76D195E6E3BF7F, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x1dc9b749e7ae282b(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x1DC9B749E7AE282B, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xc6e74cf8c884c880(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0xC6E74CF8C884C880,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0x65482bfd0923c8a1(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x65482BFD0923C8A1,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _network_spent_rdrhatchet_bonus(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xE284D46FFDB82E36, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_nightclub_entry_fee(
    player: i32,
    amount: i32,
    p1: u32,
    p2: bool,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x876056684281655D,
        native_parameters!(player, amount, p1, p2, p3)
    );

    value
}

pub fn _network_spent_nightclub_bar_drink(amount: i32, p1: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0xDD21B016E4289465,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_spent_bounty_hunter_mission(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x1BEA0CD93470BB1F, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_rehire_dj(amount: i32, p1: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0xF6C8A544E4CF14FC,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_spent_arena_join_spectator(amount: i32, p1: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x14EAEA58F93B55AF,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_earn_from_arena_skill_level_progression(amount: i32, p1: u32) -> () {
    let value = native!((), 0xE08256F972C7BB2C, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_from_arena_career_progression(amount: i32, p1: u32) -> () {
    let value = native!((), 0x0F99F70C61F14619, native_parameters!(amount, p1));

    value
}

pub fn _network_spent_make_it_rain(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xE5F5A060439C2F5D, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_spent_buy_arena(amount: i32, p1: bool, p2: bool, p3: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x40D5DA9550B7CB46,
        native_parameters!(amount, p1, p2, p3.as_ptr())
    );

    value
}

pub fn _network_spent_upgrade_arena(amount: i32, p1: bool, p2: bool, p3: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x037ABB06825D7AB1,
        native_parameters!(amount, p1, p2, p3.as_ptr())
    );

    value
}

pub fn _network_spent_arena_spectator_box(amount: i32, p1: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x7049BF858601DC0F,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_spent_spin_the_wheel_payment(amount: i32, p1: u32, p2: bool) -> () {
    let value = native!((), 0x9A5BD1D0000B339C, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_earn_from_spin_the_wheel_cash(amount: i32) -> () {
    let value = native!((), 0x676C48776CACBB5A, native_parameters!(amount));

    value
}

pub fn _network_spent_arena_premium(amount: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x619496D837EFD920, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_earn_from_arena_war(amount: i32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0x631F1CB8FB4130AA,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_earn_from_assassinate_target_killed_2(amount: i32) -> () {
    let value = native!((), 0x5E7AE8AABE8B7C0D, native_parameters!(amount));

    value
}

pub fn _network_earn_from_bb_event_cargo(amount: i32) -> () {
    let value = native!((), 0xA82959062361B259, native_parameters!(amount));

    value
}

pub fn _network_earn_from_rc_time_trial(amount: i32) -> () {
    let value = native!((), 0xDFF49EE984E7AAE8, native_parameters!(amount));

    value
}

pub fn _network_earn_from_daily_objective_event(amount: i32) -> () {
    let value = native!((), 0x5128DF14A5BB86FC, native_parameters!(amount));

    value
}

pub fn _network_spent_casino_membership(amount: i32, p1: bool, p2: bool, p3: i32) -> () {
    let value = native!(
        (),
        0xFBBE0570EDF39D46,
        native_parameters!(amount, p1, p2, p3)
    );

    value
}

pub fn _network_spent_buy_casino(amount: i32, p1: bool, p2: bool, data: *mut u32) -> () {
    let value = native!(
        (),
        0x34A6FC4D06C4DA0F,
        native_parameters!(amount, p1, p2, data)
    );

    value
}

pub fn _network_spent_upgrade_casino(amount: i32, p1: bool, p2: bool, data: *mut u32) -> () {
    let value = native!(
        (),
        0x4740D62BC1B4EBEA,
        native_parameters!(amount, p1, p2, data)
    );

    value
}

pub fn _network_spent_casino_generic(amount: i32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x88BF9B612B84D3C3,
        native_parameters!(amount, p1, p2, p3, p4)
    );

    value
}

pub fn _network_earn_from_time_trial_win(amount: i32) -> () {
    let value = native!((), 0x0819DB99FD2FBBD8, native_parameters!(amount));

    value
}

pub fn _network_earn_from_collectables_action_figures(amount: i32) -> () {
    let value = native!((), 0x5517F90043466049, native_parameters!(amount));

    value
}

pub fn _network_earn_from_complete_collection(amount: i32) -> () {
    let value = native!((), 0x83AD64F53F4E9483, native_parameters!(amount));

    value
}

pub fn _network_earn_from_selling_vehicle(amount: i32) -> () {
    let value = native!((), 0x8BCB27A057DF7B7F, native_parameters!(amount));

    value
}

pub fn _network_earn_from_casino_mission_reward(amount: i32) -> () {
    let value = native!((), 0x566FD402B25787DE, native_parameters!(amount));

    value
}

pub fn _network_earn_from_casino_story_mission_reward(amount: i32) -> () {
    let value = native!((), 0xAC95ED552157E092, native_parameters!(amount));

    value
}

pub fn _network_earn_from_casino_mission_participation(amount: i32) -> () {
    let value = native!((), 0x09E8F18641BE2575, native_parameters!(amount));

    value
}

pub fn _network_earn_from_casino_award(amount: i32, hash: u32) -> () {
    let value = native!((), 0x973A9781A34F8DEB, native_parameters!(amount, hash));

    value
}

pub fn _0x870289a558348378(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x870289A558348378, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x5574637681911fda(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x5574637681911FDA, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_casino_heist(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: u32,
    p10: u32,
) -> () {
    let value = native!(
        (),
        0xD30E8392F407C328,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn _0xb5b58e24868cb09e(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xB5B58E24868CB09E,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _network_spent_arcade_game(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xEAD3D81F2C3A1458,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _network_spent_arcade_generic(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x43AA7FAC4E6D6687,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _network_earn_casino_heist(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0x72E7C7B9615FA3C3,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0x4c3b75694f7e0d9c(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x4C3B75694F7E0D9C, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xd29334ed1a256dbf(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xD29334ED1A256DBF,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0xa95cfb4e02390842(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xA95CFB4E02390842, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x0dd362f14f18942a(amount: i32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x0DD362F14F18942A, native_parameters!(amount, p1, p2));

    value
}

pub fn _network_earn_casino_heist_bonus(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x3EC7471E6909798A,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _network_earn_from_collection_item(amount: i32, p1: u32) -> () {
    let value = native!((), 0x84FF63BD4966F33D, native_parameters!(amount, p1));

    value
}

pub fn _network_earn_collectable_completed_collection(amount: i32, p1: u32) -> () {
    let value = native!((), 0x5C9B198AF5A54FA6, native_parameters!(amount, p1));

    value
}

pub fn _0xde68e30d89f97132(amount: i32, p1: u32) -> () {
    let value = native!((), 0xDE68E30D89F97132, native_parameters!(amount, p1));

    value
}

pub fn _0xe2e244ab823b4483(amount: i32, p1: u32) -> () {
    let value = native!((), 0xE2E244AB823B4483, native_parameters!(amount, p1));

    value
}

pub fn _network_spent_beach_party_generic(p0: u32) -> () {
    let value = native!((), 0x54ABA22FA6371249, native_parameters!(p0));

    value
}

pub fn _network_spent_submarine(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x6C8BC1488527AAAB,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _network_spent_casino_club_generic(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0xC991C255AA6D90B2,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn _0x90cd7c6871fbf1b4(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x90CD7C6871FBF1B4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x89049a84065ce68e(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x89049A84065CE68E, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_spent_island_heist(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xE86689E5F82DE429, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_earn_island_heist(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0xD21D111C46BA9F15,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0xa51338e0dccd4065(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xA51338E0DCCD4065, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xe2bb399d90942091(p0: u32, p1: u32) -> () {
    let value = native!((), 0xE2BB399D90942091, native_parameters!(p0, p1));

    value
}

pub fn network_get_vc_bank_balance() -> i32 {
    let value = native!(i32, 0x76EF28DA05EA395A, native_parameters!());

    value
}

pub fn network_get_vc_wallet_balance(characterSlot: i32) -> i32 {
    let value = native!(i32, 0xA40F9C2623F6A8B5, native_parameters!(characterSlot));

    value
}

pub fn network_get_vc_balance() -> i32 {
    let value = native!(i32, 0x5CBAD97E059E1B94, native_parameters!());

    value
}

pub fn network_get_evc_balance() -> i32 {
    let value = native!(i32, 0x5D1E75F91C07DEE5, native_parameters!());

    value
}

pub fn network_get_pvc_balance() -> i32 {
    let value = native!(i32, 0x4F54F3B6C202FB4E, native_parameters!());

    value
}

pub fn network_get_string_wallet_balance(characterSlot: i32) -> String {
    let value = native!(
        *const i8,
        0xF9B10B529DCFB33B,
        native_parameters!(characterSlot)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_get_string_bank_balance() -> String {
    let value = native!(*const i8, 0xA6FA3979BED01B81, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_get_string_bank_wallet_balance() -> String {
    let value = native!(*const i8, 0x700AF71AE615E6DD, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _network_get_vc_wallet_balance_is_not_less_than(amount: i32, characterSlot: i32) -> bool {
    let value = native!(
        bool,
        0xED5AB8860415BABA,
        native_parameters!(amount, characterSlot)
    );

    value
}

pub fn _network_get_vc_bank_balance_is_not_less_than(amount: i32) -> bool {
    let value = native!(bool, 0xA31FD6A0865B6D14, native_parameters!(amount));

    value
}

pub fn _network_get_vc_bank_wallet_balance_is_not_less_than(
    amount: i32,
    characterSlot: i32,
) -> bool {
    let value = native!(
        bool,
        0xDC18531D7019A535,
        native_parameters!(amount, characterSlot)
    );

    value
}

pub fn network_get_pvc_transfer_balance() -> i32 {
    let value = native!(i32, 0x13A8DE2FD77D04F3, native_parameters!());

    value
}

pub fn _0x08e8eeadfd0dc4a0(amount: i32) -> bool {
    let value = native!(bool, 0x08E8EEADFD0DC4A0, native_parameters!(amount));

    value
}

pub fn network_can_receive_player_cash(p0: u32, p1: u32, p2: u32, p3: u32) -> bool {
    let value = native!(bool, 0x5D17BE59D2123284, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_get_remaining_transfer_balance() -> i32 {
    let value = native!(i32, 0xEA560AC9EEB1E19B, native_parameters!());

    value
}

pub fn withdraw_vc(amount: i32) -> i32 {
    let value = native!(i32, 0xF70EFA14FE091429, native_parameters!(amount));

    value
}

pub fn deposit_vc(amount: i32) -> bool {
    let value = native!(bool, 0xE260E0BB9CD995AC, native_parameters!(amount));

    value
}

pub fn _0xe154b48b68ef72bc(p0: u32) -> bool {
    let value = native!(bool, 0xE154B48B68EF72BC, native_parameters!(p0));

    value
}

pub fn _0x6fcf8ddea146c45b(p0: u32) -> bool {
    let value = native!(bool, 0x6FCF8DDEA146C45B, native_parameters!(p0));

    value
}
