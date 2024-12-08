use super::*;

// SELF DESTRUCT
unsafe extern "C" fn pichu_selfdestruct(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 160, 0, 85, 35.0, 0.0, 4.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_BLOWN_UP);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// SELF DESTRUCT EFFECT
unsafe extern "C" fn pichu_effect_selfdestruct(agent: &mut L2CAgentBase) { 
    let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
    let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_bomb_a"), Hash40::new("top"), &dumb, &dumb, 2.0, &dumb, &dumb, false, 0, 0, 0);
}

// SELF DESTRUCT SOUND
unsafe extern "C" fn pichu_sound_selfdestruct(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_bomb_ll"), true, false, false, false, enSEType(0));
    }
}

// SELF DESTRUCT EXPRESSION
unsafe extern "C" fn pichu_expression_selfdestruct(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_RAD(agent, 0, 1, 0.02, 1000, 1, 0, 0, 30);
    }
}

//--------------------TAUNTS------------------------

// DOWN TAUNT
unsafe extern "C" fn pichu_appeallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, -1.2);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, -1.2);
    }
}

// UP TAUNT
unsafe extern "C" fn pichu_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::FT_ADD_DAMAGE(agent, 0.3);
        }
        wait(agent.lua_state_agent, 13.0);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_BIND, true);
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_selfdestruct", pichu_selfdestruct, Low)
        .effect_acmd("effect_selfdestruct", pichu_effect_selfdestruct, Low)
        .sound_acmd("sound_selfdestruct", pichu_sound_selfdestruct, Low)
        .expression_acmd("expression_selfdestruct", pichu_expression_selfdestruct, Low)

        .game_acmd("game_appealhir", pichu_appealhi, Low)
        .game_acmd("game_appealhil", pichu_appealhi, Low)

        .game_acmd("game_appeallwr", pichu_appeallw, Low)
        .game_acmd("game_appeallwl", pichu_appeallw, Low)
        
        .install();
}