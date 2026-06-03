use super::*;

// OPFF
pub unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let frame = MotionModule::frame(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            UP_B_USED[ENTRY_ID] = false;
            
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_mball_flash"), false, true);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("palutena_wand_light_trace"), false, true);

            if MEGA_LASER_CHARGE[ENTRY_ID] >= 360 { 
                let vector = Vector3f{x:0.0,y:5.0,z:0.0};
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_mball_flash"), Hash40::new("stick"), &vector, &vector, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            }
            if BLACKHOLE_CHARGE[ENTRY_ID] >= 360 { 
                let vector = Vector3f{x:0.0,y:10.0,z:0.0};
                let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), &vector, &vector, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(fighter.module_accessor, effect, 0.5, 0.0, 0.5);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            }
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 {
            ArticleModule::remove_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)); // CLEAR WINGS
            CHARGE_MUL[ENTRY_ID] = 1.0; // RESET CHARGE MULT
            UP_B_USED[ENTRY_ID] = false; // GET UP B BACK
        }
        
        // ON GROUND
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            UP_B_USED[ENTRY_ID] = false; // GET UP B BACK
        } 

        // NO SPECIAL FALL ON UP B + EXTRA JUMP
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            UP_B_USED[ENTRY_ID] = true;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }

        // CHARGE DOWN SMASH MULT
        if motion_kind == hash40("attack_lw4_hold") {
            if CHARGE_MUL[ENTRY_ID] < 4.0 {
                CHARGE_MUL[ENTRY_ID] += 0.075;
            }
        }

        // STOP CHARGE SOUND!
        if motion_kind != hash40("special_n_charge") && motion_kind != hash40("special_s_charge") && motion_kind != hash40("attack_hi4") {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_attack100"));
            macros::STOP_SE(fighter, Hash40::new("se_palutena_special_n01"));
            EffectModule::kill_kind(boma, Hash40::new("palutena_wand_light2"), false, true);
        }

        if motion_kind != hash40("special_n_shoot") {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_final03"));
        }

        // AND MEGA LASER
        if motion_kind == hash40("special_s_shoot") && MEGA_LASER_CHARGE[ENTRY_ID] >= 360 && frame >= 25.0 {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_SHOOT, false);
            }
        }

        // DISABLE UP B TRANSITION 
        if UP_B_USED[ENTRY_ID] {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        } else {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        }
    }
}

// ON START
pub unsafe extern "C" fn palutena_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        ANCHOR_PLANTED[ENTRY_ID] = false; 
        UP_B_USED[ENTRY_ID] = false;
        MEGA_LASER_CHARGE[ENTRY_ID] = 0;
        BLACKHOLE_CHARGE[ENTRY_ID] = 0;
        CHARGE_MUL[ENTRY_ID] = 1.0;
        BULLET_X_POS[ENTRY_ID] = 0.0;
        BULLET_Y_POS[ENTRY_ID] = 0.0;
    }
}

pub fn install() {
    Agent::new("palutena")
        .on_line(Main, palutena_frame)
        .on_start(palutena_start)
        .install();
}