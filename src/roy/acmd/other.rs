use super::*;

// UP TAUNT EXPRESSION
unsafe extern "C" fn roy_expression_appealhi(agent: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let ENTRY_ID = get_entry_id(boma);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) { 
        PYRA_REMOVED[ENTRY_ID] = false;
        ModelModule::set_mesh_visibility(boma, Hash40::new("Pyra"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("pyra"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("alp_flame_002"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("alp_flame_003"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("alp_flame_005"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("skin_flame_001"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("skin_flame_002"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_001"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_001_add"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_002"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_004"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("weaponbladem"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("weapongripm"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("weapon_blade_m"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("weapon_grip_m"), false);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 73.0);
    if macros::is_excute(agent) {
        remove_pyra(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("roy")
        .expression_acmd("expression_appealhir", roy_expression_appealhi, Low)
        .expression_acmd("expression_appealhil", roy_expression_appealhi, Low)

        .install();
}