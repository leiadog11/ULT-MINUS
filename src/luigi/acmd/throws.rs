use super::*;

//-------------------GRABS--------------------

// CATCH - GRAB
unsafe extern "C" fn luigi_catch(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch"), false, -1.0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        macros::SEARCH(agent, 0, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
        /*
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("catch_suck"), 0.0, 1.0, false, 0.0, false, false);
        }
        */
    }
}


// SUCK
/*
unsafe extern "C" fn luigi_catchsuck(agent: &mut L2CAgentBase) {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        for _ in 0..3 {
            println!("Doin it?");
            macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            wait(agent.lua_state_agent, 1.0);
        }

        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            for _ in 0..3 {
                println!("In here?");
                macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
                macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
                wait(agent.lua_state_agent, 1.0);
            }
        }
    }

    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("catch_cut"), 0.0, 1.0, false, 0.0, false, false);
    }
}
*/

// UP THROW
unsafe extern "C" fn luigi_throwhi(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_hi"), false, -1.0);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 30, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 30, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //doubles hitbox
        //macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 6.0, 88, 55, 0, 95, 8.5, 0.0, 8.0, 10.0, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, -2, 24);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 7.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::SET_SPEED_EX(agent, 0.5, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
    }

}

pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_catch", luigi_catch, Low)
        
        .game_acmd("game_throwhi", luigi_throwhi, Low)

        .install();
}