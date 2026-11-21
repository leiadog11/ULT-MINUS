use super::*;

pub mod acmd;
pub mod frame;
pub mod blaster_bullet;

pub fn install() {
    acmd::install();
    frame::install();
    blaster_bullet::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_FALCO, slots.clone(), (hash40("run_speed_max"), 0, 1.699));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, slots.clone(), (hash40("air_speed_x_stable"), 0, 1.125));
    param_config::update_int_2(*FIGHTER_KIND_FALCO, slots.clone(), (hash40("jump_count_max"), 0, 3));
}