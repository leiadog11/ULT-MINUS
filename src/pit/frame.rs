use super::*;

// OPFF
pub unsafe extern "C" fn pit_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
        }

        // FLIGHT
        if IS_FLIGHT[ENTRY_ID] {
            KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            macros::SET_SPEED_EX(fighter, 0.0 + xpos, 0.0 + ypos, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle_b"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 { // CLEAR FLIGHT
            IS_FLIGHT[ENTRY_ID] = false;
            KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
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
pub unsafe extern "C" fn pit_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        STALL_TIMER[ENTRY_ID] = 0;
        IS_FLIGHT[ENTRY_ID] = false;
    }
}

pub fn install() {
    Agent::new("pit")
        .on_line(Main, pit_frame)
        .on_start(pit_start)
        .install();
}