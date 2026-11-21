use super::*;

static mut OPP_X: [f32; 8] = [0.0; 8];
static mut OPP_Y: [f32; 8] = [0.0; 8];
static mut OPP_Z: [f32; 8] = [0.0; 8];
static mut END_WAIT_AMOUNT: [i32; 8] = [0; 8];

//----------------- SPECIAL HI -----------------

// PRE
unsafe extern "C" fn bayonetta_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
      
    return 0.into();
}

// INIT
unsafe extern "C" fn bayonetta_special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let opponent_bomas = get_opponent_bomas(fighter.module_accessor);

    // SAVE OPP POS
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    OPP_X[ENTRY_ID] = PostureModule::pos_x(opponent_bomas[0]);
    OPP_Y[ENTRY_ID] = PostureModule::pos_y(opponent_bomas[0]) + 10.0;
    OPP_Z[ENTRY_ID] = PostureModule::pos_z(opponent_bomas[0]);

    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);

    let opp_x = OPP_X[ENTRY_ID];
    let opp_y = OPP_Y[ENTRY_ID];
    let opp_z = OPP_Z[ENTRY_ID];

    let dx = opp_x - pos_x;
    let dy = opp_y - pos_y;
    let dz = opp_z - pos_z;
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();

    if dist <= 10.0 {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

// MAIN
unsafe extern "C" fn bayonetta_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_hi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bayonetta_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_BAT.into(), false.into());
    return 1.into();
}

// END
unsafe extern "C" fn bayonetta_special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//----------------- SPECIAL HI BAT -----------------

// PRE
unsafe extern "C" fn bayonetta_special_hi_bat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn bayonetta_special_hi_bat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_bat"), 0.0, 1.0, false, 0.0, false, false);

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    GroundModule::set_collidable(fighter.module_accessor, false);
    VisibilityModule::set_whole(fighter.module_accessor, false);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_hi_bat_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bayonetta_special_hi_bat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let ENTRY_ID = get_entry_id(fighter.module_accessor);

    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);

    let opp_x = OPP_X[ENTRY_ID];
    let opp_y = OPP_Y[ENTRY_ID];
    let opp_z = OPP_Z[ENTRY_ID];

    let speed: f32 = 2.5;
    let dx = opp_x - pos_x;
    let dy = opp_y - pos_y;
    let dz = opp_z - pos_z;
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();

    let (new_x, new_y, new_z) = if dist > 10.0 {
        let step = speed.min(dist);
        let scale = step / dist;
        (pos_x + dx * scale, pos_y + dy * scale, pos_z + dz * scale)
    } else {
        (opp_x, opp_y, opp_z)
    };

    let new_pos = Vector3f{x: new_x, y: new_y, z: new_z};
    PostureModule::set_pos(fighter.module_accessor, &new_pos);

    if dist <= 10.0 {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_BAT) { 
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_BAT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn bayonetta_special_hi_bat_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//----------------- SPECIAL HI END -----------------

// PRE
unsafe extern "C" fn bayonetta_special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn bayonetta_special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_collidable(fighter.module_accessor, true);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    notify_event_msc_cmd!(fighter, Hash40::new("fighter_msc_cmd_self_set_cursor"), true);
    notify_event_msc_cmd!(fighter, Hash40::new("fighter_msc_cmd_self_set_name_cursor"), true);

    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    END_WAIT_AMOUNT[ENTRY_ID] = 5;

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_hi_end_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bayonetta_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    END_WAIT_AMOUNT[ENTRY_ID] -= 1;

    if END_WAIT_AMOUNT[ENTRY_ID] <= 0 {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
        } 
    }

    return 0.into();
}

// END
unsafe extern "C" fn bayonetta_special_hi_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("bayonetta")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_special_hi_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_special_hi_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_special_hi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_special_hi_end)

        .status(Pre, FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_BAT, bayonetta_special_hi_bat_pre)
        .status(Main, FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_BAT, bayonetta_special_hi_bat_main)
        .status(End, FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_BAT, bayonetta_special_hi_bat_end)

        .status(Pre, FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_END, bayonetta_special_hi_end_pre)
        .status(Main, FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_END, bayonetta_special_hi_end_main)
        .status(End, FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_END, bayonetta_special_hi_end_end)
    .install();
}