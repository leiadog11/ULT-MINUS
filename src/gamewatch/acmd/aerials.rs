use super::*;

//----------------AERIALS------------------

//NEUTRAL AIR
unsafe extern "C" fn gamewatch_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        WorkModule::set_flag(agent.module_accessor, true, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_OCTOPUS);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OCTOPUS, false, -1);
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

//NEUTRAL AIR SOUND
unsafe extern "C" fn gamewatch_sound_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_final02"));
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave04_hi"));
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_final03"));
    }
}

//NEUTRAL AIR EXPRESSION
unsafe extern "C" fn gamewatch_expression_attackairn(agent: &mut L2CAgentBase) {

}

// UP AIR
unsafe extern "C" fn gamewatch_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 85, 93, 0, 23, 4.0, 6.9, 7.0, 0.0, Some(-1.5), Some(20.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 85, 88, 0, 18, 4.0, 6.9, 7.0, 0.0, Some(-1.5), Some(20.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATK_POWER(agent, 0, 13);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// UP AIR SOUND
unsafe extern "C" fn gamewatch_sound_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_mi"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_hi"));
    }
}

// UP AIR EXPRESSION
unsafe extern "C" fn gamewatch_expression_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("lhand") as i64, hash40("lhand_key") as i64);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_Y_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

// DOWN AIR
unsafe extern "C" fn gamewatch_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, false, -1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 41.0);
    macros::FT_MOTION_RATE(agent, 2.0);
}

// DOWN AIR SOUND
unsafe extern "C" fn gamewatch_sound_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_mi"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_mi"));
    }
}

// DOWN AIR EXPRESSION
unsafe extern "C" fn gamewatch_expression_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("head") as i64, hash40("head_sparky") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("hand") as i64, hash40("hand_hold_lr") as i64);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        //AREA_WIND_2ND_arg10(0, 1, 90, 300, 1, 0, 30, 32, 60, 50);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        //AREA_WIND_2ND_arg10(0, 2, 90, 300, 1, 0, 30, 32, 60, 50);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_attackairn", gamewatch_attackairn, Low)
        .sound_acmd("sound_attackairn", gamewatch_sound_attackairn, Low)
        .expression_acmd("expression_attackairn", gamewatch_expression_attackairn, Low)

        .game_acmd("game_attackairhi", gamewatch_attackairhi, Low)
        .sound_acmd("sound_attackairhi", gamewatch_sound_attackairhi, Low)
        .expression_acmd("expression_attackairhi", gamewatch_expression_attackairhi, Low)

        .game_acmd("game_attackairlw", gamewatch_attackairlw, Low)
        .sound_acmd("sound_attackairlw", gamewatch_sound_attackairlw, Low)
        .expression_acmd("expression_attackairlw", gamewatch_expression_attackairlw, Low)

        .install();
}