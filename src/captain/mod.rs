use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

const FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2: i32 = 0x1ED;

static mut KICK: [bool; 8] = [false; 8];
static mut KICK_SPEED: [u64; 8] = [0; 8];
static mut GUN_COOLDOWN: [i32; 8] = [0; 8];
static mut UP_B_AMOUNT: [i32; 8] = [2; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, slots.clone(), (hash40("dash_speed"), 0, 2.28));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, slots.clone(), (hash40("run_speed_max"), 0, 2.852));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, slots.clone(), (hash40("jump_speed_x"), 0, 1.1));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, slots.clone(), (hash40("mini_jump_y"), 0, 13.98));
    param_config::update_int_2(*FIGHTER_KIND_CAPTAIN, slots.clone(), (hash40("jump_count_max"), 0, 3));
}