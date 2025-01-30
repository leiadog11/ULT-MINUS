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

        // NO SPECIAL FALL ON UP B + EXTRA JUMP
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE) {
            if frame == 20.0 {
                let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                WorkModule::set_int(boma, jump_count + 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                UP_B_USED[ENTRY_ID] = true;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }

        // GET UP B BACK
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF || DamageModule::reaction(boma, 0) > 1.0 { 
            UP_B_USED[ENTRY_ID] = false;
        } 

        // CLEAR WINGS AND CHARGE MULT ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 {
            ArticleModule::remove_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            CHARGE_MUL[ENTRY_ID] = 1.0;
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

        // TP TO ANCHOR WITH AERIAL DOWN B 
        if ANCHOR_PLANTED[ENTRY_ID] {  
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && situation_kind == *SITUATION_KIND_AIR { 
                KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            }

            if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 && situation_kind == *SITUATION_KIND_AIR {
                if frame >= 1.0 { 
                    GroundModule::set_collidable(boma, false);
                    PostureModule::set_pos(boma, &Vector3f{ x: BULLET_X_POS[ENTRY_ID] , y: BULLET_Y_POS[ENTRY_ID], z: PostureModule::pos_z(boma)});
                }
                if frame >= 2.0 { 
                    GroundModule::set_collidable(boma, true);
                    ANCHOR_PLANTED[ENTRY_ID] = false;
                }
            }
        }

        // AND MEGA LASER
        if motion_kind == hash40("special_s_shoot") && MEGA_LASER_CHARGE[ENTRY_ID] >= 360 && frame >= 25.0 {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_SHOOT, false);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn palutena_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("palutena")
        .on_line(Main, palutena_frame)
        .on_start(palutena_start)
        .install();
}