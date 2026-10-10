use super::*;

static mut UP_AMOUNT: [f32; 8] = [0.0; 8];
static mut FORWARD_AMOUNT: [f32; 8] = [0.0; 8];

//------------------ SPECIAL HI --------------------

// PRE
unsafe extern "C" fn ridley_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn ridley_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_specialhi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ridley_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let ENTRY_ID = get_entry_id(fighter.module_accessor);

    fighter.sub_transition_group_check_air_cliff();

    // UP
    if stick_x == 0.0 && stick_y > 0.3 {
        UP_AMOUNT[ENTRY_ID] = 2.0;
        FORWARD_AMOUNT[ENTRY_ID] = 0.0;
        //PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -90.0, y: 0.0, z: 0.0}, 0);
    }

    // RIGHT
    if stick_x > 0.5 && stick_y == 0.0 { 
        UP_AMOUNT[ENTRY_ID] = 0.0;
        FORWARD_AMOUNT[ENTRY_ID] = 2.0;
    }

    // UP RIGHT
    if stick_x > 0.3 && stick_y > 0.3 { 
        UP_AMOUNT[ENTRY_ID] = 1.5;
        FORWARD_AMOUNT[ENTRY_ID] = 1.5;
        //PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -45.0, y: 0.0, z: 0.0}, 0);
    }

    // UP LEFT
    if stick_x < -0.3 && stick_y > 0.3 { 
        UP_AMOUNT[ENTRY_ID] = 1.5;
        FORWARD_AMOUNT[ENTRY_ID] = -1.5;
        //PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -45.0, y: 0.0, z: 0.0}, 0);
    }

    // LEFT
    if stick_x < -0.5 && stick_y == 0.0 { 
        UP_AMOUNT[ENTRY_ID] = 0.0;
        FORWARD_AMOUNT[ENTRY_ID] = -2.0;
    }

    // DOWN LEFT
    if stick_x < -0.3 && stick_y < -0.3 { 
        UP_AMOUNT[ENTRY_ID] = -1.5;
        FORWARD_AMOUNT[ENTRY_ID] = -1.5;
        //PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 45.0, y: 0.0, z: 0.0}, 0);
    }

    // DOWN
    if stick_x == 0.0 && stick_y < -0.5 { 
        UP_AMOUNT[ENTRY_ID] = -2.0;
        FORWARD_AMOUNT[ENTRY_ID] = 0.0;
        //PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0);
    }

    // DOWN RIGHT
    if stick_x > 0.3 && stick_y < -0.3 { 
        UP_AMOUNT[ENTRY_ID] = -1.5;
        FORWARD_AMOUNT[ENTRY_ID] = 1.5;
        //PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 45.0, y: 0.0, z: 0.0}, 0);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_START.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn ridley_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//------------------ SPECIAL HI CHARGE --------------------

// PRE
unsafe extern "C" fn ridley_specialhi_charge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
unsafe extern "C" fn ridley_specialhi_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_charge"), 0.0, 1.0, false, 0.0, false, false);

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_specialhi_charge_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ridley_specialhi_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let ENTRY_ID = get_entry_id(fighter.module_accessor);

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        FORWARD_AMOUNT[ENTRY_ID],
        0.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        UP_AMOUNT[ENTRY_ID],
        0.0
    );

    // LANDING
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
        
    return 0.into();
}

// END
unsafe extern "C" fn ridley_specialhi_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    return 0.into();
}

pub fn install() {
    Agent::new("ridley")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_end)

        .status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_START, ridley_specialhi_charge_pre)
        .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_START, ridley_specialhi_charge_main)
        .status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_START, ridley_specialhi_charge_end)
        
        .install();
}