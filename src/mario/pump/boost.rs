use super::*;

// --------------- BOOST ---------------

// PRE
unsafe extern "C" fn mario_pump_boost_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *WEAPON_KINETIC_TYPE_NONE, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn mario_pump_boost_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("boost"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(mario_pump_boost_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn mario_pump_boost_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 

    // SHOOT WATER
    if MotionModule::frame(weapon.module_accessor) >= 8.0 && MotionModule::frame(weapon.module_accessor) <= 28.0 { 
        ArticleModule::generate_article(weapon.module_accessor, *WEAPON_MARIO_PUMP_GENERATE_ARTICLE_WATER, false, -1);
        ArticleModule::change_status(weapon.module_accessor, *WEAPON_MARIO_PUMP_GENERATE_ARTICLE_WATER, WEAPON_MARIO_PUMP_WATER_STATUS_KIND_BOOST, ArticleOperationTarget(0));
    }
    return 0.into();
}

// END
unsafe extern "C" fn mario_pump_boost_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("mario_pump")
        .status(Pre, WEAPON_MARIO_PUMP_STATUS_KIND_BOOST, mario_pump_boost_pre)
        .status(Main, WEAPON_MARIO_PUMP_STATUS_KIND_BOOST, mario_pump_boost_main)
        .status(End, WEAPON_MARIO_PUMP_STATUS_KIND_BOOST, mario_pump_boost_end)
        .install();
}