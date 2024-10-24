use super::*;

// SHOT
unsafe extern "C" fn autoaimbullet_shot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        
    }
}

// SHOT EFFECT
unsafe extern "C" fn autoaimbullet_effect_shot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("palutena_bullet_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

// SHOT SOUND
unsafe extern "C" fn autoaimbullet_sound_shot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_n02"));
    }
}

pub fn install() {
    Agent::new("palutena_autoaimbullet")
        .game_acmd("game_shot", autoaimbullet_shot, Low)
        .effect_acmd("effect_shot", autoaimbullet_effect_shot, Low)
        .sound_acmd("sound_shot", autoaimbullet_sound_shot, Low)
        .install();
    
}