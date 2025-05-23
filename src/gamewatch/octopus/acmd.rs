use super::*;

// OCTOPUS NAIR
unsafe extern "C" fn gamewatch_octopus_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.2, 361, 90, 0, 60, 6.5, -20.0, 14.0, 0.0, Some(20.0), Some(14.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.2, 361, 90, 0, 60, 6.5, -25.0, 24.0, 0.0, Some(25.0), Some(24.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);    
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 16.2, 361, 90, 0, 60, 6.5, -30.0, 34.0, 0.0, Some(30.0), Some(34.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);     
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 16.2, 361, 90, 0, 60, 6.5, -30.0, 44.0, 0.0, Some(30.0), Some(44.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);     
        AttackModule::set_special_paint(agent.module_accessor, 0, SpecialPaintKind{_address:*SPECIAL_PAINT_OIL as u8});
        AttackModule::set_special_paint(agent.module_accessor, 1, SpecialPaintKind{_address:*SPECIAL_PAINT_OIL as u8});
        AttackModule::set_special_paint(agent.module_accessor, 2, SpecialPaintKind{_address:*SPECIAL_PAINT_OIL as u8});
        AttackModule::set_special_paint(agent.module_accessor, 3, SpecialPaintKind{_address:*SPECIAL_PAINT_OIL as u8});
    }
}

//OCTOPUS NAIR EXPRESSION
unsafe extern "C" fn gamewatch_octopus_expression_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 7);
    }
}

pub fn install() {
    Agent::new("gamewatch_octopus")
        .game_acmd("game_attackairn", gamewatch_octopus_attackairn, Low)
        .expression_acmd("expression_attackairn", gamewatch_octopus_expression_attackairn, Low)
        .install();
}
