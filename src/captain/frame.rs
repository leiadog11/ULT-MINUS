use super::*;

// OPFF
pub unsafe extern "C" fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        // CLEAR ZOOM IN UP TILT
        if motion_kind == hash40("attack_hi3") {
            if MotionModule::is_end(fighter.module_accessor) || DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }

        // CLEAR ZOOM IN RAPID JAB
        if motion_kind == hash40("attack_100") {
            if MotionModule::is_end(fighter.module_accessor) || DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }
        if motion_kind == hash40("attack_100_end") {
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // CLEAR ZOOM IN FORWARD AIR
        if motion_kind == hash40("attack_air_f") { 
            if frame >= 40.0 || DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }
        if motion_kind == hash40("landing_air_f") {
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }
        if status_kind == *FIGHTER_STATUS_KIND_WAIT || status_kind == *FIGHTER_STATUS_KIND_FALL {
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // CLEAR FIRE BIRD ON F SMASH DAMAGE
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        // GUN COOLDOWN
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN);
        }

        // GROUND CHECK FOR UP B COUNT
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT);
        } 
    }
}

// ON START
pub unsafe extern "C" fn captain_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN);
        WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_KICK);
    }
}

pub fn install() {
    Agent::new("captain")
        .on_line(Main, captain_frame)
        .on_start(captain_start)
        .install();
}