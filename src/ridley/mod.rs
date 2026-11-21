use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut AURA: [bool; 8] = [false; 8];
static mut UP_AIR_HOLD: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("dash_speed"), 0, 1.9));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("run_accel_add"), 0, 0.05));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("run_accel_add"), 0, 2.5));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("jump_speed_x"), 0, 1.5));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("jump_speed_x_max"), 0, 1.8));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("jump_y"), 0, 36.0));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("mini_jump_y"), 0, 16.2));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("jump_aerial_y"), 0, 40.0));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("air_accel_x_mul"), 0, 0.07));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("air_speed_x_stable"), 0, 1.55));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("weight"), 0, 130.0));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("scale"), 0, 1.35));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("shield_radius"), 0, 16.6));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("shield_break_y"), 0, 45.0));
    param_config::update_int_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("jump_count_max"), 0, 5));

    // VL PARAMS
    param_config::update_int_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("param_special_n"), hash40("max_fire_num"), 20));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("param_special_n"), hash40("shoot_offset_x"), 15.2));
    param_config::update_float_2(*FIGHTER_KIND_RIDLEY, slots.clone(), (hash40("param_special_n"), hash40("shoot_offset_y"), 14.5));
    param_config::update_int_2(-*WEAPON_KIND_RIDLEY_BREATH, slots.clone(), (hash40("param_breath"), hash40("life"), 75));
}