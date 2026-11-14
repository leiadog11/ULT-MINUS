use super::*;

static mut OPP_X: [f32; 8] = [0.0; 8];
static mut OPP_Y: [f32; 8] = [0.0; 8];
static mut OPP_Z: [f32; 8] = [0.0; 8];

//----------------- SPECIAL HI -----------------

// PRE
unsafe extern "C" fn bayonetta_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe extern "C" fn bayonetta_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    GroundModule::set_collidable(fighter.module_accessor, false);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    let opponent_bomas = get_opponent_bomas(fighter.module_accessor);

    // SAVE OPP POS
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    OPP_X[ENTRY_ID] = PostureModule::pos_x(opponent_bomas[0]);
    OPP_Y[ENTRY_ID] = PostureModule::pos_y(opponent_bomas[0]) + 20.0;
    OPP_Z[ENTRY_ID] = PostureModule::pos_z(opponent_bomas[0]);

    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_specialhi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bayonetta_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let ENTRY_ID = get_entry_id(fighter.module_accessor);

    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);

    let opp_x = OPP_X[ENTRY_ID];
    let opp_y = OPP_Y[ENTRY_ID];
    let opp_z = OPP_Z[ENTRY_ID];

    let lerp_factor = 0.06;

    let new_x = pos_x + (opp_x - pos_x) * lerp_factor;
    let new_y = pos_y + (opp_y - pos_y) * lerp_factor;
    let new_z = pos_z + (opp_z - pos_z) * lerp_factor;

    let new_pos = Vector3f{x: new_x, y: new_y, z: new_z};
    PostureModule::set_pos(fighter.module_accessor, &new_pos);

    let dSquared: f32 = (new_x - opp_x) * (new_x - opp_x) + (new_y - opp_y) * (new_y - opp_y);
    let d = dSquared.sqrt();

    if d < 15.0 {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_BAT) { 
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_BAT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

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
unsafe extern "C" fn bayonetta_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_collidable(fighter.module_accessor, true);
    notify_event_msc_cmd!(fighter, Hash40::new("fighter_msc_cmd_self_set_cursor"), true);
    notify_event_msc_cmd!(fighter, Hash40::new("fighter_msc_cmd_self_set_name_cursor"), true);

    return 0.into();
}

pub fn install() {
    Agent::new("bayonetta")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_specialhi_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_specialhi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, bayonetta_specialhi_end)
    .install();
}