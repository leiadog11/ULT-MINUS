use super::*;

// SPECIAL N - PRE
unsafe extern "C" fn purin_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
  StatusModule::init_settings(
    fighter.module_accessor,
    smash::app::SituationKind(*SITUATION_KIND_NONE),
    *FIGHTER_KINETIC_TYPE_UNIQ,
    (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
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
    (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
    (*FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE).try_into().unwrap(),
    (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N).try_into().unwrap(),
    0
  );
      
  return 0.into();
}

// SPECIAL N - MAIN
unsafe extern "C" fn purin_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);

  fighter.sub_shift_status_main(L2CValue::Ptr(purin_specialn_main_loop as *const () as _))
}

// SPECIAL N - MAIN LOOP
unsafe extern "C" fn purin_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
  if MotionModule::frame(fighter.module_accessor) >= 28.0 && MotionModule::frame(fighter.module_accessor) < 29.0 {
    let rand = smash::app::sv_math::rand(hash40("fighter"), 4) as u64;
    // DRILL
    if rand == 0 {
      ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_DRILL), 0, 0, false, false);
    }
    // FREEZER
    else if rand == 1 {
      ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_FREEZER), 0, 0, false, false);
    }
    // TURN TO METAL
    else if rand == 2 {
      ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_METALBLOCK), 0, 0, false, false);
    }
    // HEAL
    else if rand == 3 {
      DamageModule::add_damage(fighter.module_accessor, -8.0, 0);
      SoundModule::play_se(fighter.module_accessor, Hash40::new("se_item_teamhealfield_recover"), true, false, false, false, enSEType(0));
      macros::EFFECT(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    else { }
  }

  if MotionModule::is_end(fighter.module_accessor) {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
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

// SPECIAL N - END
unsafe extern "C" fn purin_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
  return 0.into();
}

pub fn install() {
    Agent::new("purin")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_end)
        .install();
}