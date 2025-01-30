use super::*;

//-------------------SPECIALS------------------------

// PRE
unsafe extern "C" fn palutena_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        0, 
        0, 
        0, 
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// INIT
unsafe extern "C" fn palutena_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if BLACKHOLE_CHARGE[get_entry_id(fighter.module_accessor)] >= 360 {
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

// MAIN
unsafe extern "C" fn palutena_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_specials_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn palutena_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_CHARGE.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

// END
unsafe extern "C" fn palutena_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//-------------------CHARGE------------------------

// PRE
unsafe extern "C" fn palutena_specials_charge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        0, 
        0, 
        0, 
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn palutena_specials_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_charge"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_specials_charge_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn palutena_specials_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    BLACKHOLE_CHARGE[ENTRY_ID] += 1;

    // SHIELD CANCEL
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
            return 1.into();
        }
    }

    // JUMP CANCEL
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    // CANCEL WITH B
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    // HALF CHARGE EFFECT
    if BLACKHOLE_CHARGE[ENTRY_ID] >= 120 && BLACKHOLE_CHARGE[ENTRY_ID] <= 121  { 
        let dumb = Vector3f{x:0.0,y:5.0,z:0.0};
        let eff = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("hip"), &dumb, &dumb, 0.5, &dumb, &dumb, false, 0, 0, 0);
        EffectModule::set_rgb(fighter.module_accessor, eff.try_into().unwrap(), 0.5, 0.0, 0.5);
    }

    // FINISH
    if BLACKHOLE_CHARGE[ENTRY_ID] >= 360 {
        let mut dumb = Vector3f{x:0.0,y:10.0,z:0.0};
        let eff = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("hip"), &dumb, &dumb, 1.2, &dumb, &dumb, false, 0, 0, 0);
        EffectModule::set_rgb(fighter.module_accessor, eff.try_into().unwrap(), 0.5, 0.0, 0.5);
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_item_deathscythe_swing_l"), true, false, false, false, enSEType(0));
        dumb = Vector3f{x:0.0,y:8.65,z:1.0};
        let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), &dumb, &dumb, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(fighter.module_accessor, effect, 0.5, 0.0, 0.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn palutena_specials_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//-------------------SHOOT------------------------

// PRE
unsafe extern "C" fn palutena_specials_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        (*GROUND_CORRECT_KIND_NONE).try_into().unwrap(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | 
        *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        ((*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE |
        *FIGHTER_STATUS_ATTR_FINAL)).try_into().unwrap(),
        *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32,
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn palutena_specials_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_shoot"), 0.0, 1.0, false, 0.0, false, false);
    BLACKHOLE_CHARGE[get_entry_id(fighter.module_accessor)] = 0;
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8}, -1.0, -1.0, -1);
    AreaModule::set_whole(fighter.module_accessor, false);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("palutena_wand_light_trace"), false, true);

    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_specials_shoot_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn palutena_specials_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    CameraModule::req_quake(fighter.module_accessor, *CAMERA_QUAKE_KIND_SMALL);
    if MotionModule::is_end(fighter.module_accessor) { 
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    return 0.into();
}

// END
unsafe extern "C" fn palutena_specials_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    return 0.into();
}

pub fn install() {
    Agent::new("palutena")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_end)

        .status(Pre, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_CHARGE, palutena_specials_charge_pre)
        .status(Main, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_CHARGE, palutena_specials_charge_main)
        .status(End, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_CHARGE, palutena_specials_charge_end)

        .status(Pre, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_SHOOT, palutena_specials_shoot_pre)
        .status(Main, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_SHOOT, palutena_specials_shoot_main)
        .status(End, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_SHOOT, palutena_specials_shoot_end)

        .install();
}