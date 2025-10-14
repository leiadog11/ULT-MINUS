use super::*;

static mut PUFF_X: [f32; 8] = [0.0; 8];
static mut PUFF_Y: [f32; 8] = [0.0; 8];
static mut PUFF_Z: [f32; 8] = [0.0; 8];

static mut OPP_X: [f32; 8] = [0.0; 8];
static mut OPP_Y: [f32; 8] = [0.0; 8];
static mut OPP_Z: [f32; 8] = [0.0; 8];

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

  let ENTRY_ID = get_entry_id(fighter.module_accessor);
  METRONOME[ENTRY_ID] = smash::app::sv_math::rand(hash40("fighter"), 7) as u64;

  let opponent_bomas = get_opponent_bomas(fighter.module_accessor);

  // GET PUFF POS
  PUFF_X[ENTRY_ID] = PostureModule::pos_x(fighter.module_accessor);
  PUFF_Y[ENTRY_ID]= PostureModule::pos_y(fighter.module_accessor);
  PUFF_Z[ENTRY_ID] = PostureModule::pos_z(fighter.module_accessor);

  // GET OPP POS
  OPP_X[ENTRY_ID] = PostureModule::pos_x(opponent_bomas[0]);
  OPP_Y[ENTRY_ID] = PostureModule::pos_y(opponent_bomas[0]);
  OPP_Z[ENTRY_ID] = PostureModule::pos_z(opponent_bomas[0]);

  fighter.sub_shift_status_main(L2CValue::Ptr(purin_specialn_main_loop as *const () as _))
}

// SPECIAL N - MAIN LOOP
unsafe extern "C" fn purin_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
  let ENTRY_ID = get_entry_id(fighter.module_accessor);

  // METRONOME DECISION
  if MotionModule::frame(fighter.module_accessor) >= 28.0 && MotionModule::frame(fighter.module_accessor) < 29.0 {
    // DRILL
    if METRONOME[ENTRY_ID] == 0 {
      ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_DRILL), 0, 0, false, false);
    }
    // FREEZER
    if METRONOME[ENTRY_ID] == 1 {
      ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_FREEZER), 0, 0, false, false);
    }
    // TURN TO METAL
    if METRONOME[ENTRY_ID] == 2 {
      ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_METALBLOCK), 0, 0, false, false);
    }
    // HEAL
    if METRONOME[ENTRY_ID] == 3 {
      DamageModule::add_damage(fighter.module_accessor, -8.0, 0);
      SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_lifeup"), true, false, false, false, enSEType(0));
      macros::EFFECT(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    // SLEEP
    if METRONOME[ENTRY_ID] == 4 {
      StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLEEP_START, false.into());
    }
    // SWAP PLACES
    if METRONOME[ENTRY_ID] == 5 {
      let opponent_bomas = get_opponent_bomas(fighter.module_accessor);

      // SET OPP POS
      PostureModule::set_pos(opponent_bomas[0], &Vector3f{ x: PUFF_X[ENTRY_ID], y: PUFF_Y[ENTRY_ID], z: PUFF_Z[ENTRY_ID]});
      GroundModule::set_collidable(opponent_bomas[0], false);

      // SET PUFF POS
      PostureModule::set_pos(fighter.module_accessor, &Vector3f{ x: OPP_X[ENTRY_ID], y: OPP_Y[ENTRY_ID], z: OPP_Z[ENTRY_ID]});
      GroundModule::set_collidable(fighter.module_accessor, false);
    }
    // ROLLOUT?
    if METRONOME[ENTRY_ID] == 6 {
      WorkModule::set_float(fighter.module_accessor, 100.0, *FIGHTER_PURIN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
      StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR, false.into());
    }
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
  let opponent_bomas = get_opponent_bomas(fighter.module_accessor);

  GroundModule::set_collidable(fighter.module_accessor, true);
  GroundModule::set_collidable(opponent_bomas[0], true);

  return 0.into();
}

pub fn install() {
    Agent::new("purin")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_end)
        .install();
}