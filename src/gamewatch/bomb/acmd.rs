use super::*;

// FLY
unsafe extern "C" fn bomb_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = get_entry_id(owner_boma);
        BOMB_OUT[ENTRY_ID] = true;
        EXPLODED[ENTRY_ID] = false;
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 0.4, 0.0, Some(0.0), Some(0.4), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::off_target_kind(agent.module_accessor, 0, (*COLLISION_KIND_MASK_HIT).try_into().unwrap());
    }
}

// BURST
unsafe extern "C" fn bomb_burst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = get_entry_id(owner_boma);
        BOMB_OUT[ENTRY_ID] = false;
        EXPLODED[ENTRY_ID] = true;
        VisibilityModule::set_whole(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 90, 0, 40, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    Agent::new("gamewatch_bomb")
        .game_acmd("game_fly", bomb_fly, Low)

        .game_acmd("game_burst", bomb_burst, Low)
        .install();
}