use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash2::*,
    smash_script::*,
    smashline::*,
    smashline::Priority::*
};



//GROUNDED MOVES

//RAPID JAB
unsafe extern "C" fn falco_attack100(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        //macros::ATTACK(agent, 1, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        println!("im fuckin spinning");
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    macros::wait_loop_clear(agent);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
    frame(agent.lua_state_agent, 16.0);
}

//RAPID JAB SUB
unsafe extern "C" fn falco_attack100sub(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 15, 0, 6, 5.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        println!("im fuckin spinning SUB");
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//FORWARD TILT
unsafe extern "C" fn falco_attacks3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 6.0, 120, 80, 0, 25, 3.2, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 6.0, 120, 80, 0, 25, 3.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 6.0, 120, 80, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


//UP TILT
unsafe extern "C" fn falco_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 110, 100, 92, 0, 6.0, 0.0, 9.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.5, 110, 100, 70, 0, 6.0, 0.0, 9.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 3.5, 100, 100, 40, 0, 4.0, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 3.5, 100, 100, 40, 0, 4.0, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 2.7, 270, 100, 15, 0, 5.0, 5.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 4.0, 90, 150, 0, 30, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 90, 150, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 4.0, 97, 150, 0, 30, 5.0, 6.2, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//DOWN TILT
unsafe extern "C" fn falco_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("tail2"), 13.0, 85, 90, 0, 50, 2.6, -4.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("tail2"), 12.0, 85, 90, 0, 50, 3.5, 1.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail2"), 10.5, 85, 90, 0, 50, 3.3, 7.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 3, 0, Hash40::new("tail2"), 10.5, 85, 90, 0, 50, 3.3, 13.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


//DASH ATTACK
unsafe extern "C" fn falco_attackdash(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 6.0, 70, 30, 0, 55, 4.7, 4.4, -0.7, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 70, 30, 0, 55, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 6.0, 70, 30, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.8);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);

    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 4.0, 70, 30, 0, 40, 3.8, 3.8, -0.6, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 4.0, 70, 30, 0, 40, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 4.0, 70, 30, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.8);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);

    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


//SMASH ATTACKS

