use super::*;

// -------- SHIELD --------

// PRE
unsafe extern "C" fn chariotsight_shield_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NONE, 
        *GROUND_CORRECT_KIND_NONE as u32, 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn chariotsight_shield_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shield"), 0.0, 1.0, false, 0.0, false, false); // add to motion list

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);

    SHIELD_ON[ENTRY_ID] = true;

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    weapon.fastshift(L2CValue::Ptr(chariotsight_shield_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn chariotsight_shield_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);

    // Snap to center position
    let mut offset_add = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut newPos = Vector3f{x:0.0,y:0.0,z:0.0};
    newPos = Vector3f{x: PostureModule::pos_x(owner_boma) + offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};

    PostureModule::set_pos(weapon.module_accessor, &newPos);
    ArticleModule::set_pos(weapon.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_CHARIOTSIGHT, newPos);

    SHIELD_LIFE[ENTRY_ID] -= 1;

    // REMOVE IF DOWN B PRESSED AGAIN
    let stick_x = ControlModule::get_stick_x(owner_boma);
	let stick_y = ControlModule::get_stick_y(owner_boma);
    if ControlModule::check_button_trigger(owner_boma, *CONTROL_PAD_BUTTON_SPECIAL) && 
    (stick_x < 0.5 && stick_x > -0.5) && stick_y < 0.5 {
        chariotsight_remove(weapon);
        return 0.into();
    }

    // CREATE A SOUND EFFECT ON 8 SECONDS AND 9 SECONDS BEFORE BREAKING

    // BREAK IF OUT OF LIFE
    if SHIELD_LIFE[ENTRY_ID] <= 0 {
        weapon.change_status(WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_BREAK.into(), false.into());
        return 1.into();
    }

    // ADD A BREAK FOR DAMAGE IF POSSIBLE
    
    return 0.into();
}

// END
unsafe extern "C" fn chariotsight_shield_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// -------- BREAK --------

// PRE
unsafe extern "C" fn chariotsight_break_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NONE, 
        *GROUND_CORRECT_KIND_NONE as u32, 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn chariotsight_break_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 0.0, 1.0, false, 0.0, false, false); // add to motion list

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);

    BREAK_WAIT_TIME[ENTRY_ID] = 120;

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    weapon.fastshift(L2CValue::Ptr(chariotsight_break_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn chariotsight_break_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);

    BREAK_WAIT_TIME[ENTRY_ID] -= 1;

    if BREAK_WAIT_TIME[ENTRY_ID] <= 0 {
        SHIELD_ON[ENTRY_ID] = false;
        chariotsight_remove(weapon);
        return 0.into();
    }
    
    return 0.into();
}

// END
unsafe extern "C" fn chariotsight_break_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn chariotsight_remove(weapon: &mut smashline::L2CWeaponCommon) {
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("pit_chariotsight")
        .status(Pre, *WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_FINAL, chariotsight_shield_pre)
        .status(Main, *WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_FINAL, chariotsight_shield_main)
        .status(End, *WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_FINAL, chariotsight_shield_end)

        .status(Pre, WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_BREAK, chariotsight_break_pre)
        .status(Main, WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_BREAK, chariotsight_break_main)
        .status(End, WEAPON_PIT_CHARIOTSIGHT_STATUS_KIND_BREAK, chariotsight_break_end)

        .install();
}