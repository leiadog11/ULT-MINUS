use super::*;

// //TETHER GRAB
// unsafe extern "C" fn jack_catch(agent: &mut L2CAgentBase) {
//     if macros::is_excute(agent) {
//         ArticleModule::generate_article(agent.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, -1);
//         ArticleModule::change_motion(agent.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("catch"), false, -1.0);
//     }
//     frame(agent.lua_state_agent, 6.0);
//     if macros::is_excute(agent) {
//         GrabModule::set_rebound(agent.module_accessor, true);
//     }
//     frame(agent.lua_state_agent, 7.0);
//     if macros::is_excute(agent) {
//         macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.5, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
//     }
//     // macros::game_CaptureCutCommon(agent);
//     // frame(agent.lua_state_agent, 9.0);
//     // if macros::is_excute(agent) {
//     //     macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
//     // }
//     frame(agent.lua_state_agent, 18.0);
//     if macros::is_excute(agent) {
//         grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
//         GrabModule::set_rebound(agent.module_accessor, false);
//     }
//     frame(agent.lua_state_agent, 69.0);
//     if macros::is_excute(agent) {
//         ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
//     }
// }

// //ZAIR
// unsafe extern "C" fn jack_catchattack(agent: &mut L2CAgentBase) {
//     if macros::is_excute(agent) {
//         ArticleModule::change_motion(agent.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("catch_attack"), false, -1.0);
//     }
//     frame(agent.lua_state_agent, 1.0);
//     if macros::is_excute(agent) {
//         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 361, 100, 30, 0, 5.0, 0.0, 8.0, 8.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
//         AttackModule::set_catch_only_all(agent.module_accessor, true, false);
//     }
//     wait(agent.lua_state_agent, 1.0);
//     if macros::is_excute(agent) {
//         AttackModule::clear_all(agent.module_accessor);
//     }
// }

pub fn install() {
    Agent::new("jack")
        // .game_acmd("game_catchattack", jack_catchattack, Low)

        // .game_acmd("game_catch", jack_catch, Low)

        .install();
}