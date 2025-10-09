use super::*;

// OPFF
pub unsafe extern "C" fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let pos_x = PostureModule::pos_x(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let lr = PostureModule::lr(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
            SHRUNK[ENTRY_ID] = false;
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 { 
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("block"), false);
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        // SHIELD CANCEL FORWARD SMASH CHARGE
        if motion_kind == hash40("attack_s4_hold") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
        
        // SHRINK
        if DamageModule::damage(boma, 0) > 100.0 && DamageModule::reaction(boma, 0) > 42.0 {
            if !SHRUNK[ENTRY_ID] {
                StatusModule::change_status_request_from_script(boma, FIGHTER_MARIO_STATUS_KIND_SHRINK, true);
                SHRUNK[ENTRY_ID] = true;
            }
        }   

        // DASH CANCEL FIREBALL ON THE GROUND
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && situation_kind == *SITUATION_KIND_GROUND {
            if frame > 14.0 {
                if xpos > 0.5 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
                    if lr == -1.0 {
                        macros::REVERSE_LR(fighter);
                    }
                }
                else if xpos < -0.5 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
                    if lr == 1.0 {
                        macros::REVERSE_LR(fighter);
                    }
                }
            }
        }

        // CANCEL DASH ATTACK INTO DASH ATTACK 2
        if motion_kind == hash40("attack_dash") && frame > 5.0 { 
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion(boma, Hash40::new("breakdance"), 0.0, 1.0, false, 0.0, false, false);
            }
        }

        // FORWARD SMASH CHARGE
        if motion_kind == hash40("attack_s4_hold") || motion_kind == hash40("attack_s4_s") || motion_kind == hash40("attack_s4_hi") || motion_kind == hash40("attack_s4_lw") {
            if FORWARD_SMASH_CHARGE[ENTRY_ID] < 60.0 {
                FORWARD_SMASH_CHARGE[ENTRY_ID] += 1.0;
            }
        }
        else {
            FORWARD_SMASH_CHARGE[ENTRY_ID] = 0.0;
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

        // MOVE DURING DOWN SMASH
        if motion_kind == hash40("attack_lw4") {
            if frame < 25.0 {
                //RIGHT
                if xpos > 0.0  {
                    PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x + 0.75, y: PostureModule::pos_y(boma)});
                }
                //LEFT
                if xpos < 0.0  {
                    PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x - 0.75, y: PostureModule::pos_y(boma)});
                }
            }
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
pub unsafe extern "C" fn mario_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("block"), false);
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        SHRUNK[ENTRY_ID] = false;
        ICEBALL[ENTRY_ID] = false;
        FORWARD_SMASH_CHARGE[ENTRY_ID] = 0.0;
        FORWARD_AIR_CHARGE[ENTRY_ID] = 0.0;
        STALL_TIMER[ENTRY_ID] = 0;
        COIN_COUNT[ENTRY_ID] = 0;
        SHRINK_TIME[ENTRY_ID] = 45;
    }
}

pub fn install() {
    Agent::new("mario")
        .on_line(Main, mario_frame)
        .on_start(mario_start)
        .install();
}