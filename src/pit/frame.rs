use super::*;

// OPFF
pub unsafe extern "C" fn pit_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma);
    }

    // DANGER
    if situation_kind == *SITUATION_KIND_AIR {
        if STALL_TIMER[ENTRY_ID] == 900 {
            let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
            EffectModule::req_follow(boma, Hash40::new("sys_flies_up"), Hash40::new("top"), &dumb, &dumb, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            SoundModule::play_se(boma, Hash40::new("se_common_spirits_machstamp_landing"), true, false, false, false, enSEType(0));
            STALL_TIMER[ENTRY_ID] = 901;
        }
        else if STALL_TIMER[ENTRY_ID] == 901 {
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
    }
    if DamageModule::reaction(boma, 0) > 1.0 { 
        STALL_TIMER[ENTRY_ID] = 0;
    }
}

// ON START
pub unsafe extern "C" fn pit_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        STALL_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("pit")
        .on_line(Main, pit_frame)
        .on_start(pit_start)
        .install();
}