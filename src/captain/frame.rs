use super::*;

// OPFF
pub unsafe extern "C" fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { // COLLISION FIX
            GroundModule::set_collidable(boma, true);
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 { // CLEAR FIRE BIRD ON F SMASH DAMAGE
            ArticleModule::remove_exist(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        // ON GROUND
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { // GROUND CHECK FOR UP B COUNT
            UP_B_AMOUNT[ENTRY_ID] = 2;
        } 

        // CLEAR ZOOM IN UP TILT
        if motion_kind == hash40("attack_hi3") {
            if MotionModule::is_end(boma) || DamageModule::reaction(boma, 0) > 1.0 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }

        // CLEAR ZOOM IN RAPID JAB
        if motion_kind == hash40("attack_100") {
            if MotionModule::is_end(boma) || DamageModule::reaction(boma, 0) > 1.0 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }
        if motion_kind == hash40("attack_100_end") {
            SlowModule::clear_whole(boma);
            CameraModule::reset_all(boma);
            EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // CLEAR ZOOM IN FORWARD AIR
        if motion_kind == hash40("attack_air_f") { 
            if frame >= 40.0 || DamageModule::reaction(boma, 0) > 1.0 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_boss_finishhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }
        if motion_kind == hash40("landing_air_f") {
            SlowModule::clear_whole(boma);
            CameraModule::reset_all(boma);
            EffectModule::kill_kind(boma, Hash40::new("sys_bg_boss_finishhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }
        if status_kind == *FIGHTER_STATUS_KIND_WAIT || status_kind == *FIGHTER_STATUS_KIND_FALL {
            SlowModule::clear_whole(boma);
            CameraModule::reset_all(boma);
            EffectModule::kill_kind(boma, Hash40::new("sys_bg_boss_finishhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // GUN COOLDOWN
        if GUN_COOLDOWN[ENTRY_ID] > 0 {
            GUN_COOLDOWN[ENTRY_ID] -= 1;
        }
    }
}

// ON START
pub unsafe extern "C" fn captain_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        KICK[ENTRY_ID] = false;
        GUN_COOLDOWN[ENTRY_ID] = 0;
        UP_B_AMOUNT[ENTRY_ID] = 2;
    }
}

pub fn install() {
    Agent::new("captain")
        .on_line(Main, captain_frame)
        .on_start(captain_start)
        .install();
}