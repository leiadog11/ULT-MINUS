use super::*;

//-------------------SPECIALS------------------

// DOWN SPECIAL START
unsafe extern "C" fn gamewatch_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 10, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 24, 45, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
}

// DOWN SPECIAL
unsafe extern "C" fn gamewatch_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_ABSORB_ENABLE);
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

// //UP SPECIAL
// unsafe extern "C" fn gamewatch_specialhi(agent: &mut L2CAgentBase) {
//     if macros::is_excute(agent) {
//         WorkModule::set_int64(agent.module_accessor, hash40("special_hi") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
//     }
//     frame(agent.lua_state_agent, 1.0);
//     if macros::is_excute(agent) {
//         ArticleModule::generate_article(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_FIREPILLAR, false, -1);
//         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 55, 100, 0, 60, 4.5, 0.0, 4.0, 2.0, Some(0.0), Some(4.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 100, 0, 65, 5.0, 0.0, -4.0, 6.0, Some(0.0), Some(-8.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//     }
//     frame(agent.lua_state_agent, 2.0);
//     if macros::is_excute(agent) {
//         AttackModule::clear_all(agent.module_accessor);
//     }
//     frame(agent.lua_state_agent, 60.0);
//     if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
//         if macros::is_excute(agent) {
//             ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
//         }
//     }
//     if MotionModule::is_changing(agent.module_accessor) {
//         if macros::is_excute(agent) {
//             WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
//         }
//     }
// }




pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_speciallwstart", gamewatch_speciallwstart, Low)
        .game_acmd("game_specialairlwstart", gamewatch_speciallwstart, Low)

        .game_acmd("game_speciallw", gamewatch_speciallw, Low)
        .game_acmd("game_specialairlw", gamewatch_speciallw, Low)

        //.game_acmd("game_specialhi", gamewatch_specialhi, Low)

        .install();
}