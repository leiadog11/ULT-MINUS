use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;

static mut MISFIRE_SPECIAL_N: [bool; 8] = [false; 8];
static mut MISFIRE_UP_SMASH: [bool; 8] = [false; 8];
static mut DOWN_TILT_COUNTER: [i32; 8] = [0; 8];
static mut NEG_ZONE: [f32; 8] = [0.1; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, slots.clone(), (hash40("run_speed_max"), 0, 1.75));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, slots.clone(), (hash40("jump_y"), 0, 50.0));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, slots.clone(), (hash40("ground_brake"), 0, 0.026));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, slots.clone(), (hash40("air_speed_x_stable"), 0, 0.87));
    param_config::update_int_2(*FIGHTER_KIND_LUIGI, slots.clone(), (hash40("wall_jump_type"), 0, 1));

    // VL PARAMS
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, slots.clone(), (hash40("param_special_s"), hash40("discharge_prob"), 40.0));
}