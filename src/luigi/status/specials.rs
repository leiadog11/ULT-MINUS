use super::*;

///////// SIDE B

/*

// CHARGE - PRE
unsafe extern "C" fn luigi_specials_charge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_CHARGE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_CHARGE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_CHARGE_FLOAT,
        0
      );
      
      FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );
      
      return 0.into();
}

// CHARGE - MAIN
unsafe extern "C" fn luigi_specials_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    if !StopModule::is_stop(fighter.module_accessor) {
        luigi_specials_charge_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(luigi_specials_charge_substatus as *const () as _));
    luigi_specials_charge_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specials_charge_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_specials_charge_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_speed_mul"));
    WorkModule::add_float(fighter.module_accessor, charge_speed, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    0.into()
}

unsafe extern "C" fn luigi_specials_charge_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s_hold"),
                1.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_s_hold"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_hold"),
                1.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_s_hold"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
}

// CHARGE - MAIN LOOP
unsafe extern "C" fn luigi_specials_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
        let charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));

        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);

        //UP ANGLE
        if xpos == 0.0 && ypos > 0.0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -20.0, y: 0.0, z: 0.0}, 0);
        }
        //FORWARD ANGLE
        else if xpos > 0.0 && ypos == 0.0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
        }
        //DOWN ANGLE
        else if xpos == 0.0 && ypos < 0.0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 20.0, y: 0.0, z: 0.0}, 0);
        }



        if charge_frame <= charge {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH).into(), false.into());
            }
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            luigi_specials_charge_mot_helper(fighter);
        }
    }
    else {
        fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM).into(), false.into());
    }
    0.into()
}

// CHARGE - END
unsafe extern "C" fn luigi_specials_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CHARGE_MELEE_NO_RANDOM) != true {
        let misfire_chance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("discharge_prob"));
        let rand = smash::app::sv_math::rand(hash40("fighter"), 10) as u64;
        if rand != 9 {

        }
        else {
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE)
        }
    }
    
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
      
    return 0.into();
}
*/

/*

// RAM - INIT
unsafe extern "C" fn luigi_specials_ram_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::set_test_coll_stop_status(fighter.module_accessor, true);
    return 0.into();
}

// RAM - PRE
unsafe extern "C" fn luigi_specials_ram_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_FLOAT,
        0
      );
      
      FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );

    return 0.into();
}

// RAM - MAIN
unsafe extern "C" fn luigi_specials_ram_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_discharge"), 0.0, 1.0, false, 0.0, false, false);
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specials_ram_main_loop as *const () as _))
}

// RAM - MAIN LOOP
unsafe extern "C" fn luigi_specials_ram_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END).into(), false.into());
        
        return 1.into();
      }
    
      if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
          fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END).into(), false.into());
    
          return 1.into();
        }
      }
    
      if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND) {
        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
          fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END).into(), false.into());
    
          return 1.into();
        }
      }
    
      let touch = GroundModule::get_touch_flag(fighter.module_accessor);
      let lr = PostureModule::lr(fighter.module_accessor);
      
      if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK) {
    
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
    
        }
    
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER) {
          // get attack power
          // set attack power 0
        }
        else {
          if AttackModule::is_attack(fighter.module_accessor, 0, false) {
            let power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power"));
            let attack_charge = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power_charge"));
            let mut charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
            let mut charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
    
            if charge == charge_frame {
              charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE)
            }
            else {
              charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
            }
    
            charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
    
            AttackModule::set_power_mul_status(fighter.module_accessor, power * attack_charge);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);

            return 0.into();
          }
        }
        return 0.into();
      }
      else {
        let attach_side = if 0.0 <= lr {*GROUND_TOUCH_FLAG_LEFT} else { *GROUND_TOUCH_FLAG_RIGHT };
    
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
        if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_side)) {
          let wall_prob = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wall_prob"));
    
          let rand = smash::app::sv_math::rand(hash40("fighter"), 1) as u64;
        /*
          if rand != 0 {
    
          }
        */
          let wall_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wall_speed_x"));
          fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL).into(), false.into());
    
          return 0.into();
        }
        return 0.into();
    }
}

// RAM - END
unsafe extern "C" fn luigi_specials_ram_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_HIT);
    }
    
    return 0.into();
}

*/

pub fn install() {
    Agent::new("luigi")
        //.status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specials_charge_pre)
        //.status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specials_charge_main)
        //.status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specials_charge_end)
        //.status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_init)
        //.status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_pre)
        //.status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_main)
        //.status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_end)
        .install();
}