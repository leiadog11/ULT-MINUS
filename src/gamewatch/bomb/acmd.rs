use super::*;

unsafe extern "C" fn gamewatch_burst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 45, 125, 0, 40, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        //AREA_WIND_2ND_RAD(0, 1, 0.02, 1000, 1, 0, 0, 25);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    Agent::new("gamewatch_bomb")
    .game_acmd("game_burst", gamewatch_burst, Low)
    .install();
}