//FORWARD SMASH
unsafe extern "C" fn falco_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 16.0, 361, 89, 0, 42, 2.2, 0.0, 1.5, -1.0, Some(0.0), Some(-3.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 16.0, 361, 91, 0, 42, 3.0, 5.0, 1.8, -1.0, Some(0.0), Some(-3.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 16.0, 361, 93, 0, 42, 4.0, 10.0, 2.2, -1.0, Some(0.0), Some(-4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 16.0, 361, 91, 0, 42, 3.0, 4.0, 1.8, -1.0, Some(0.0), Some(-3.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 16.0, 361, 93, 0, 42, 4.0, 8.0, 2.2, -1.0, Some(0.0), Some(-4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 89, 0, 20, 6.0, 0.0, 2.0, 12.0, Some(0.0), Some(2.0), Some(6.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 1, false);
        AttackModule::clear(agent.module_accessor, 2, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


//DOWN SMASH
unsafe extern "C" fn falco_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 155, 98, 0, 40, 4.3, 0.0, 1.7, 9.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 155, 98, 0, 40, 4.3, 0.0, 1.7, -12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 361, 98, 0, 40, 3.5, 0.0, 1.7, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 361, 98, 0, 40, 3.5, 0.0, 1.7, -5.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);        
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}


//UP SMASH
unsafe extern "C" fn falco_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 110, 15, 0, 70, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 4.0, 110, 15, 0, 70, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 4.0, 130, 15, 0, 70, 4.0, 7.7, -1.3, -1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneer"), 4.0, 110, 100, 100, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("legr"), 4.0, 95, 100, 100, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("kneer"), 4.0, 127, 100, 100, 0, 4.0, 7.7, -1.3, -1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 110, 15, 0, 45, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 4.0, 110, 15, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 4.0, 200, 100, 20, 0, 3.5, 7.0, -1.3, -1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(agent.module_accessor, 3, false);
        AttackModule::clear(agent.module_accessor, 4, false);
        AttackModule::clear(agent.module_accessor, 5, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 1, Hash40::new("kneel"), 13.0, 80, 107, 0, 28, 5.7, 7.0, 0.0, 0.0, Some(0.0), Some(5.0), Some(-3.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 1, Hash40::new("kneel"), 13.0, 80, 107, 0, 28, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 1, Hash40::new("legl"), 13.0, 80, 107, 0, 28, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 1, Hash40::new("legl"), 13.0, 80, 137, 0, 28, 3.0, 0.0, 20.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("kneel"), 13.0, 80, 107, 0, 28, 5.7, 7.0, 0.0, 0.0, Some(0.0), None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}


//AERIALS

//NEUTRAL AIR
unsafe extern "C" fn falco_attackairn(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 366, 100, 30, 0, 5.0, 0.0, 11.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 3.0, 366, 100, 30, 0, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 3.0, 366, 100, 30, 0, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("shoulderl"), 3.0, 366, 100, 30, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::SET_SPEED_EX(agent, 1.5, KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 30, 0, 5.0, 0.0, 11.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 2.0, 366, 100, 30, 0, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 2.0, 366, 100, 30, 0, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("shoulderl"), 2.0, 366, 100, 30, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 100, 30, 0, 5.0, 0.0, 11.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 2.0, 366, 100, 30, 0, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 2.0, 366, 100, 30, 0, 3.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("shoulderl"), 2.0, 366, 100, 30, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 130, 0, 55, 7.0, 0.0, 7.0, 7.0, Some(0.0), Some(11.0), Some(1.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//FORWARD AIR
unsafe extern "C" fn falco_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 60, 0, 4.0, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 367, 100, 60, 0, 4.0, 0.0, 5.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 137, 0, 55, 5.0, 0.0, 5.5, 12.0, Some(0.0), Some(5.5), Some(8.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


//UP AIR
unsafe extern "C" fn falco_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 9.0, 65, 95, 0, 38, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 9.0, 75, 95, 0, 38, 4.2, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 9.0, 85, 95, 0, 38, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_hi_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
    }    
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//UP AIR HOLD
unsafe extern "C" fn falco_attackairhihold(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.5, 55, 80, 0, 40, 5.5, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.5, 55, 80, 0, 40, 4.2, 2.5, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 7.5, 55, 80, 0, 40, 3.0, -1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }    
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_hi_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//UP AIR EFFECT
unsafe extern "C" fn falco_effect_attackairhihold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 11, 0, 180, -70, 90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 11, 0, 180, -140, 90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
}

//DOWN AIR
unsafe extern "C" fn falco_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 270, 80, 0, 10, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 9.0, 270, 50, 0, 55, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 8.0, 270, 90, 0, 20, 7.2, 3.5, 3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(agent.module_accessor, 1, false);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 32.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 52.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

//BACK AIR
unsafe extern "C" fn falco_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 13.0, 361, 130, 0, 0, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 13.0, 361, 130, 0, 0, 3.2, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 7.0, 361, 100, 0, 30, 3.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.0, 361, 100, 0, 30, 2.8, -2.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//BACK AIR SOUND EFFECT
unsafe extern "C" fn falco_sound_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_attack07"));
        macros::PLAY_SE(agent, Hash40::new("se_falco_swing_m"));
    }
}

//SPECIALS

//UP SPECIAL
unsafe extern "C" fn falco_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 3.0, 367, 60, 0, 60, 5.5, 4.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 3.0, 80, 100, 80, 0, 5.5, 4.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
    for _ in 0..7 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 2.0, 367, 60, 0, 60, 4.0, 4.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 2.0, 65, 100, 80, 0, 4.0, 4.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 2.0, 361, 90, 0, 70, 10.0, 4.2, -3.1, -1.5, None, None, None, 1.7, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//UP SPECIAL EFFECT
unsafe extern "C" fn falco_effect_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("falco_firebird_start"), Hash40::new("waist"), -8, 0, 0, 0, 90, 10, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("falco_firebird"), Hash40::new("rot"), 0.75, -1.5, 1.5, 90, 0, 0, 0.85, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 0.2, 0.0, 1.0, 0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("rot"), 0, -3, 7, 0, 0, 0, 2.7, 0, 0, 0, 0, 0, 0, true, 0.9);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_firebird"), false, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 2, 0.1, 0, 0.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_FRAME(agent, 2, 0.2, 0.0, 1.0, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 1.0);
}




//SIDE SPECIAL
unsafe extern "C" fn falco_specials(agent: &mut L2CAgentBase) {
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION, false, -1);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION, false, -1);
    }
}

//DOWN SPECIAL
unsafe extern "C" fn falco_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 361, 78, 0, 50, 3.5, 1.5, 0.0, 0.0, None, None, None, 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 140, 78, 0, 50, 3.5, 1.5, 0.0, 0.0, None, None, None, 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

//AERIAL DOWN SPECIAL
unsafe extern "C" fn falco_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 361, 30, 0, 35, 3.5, 1.5, 0.0, 0.0, None, None, None, 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 140, 78, 0, 50, 3.5, 1.5, 0.0, 0.0, None, None, None, 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

//THROWS

