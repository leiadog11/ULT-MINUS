use super::*;

// OPFF
pub unsafe extern "C" fn peach_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let frame = MotionModule::frame(boma);
        let xpos = ControlModule::get_stick_x(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true); // COLLISION FIX
            SLEEP_MOVE[ENTRY_ID] = false;
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 {
            ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        // ON LEDGE
        if situation_kind == *SITUATION_KIND_CLIFF { 
            ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        // FORWARD AIR CHARGE
        if motion_kind == hash40("attack_air_f") { 
            if frame >= 6.0 && frame <= 13.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::set_rate(boma, 0.0);
                    if FORWARD_AIR_CHARGE[ENTRY_ID] < 20.0 {
                        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, true);
                        FORWARD_AIR_CHARGE[ENTRY_ID] += 1.0;
                    }
                    else if FORWARD_AIR_CHARGE[ENTRY_ID] == 20.0 {
                        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                        if !SoundModule::is_playing(boma, Hash40::new("se_gohoubi_bounus_add")) {
                            SoundModule::play_se(boma, Hash40::new("se_gohoubi_bounus_add"), true, false, false, false, enSEType(0));
                            FORWARD_AIR_CHARGE[ENTRY_ID] += 1.0;
                        }
                    }
                }
                else {
                    MotionModule::set_rate(boma, 1.0);
                }
            }
        }
        else {
            FORWARD_AIR_CHARGE[ENTRY_ID] = 0.0;
        }

        // CANCEL NAIR WITH SIDE B
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && (xpos > 0.5 || xpos < -0.5) && CAN_CANCEL_NAIR[ENTRY_ID] {
            CancelModule::enable_cancel(boma);
            CAN_CANCEL_NAIR[ENTRY_ID] = false;
        }

        // CANCEL SIDE B WITH AERIALS
        if motion_kind == smash::hash40("special_air_s_start") || motion_kind == smash::hash40("special_air_s_end") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                CancelModule::enable_cancel(boma);
            }
        }

        // TRANSITION TO FALL FROM SPECIAL FALL
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }

        // DANGER
        if situation_kind == *SITUATION_KIND_AIR {
            if STALL_TIMER[ENTRY_ID] == 720 {
                let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
                EffectModule::req_follow(boma, Hash40::new("sys_flies_up"), Hash40::new("top"), &dumb, &dumb, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                SoundModule::play_se(boma, Hash40::new("se_common_spirits_machstamp_landing"), true, false, false, false, enSEType(0));
                STALL_TIMER[ENTRY_ID] = 721;
            }
            else if STALL_TIMER[ENTRY_ID] == 721 {
                DamageModule::add_damage(boma, 0.5, 0);
                if DamageModule::damage(boma, 0) >= 200.0 {
                    STALL_TIMER[ENTRY_ID] = 0;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
                }
            }
            else {
                STALL_TIMER[ENTRY_ID] += 1;
            }
        }
        else {
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEMO {
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
        if DamageModule::reaction(boma, 0) > 1.0 { 
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
    }
}

// ON START
pub unsafe extern "C" fn peach_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        STALL_TIMER[ENTRY_ID] = 0;
        FORWARD_AIR_CHARGE[ENTRY_ID] = 0.0;
        SLEEP_MOVE[ENTRY_ID] = false;
        CAN_CANCEL_NAIR[ENTRY_ID] = false;
    }
}

pub fn install() {
    Agent::new("peach")
        .on_line(Main, peach_frame)
        .on_start(peach_start)
        .install();
}