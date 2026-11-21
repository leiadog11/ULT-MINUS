use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut PUMMEL_AMOUNT: [i32; 8] = [0; 8];
static mut GYRO_LIFE: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_int_2(*FIGHTER_KIND_ROBOT, slots.clone(), (hash40("attack_combo_max"), 0, 3));
    param_config::update_int_2(*FIGHTER_KIND_ROBOT, slots.clone(), (hash40("jump_count_max"), 0, 2));

    // VL PARAMS
    param_config::update_float_2(*FIGHTER_KIND_ROBOT, slots.clone(), (hash40("param_special_n"), hash40("miss_shoot_energy"), -1.0));
    param_config::update_float_2(*FIGHTER_KIND_ROBOT, slots.clone(), (hash40("param_special_n"), hash40("strong_shoot_energy"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_ROBOT, slots.clone(), (hash40("param_special_n"), hash40("start_energy"), 0.0));
}