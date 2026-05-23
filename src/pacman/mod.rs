use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bigpacman;
pub mod firehydrant;

static mut UP_SMASH: [bool; 8] = [false; 8];
static mut DOWN_SMASH: [bool; 8] = [false; 8];
static mut ITEM_CHOICE: [i32; 8] = [0; 8];
static mut KEY_COOLDOWN: [i32; 8] = [0; 8];
static mut APPLE_COOLDOWN: [i32; 8] = [0; 8];
static mut MELON_COOLDOWN: [i32; 8] = [0; 8];
static mut GALAXIAN_COOLDOWN: [i32; 8] = [0; 8];
static mut BELL_COOLDOWN: [i32; 8] = [0; 8];

extern "C" {
    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
    pub fn dead_range(lua_state: u64) -> Vector4f; 
}

// BLASTZONE CHECK
unsafe extern "C" fn blastzone_check(agent: &mut L2CAgentBase, side_add: f32) { 
    let boma = agent.module_accessor;
    let posx = PostureModule::pos_x(boma);
    let posy = PostureModule::pos_y(boma);

    if posx <= dead_range(agent.lua_state_agent).x + side_add {
        PostureModule::set_pos(boma, &Vector3f{ x: dead_range(agent.lua_state_agent).y - side_add , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
        AttackModule::clear_all(boma);
        GroundModule::set_collidable(boma, false);
    }
    else if posx >= dead_range(agent.lua_state_agent).y - side_add {
        PostureModule::set_pos(boma, &Vector3f{ x: dead_range(agent.lua_state_agent).x + side_add , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
        AttackModule::clear_all(boma);
        GroundModule::set_collidable(boma, false);
    }
    else if posy >= dead_range(agent.lua_state_agent).z - 70.0 {
        PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: dead_range(agent.lua_state_agent).w + 70.0, z: PostureModule::pos_z(boma)});
        AttackModule::clear_all(boma);
        GroundModule::set_collidable(boma, false);
    }
    else if posy <= dead_range(agent.lua_state_agent).w + 70.0 {
        PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: dead_range(agent.lua_state_agent).z - 70.0, z: PostureModule::pos_z(boma)});
        AttackModule::clear_all(boma);
        GroundModule::set_collidable(boma, false);
    }
}

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bigpacman::install();
    firehydrant::install();
}