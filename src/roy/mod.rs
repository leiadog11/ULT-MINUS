use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

const FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_ROLL: i32 = 0x1EA;
const FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_DIVE: i32 = 0x1EB;
const FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_LANDING: i32 = 0x1EC;

static mut PYRA_REMOVED: [bool; 8] = [true; 8];

// REMOVE PYRA
unsafe extern "C" fn remove_pyra(boma: *mut BattleObjectModuleAccessor) { 
    let ENTRY_ID = get_entry_id(boma);
    PYRA_REMOVED[ENTRY_ID] = true;
    ModelModule::set_mesh_visibility(boma, Hash40::new("Pyra"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("pyra"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("alp_flame_002"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("alp_flame_003"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("alp_flame_005"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("skin_flame_001"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("skin_flame_002"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_001"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_001_add"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_002"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("def_flame_004"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("weaponbladem"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("weapongripm"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("weapon_blade_m"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("weapon_grip_m"), true);
}

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}