use super::*;

//------------------GROUND-------------------

//JAB
// unsafe extern "C" fn gamewatch_attack100(agent: &mut L2CAgentBase) {
//     if macros::is_excute(agent) {
//         if macros::is_excute(agent) {
//             WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
//         }
//         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 5.5, 0.0, 5.0, 14.5, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
//         macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 4.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(5.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
//         AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
//         AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
//         macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 10);
//         macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 10);
//         ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, false, -1);
//     }
//     wait(agent.lua_state_agent, 1.0);
//     if macros::is_excute(agent) {
//         AttackModule::clear_all(agent.module_accessor);
//         WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
//     }
//     macros::wait_loop_clear(agent);
//     if macros::is_excute(agent) {
//         ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_100"), false, -1.0);
//     }
//     frame(agent.lua_state_agent, 5.0);
//     if macros::is_excute(agent) {
//         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 5.5, 0.0, 5.0, 14.5, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
//         macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 4.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(5.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
//         AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
//         AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
//         macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 10);
//         macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 10);
//     }
//     wait(agent.lua_state_agent, 1.0);
//     if macros::is_excute(agent) {
//         AttackModule::clear_all(agent.module_accessor);
//         WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
//     }
//     frame(agent.lua_state_agent, 13.0);
// }


//DOWN TILT
unsafe extern "C" fn gamewatch_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_MANHOLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_lw3"), false, -1.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 1, 112, 0, 45, 1.2, 0.0, 2.1, 6.7, Some(0.0), Some(2.1), Some(16.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 1, 112, 0, 45, 1.2, 0.0, 9.3, 6.7, Some(0.0), Some(2.1), Some(16.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        //ArticleModule::remove(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON);
    }
}

//UP TILT
unsafe extern "C" fn gamewatch_attackhi3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.833);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 98, 76, 0, 52, 3.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 8.0, 98, 76, 0, 52, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.842);
}

//FORWARD TILT


//DASH ATTACK
unsafe extern "C" fn gamewatch_attackdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 9.0, 4.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 50, 0, 20, 6.0, 0.0, 3.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.5, 361, 50, 0, 20, 5.5, 0.0, 3.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 12.0);
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}


pub fn install() {
    Agent::new("gamewatch")
        // .game_acmd("game_attack100", gamewatch_attack100, Low)

        .game_acmd("game_attacklw3", gamewatch_attacklw3, Low)

        .game_acmd("game_attackhi3", gamewatch_attackhi3, Low)

        .game_acmd("game_attackdash", gamewatch_attackdash, Low)


        .install();
}