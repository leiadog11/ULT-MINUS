use super::*;

//----------------AERIALS------------------

// NEUTRAL AIR
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

// NEUTRAL AIR SOUND
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

// NEUTRAL AIR EXPRESSION
unsafe extern "C" fn gamewatch_expression_attackairn(agent: &mut L2CAgentBase) {

}

// UP AIR
unsafe extern "C" fn gamewatch_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 85, 103, 0, 47, 4.0, 6.9, 7.0, 0.0, Some(-1.5), Some(20.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 85, 88, 0, 18, 4.0, 6.9, 7.0, 0.0, Some(-1.5), Some(20.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
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

// BACK AIR
unsafe extern "C" fn gamewatch_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, false, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// BACK AIR SOUND
unsafe extern "C" fn gamewatch_sound_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_mi"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_hi"));
    }
}

// BACK AIR EXPRESSION
unsafe extern "C" fn gamewatch_expression_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("havel"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 7);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 7);
    }
}

// BACK AIR LANDING
unsafe extern "C" fn gamewatch_landingairb(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

// FORWARD AIR
unsafe extern "C" fn gamewatch_attackairf(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_f2"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, false, -1);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 50, 30, 0, 2.0, 0.0, 9.0, 11.3, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 5.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// FORWARD AIR 2
unsafe extern "C" fn gamewatch_attackairf2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 7.0, 55, 80, 0, 40, 8.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 4.0, 55, 60, 0, 20, 8.0, 0.0, 9.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        AttackModule::clear_all(agent.module_accessor);
    }
}

// FORWARD AIR 2 SOUND
unsafe extern "C" fn gamewatch_sound_attackairf2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave08_hi"));
    }
}

// FORWARD AIR 2 EXPRESSION
unsafe extern "C" fn gamewatch_expression_attackairf2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("lhand") as i64, hash40("lhand_block") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 7);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackss"), 7);
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

        .game_acmd("game_attackairb", gamewatch_attackairb, Low)
        .sound_acmd("sound_attackairb", gamewatch_sound_attackairb, Low)
        .expression_acmd("expression_attackairb", gamewatch_expression_attackairb, Low)

        .game_acmd("game_landingairb", gamewatch_landingairb, Low)

        .game_acmd("game_attackairf", gamewatch_attackairf, Low)

        .game_acmd("game_attackairf2", gamewatch_attackairf2, Low)
        .sound_acmd("sound_attackairf2", gamewatch_sound_attackairf2, Low)
        .expression_acmd("expression_attackairf2", gamewatch_expression_attackairf2, Low)

        .install();
}