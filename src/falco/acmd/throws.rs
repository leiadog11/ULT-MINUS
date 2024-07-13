use super::*;

//------------------GRABS--------------------

// GRAB
unsafe extern "C" fn falco_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("trans"), 3.1, 0.0, 7.2, 4.0, Some(0.0), Some(7.2), Some(8.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("trans"), 1.55, 0.0, 7.2, 2.45, Some(0.0), Some(7.2), Some(10.45), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

// DASH GRAB
unsafe extern "C" fn falco_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("trans"), 2.5, 0.0, 7.2, 4.0, Some(0.0), Some(6.0), Some(10.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("trans"), 1.25, 0.0, 7.2, 2.75, Some(0.0), Some(6.0), Some(11.75), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

// PIVOT GRAB
unsafe extern "C" fn falco_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("trans"), 3.1, 0.0, 7.2, -4.0, Some(0.0), Some(7.2), Some(-14.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("trans"), 1.55, 0.0, 7.2, -2.45, Some(0.0), Some(7.2), Some(-16.45), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}


//-------------------THROWS------------------

// BACK THROW
unsafe extern "C" fn falco_throwb(agent: &mut L2CAgentBase) {
    PostureModule::scale(agent.module_accessor);
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

// UP THROW
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

// DOWN THROW
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

pub fn install() {
    Agent::new("falco")
        .game_acmd("game_catch", falco_catch, Low)

        .game_acmd("game_catchdash", falco_catchdash, Low)

        .game_acmd("game_catchturn", falco_catchturn, Low)
        
        .game_acmd("game_throwb", falco_throwb, Low)

        .game_acmd("game_throwhi", falco_throwhi, Low)

        .game_acmd("game_throwlw", falco_throwlw, Low)

        .install();
}