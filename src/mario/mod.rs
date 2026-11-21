use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;
pub mod pump;

const FIGHTER_MARIO_STATUS_KIND_SHRINK: i32 = 0x1EC;

const WEAPON_MARIO_PUMP_STATUS_KIND_BOOST: i32 = 0x5;
const WEAPON_MARIO_PUMP_WATER_STATUS_KIND_BOOST: i32 = 0x8;

static mut SHRUNK: [bool; 8] = [false; 8];
static mut ICEBALL: [bool; 8] = [false; 8];
static mut FORWARD_SMASH_CHARGE: [f32; 8] = [0.0; 8];
static mut FORWARD_AIR_CHARGE: [f32; 8] = [0.0; 8];
static mut SHRINK_TIME: [i32; 8] = [0; 8];
static mut COIN_COUNT: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();
    pump::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_MARIO, slots.clone(), (hash40("run_speed_max"), 0, 1.86));
}