//BACK THROW
unsafe extern "C" fn falco_throwb(agent: &mut L2CAgentBase) {
    PostureModule::scale(agent.module_accessor);
    // if(methodlib::L2CValue::operator<=(lib::L2CValueconst&)const(149605315, 1.4)) {
    //     if macros::is_excute(agent) {
    //         macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 35, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    //         macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    //     }
    //     else {
    //     PostureModule::scale(agent.module_accessor);
    //         if(methodlib::L2CValue::operator<=(lib::L2CValueconst&)const(149605315, 0.5)) {
    //             if macros::is_excute(agent) {
    //                 macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 44, 40, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    //                 macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    //             }
    //             else {
    //                 if macros::is_excute(agent) {
    //                     macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 35, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    //                     macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    //                 }
    //             }
    //         }
    //     }
    // }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(agent, 7, 8);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 2.0, y: 2.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//UP THROW
unsafe extern "C" fn falco_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 94, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 6, 13);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 3.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//DOWN THROW
unsafe extern "C" fn falco_throwlw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 50, 110, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        macros::CHECK_FINISH_CAMERA(agent, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        macros::CHECK_FINISH_CAMERA(agent, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        macros::CHECK_FINISH_CAMERA(agent, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        macros::CHECK_FINISH_CAMERA(agent, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        macros::CHECK_FINISH_CAMERA(agent, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        macros::CHECK_FINISH_CAMERA(agent, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


//TAUNTS

//SIDE TAUNT FACING RIGHT
unsafe extern "C" fn falco_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

//SIDE TAUNT SOUND EFFECT
unsafe extern "C" fn falco_sound_appeals(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_appeal02"));
    }
}

// Char opff, Global opff
unsafe extern "C" fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT).try_into().unwrap());
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 1.5, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 1.5, y: PostureModule::pos_y(fighter.module_accessor)});
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash") {
            if MotionModule::frame(fighter.module_accessor) > 12.0 && MotionModule::frame(fighter.module_accessor) < 19.0 {
                if xpos > 0.5 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                if xpos < -0.5 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                    
                }
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw_4") {
            if MotionModule::frame(fighter.module_accessor) > 1.0 {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n") {
            if MotionModule::frame(fighter.module_accessor) > 8.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
                }
                if xpos > 0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
                }
                if xpos < -0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
                }
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") && is_touch {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f_beak_bonk"), 0.0, 1.0, false, 0.0, false, false);
            PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.0, y: PostureModule::pos_y(fighter.module_accessor)});
        }
    }
}
// Status script


//-------CHECK ATTACK--------

unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

//FALCO DOWN SPECIAL CHECK ATTACK
unsafe extern "C" fn falco_special_lw_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
            }
        }
    }
    0.into()
}


pub fn install() {
    Agent::new("falco")
        .game_acmd("game_attack100", falco_attack100, Low)
        .game_acmd("game_attacks3", falco_attacks3, Low)
        .game_acmd("game_attacklw3", falco_attacklw3, Low)
        .game_acmd("game_attackdash", falco_attackdash, Low)
        .game_acmd("game_attacks4", falco_attacks4, Low)
        .game_acmd("game_attacklw4", falco_attacklw4, Low)
        .game_acmd("game_attackhi4", falco_attackhi4, Low)
        .game_acmd("game_attackhi3", falco_attackhi3, Low)
        .game_acmd("game_attackairhi", falco_attackairhi, Low)
        .game_acmd("game_attackairn", falco_attackairn, Low)
        .game_acmd("game_specialhi", falco_specialhi, Low)
        .game_acmd("game_appealsr", falco_appeals, Low)
        .game_acmd("game_appealsl", falco_appeals, Low)
        .game_acmd("game_attackairlw", falco_attackairlw, Low)
        .game_acmd("game_specials", falco_specials, Low)
        .game_acmd("game_speciallw", falco_speciallw, Low)
        .game_acmd("game_attack100sub", falco_attack100sub, Low)
        .game_acmd("game_specialairlw", falco_specialairlw, Low)
        .game_acmd("game_attackairhihold", falco_attackairhihold, Low)
        .game_acmd("game_attackairf", falco_attackairf, Low)
        .game_acmd("game_attackairb", falco_attackairb, Low)
        .game_acmd("game_throwb", falco_throwb, Low)
        .game_acmd("game_throwhi", falco_throwhi, Low)
        .game_acmd("game_throwlw", falco_throwlw, Low)
        .effect_acmd("effect_specialhi", falco_effect_specialhi, Low)
        .effect_acmd("effect_attackairhihold", falco_effect_attackairhihold, Low)
        .sound_acmd("sound_attackairb", falco_sound_attackairb, Low)
        .sound_acmd("sound_appealsr", falco_sound_appeals, Low)
        .sound_acmd("sound_appealsl", falco_sound_appeals, Low)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_LW, falco_special_lw_check_attack_status)
        .on_line(Main, falco_frame) // Char opff
        .install();
}
