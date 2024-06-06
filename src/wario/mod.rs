use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

//GROUND MOVES

//JAB 2
unsafe extern "C" fn wario_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 20, 0, 25, 3.8, 0.0, 8.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 7.0, 361, 20, 0, 25, 5.8, 3.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
}

//RAPID JAB START
unsafe extern "C" fn wario_attack100start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 4.0);
    }
}

//RAPID JAB START EXPRESSION
unsafe extern "C" fn wario_expression_attack100start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//RAPID JAB
unsafe extern "C" fn wario_attack100(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 8, 5.2, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    macros::wait_loop_clear(agent);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 8, 5.2, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 8, 5.2, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 8, 5.2, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 14.0);
}

//RAPID JAB EFFECT
unsafe extern "C" fn wario_effect_attack100(agent: &mut L2CAgentBase) {
    wait_loop_sync_mot(0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 2, 0, 0, 0, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("packun_atk_100_bite"), Hash40::new("packun_atk_100_bite"), Hash40::new("top"), 0, 11, 10, 0, -90, 10, 0.8, 3, 10, 10, 0, 0, 45, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 3, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 5, 0, 0, 0, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("packun_atk_100_bite"), Hash40::new("packun_atk_100_bite"), Hash40::new("top"), 0, 8, 12, 0, -90, 10, 0.8, 3, 10, 10, 0, 0, 45, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 5, 0, 0, 0, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("packun_atk_100_bite"), Hash40::new("packun_atk_100_bite"), Hash40::new("top"), 0, 9, 12, 0, -90, 10, 0.8, 3, 10, 10, 0, 0, 45, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 3, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 5, 0, 0, 0, 0, -10, 0.7, 0, 1, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 14.0);
    macros::EFFECT_FLIP(agent, Hash40::new("packun_atk_100_bite"), Hash40::new("packun_atk_100_bite"), Hash40::new("top"), 0, 13, 12, 0, -90, 10, 0.8, 3, 10, 10, 0, 0, 45, true, *EF_FLIP_YZ);
}

//RAPID JAB SOUND
unsafe extern "C" fn wario_sound_attack100(agent: &mut L2CAgentBase) {
    wait_loop_sync_mot(0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_packun_attack100_01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_packun_attack100_01"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_packun_attack100_01"));
    }
    frame(agent.lua_state_agent, 13.0);
    macros::PLAY_STATUS(agent, Hash40::new("se_packun_attack100_01"));
}

//RAPID JAB EXPRESSION
unsafe extern "C" fn wario_expression_attack100(agent: &mut L2CAgentBase) {
    wait_loop_sync_mot(0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackss"), 6);
    }
    frame(agent.lua_state_agent, 9.0);
    ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
}
//RAPID JAB END
unsafe extern "C" fn wario_attack100end(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.4, 50, 150, 0, 40, 6.0, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(15.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//RAPID JAB END EFFECT
unsafe extern "C" fn wario_effect_attack100end(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_atk_100_finish"), Hash40::new("packun_atk_100_finish"), Hash40::new("top"), -8, 8, 15, 10, -90, 0, 1.4, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("packun_bite_line"), Hash40::new("mouth"), 3, 0, 0, 0, 0, 10, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2.5);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("neck"), 0, 0, 0, 0, 90, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.3);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, 0, 0, -20, 0, 0, 0.9, 0, 1, 0, 0, 0, 0, true);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("packun_atk_100_finish"), -1);
    }
}

//RAPID JAB END SOUND
unsafe extern "C" fn wario_sound_attack100end(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_packun_attack100_02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_packun_attack100_03"));
    }
}

//RAPID JAB END EXPRESSION
unsafe extern "C" fn wario_expression_attack100end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//FORWARD TILT
unsafe extern "C" fn wario_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 13.0, 361, 102, 0, 20, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 11.0, 361, 102, 0, 20, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 13.0, 361, 102, 0, 20, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 11.0, 361, 102, 0, 20, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//UP TILT
unsafe extern "C" fn wario_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        for _ in 0..10 {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 92, 130, 0, 28, 5.0, 0.0, 18.0, 5.0, Some(0.0), Some(18.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            wait(agent.lua_state_agent, 3.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 90, 120, 0, 20, 4.0, 0.0, 14.5, 3.5, Some(0.0), Some(14.5), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//DASH ATTACK
unsafe extern "C" fn wario_attackdash(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.1);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.65);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}




//AERIALS

//FORWARD AIR
unsafe extern "C" fn wario_attackairf(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 62, 97, 0, 30, 10.5, 0.0, 5.4, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 62, 97, 0, 30, 10.5, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 62, 80, 0, 15, 10.5, 0.0, 5.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 62, 80, 0, 15, 10.5, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


//NEUTRAL AIR
unsafe extern "C" fn wario_attackairn(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 6.0, 40, 100, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 100, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 40, 100, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 100, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 6.0, 40, 100, 0, 30, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 100, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 4.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATK_POWER(agent, 0, 3);
        macros::ATK_POWER(agent, 1, 3);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 100, 0, 65, 3.5, 0.0, 5.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATK_POWER(agent, 0, 5);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


// Game acmd script
unsafe extern "C" fn example_acmd_script(agent: &mut L2CAgentBase) {
    
}

// Char opff, Global opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
    }
}

// Status script
unsafe extern "C" fn wario_attacks3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let defender_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1)); 
    let collision_kind = sv_battle_object::kind(Fighter::get_id_from_entry_id(1)); 
    let category = utility::get_category(&mut *defender_boma);
    let rand = smash::app::sv_math::rand(hash40("fighter"), 2);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        println!("category");
        if collision_kind == *COLLISION_KIND_HIT {
            println!("collision_kind");
            if rand != 1 {
                println!("random!");
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WIN, false);
            }
        }
    }

    0.into()
}



pub fn install() {
    Agent::new("wario")
        .game_acmd("game_attack12", wario_attack12)
        .game_acmd("game_attack100start", wario_attack100start)
        .game_acmd("game_attack100", wario_attack100)
        .game_acmd("game_attack100end", wario_attack100end)
        .game_acmd("game_attacks3", wario_attacks3)
        .game_acmd("game_attackdash", wario_attackdash)
        .game_acmd("game_attackairf", wario_attackairf)
        .game_acmd("game_attackairn", wario_attackairn)
        .game_acmd("game_attackhi3", wario_attackhi3)
        .effect_acmd("effect_attack100", wario_effect_attack100)
        .effect_acmd("effect_attack100end", wario_effect_attack100end)
        .sound_acmd("sound_attack100end", wario_sound_attack100end)
        .sound_acmd("sound_attack100", wario_sound_attack100)
        .expression_acmd("expression_attack100start", wario_expression_attack100start)
        .expression_acmd("expression_attack100end", wario_expression_attack100end)
        .expression_acmd("expression_attack100", wario_expression_attack100)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S3, wario_attacks3_check_attack_status)
        .install();
    Agent::new("fighter")
        .on_line(Main, fighter_frame) // Global opff
        .install();
}