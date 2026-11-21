use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod wariobike;

static mut DOWN_SMASH_AMOUNT: [i32; 8] = [0; 8];
static mut WECTOR: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    wariobike::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // VL PARAMS
    param_config::update_int_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_s"), hash40("interval_time"), 0));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("start_speed_x_ground"), 10.6));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("start_speed_x_air"), 11.8));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("start_speed_y_add_air"), 0.2));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("accel_x_rate"), 0.05));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("speed_x_min_ground"), 2.9));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("speed_x_max_ground"), 4.6));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("speed_x_min_air"), 2.9));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("speed_x_max_air"), 4.6));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("accel_y"), 0.015));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("speed_y_max"), 3.5));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("turn_frame_rate_max"), 300.0));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("turn_frame_rate_min"), 100.0));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("turn_brake"), 100.07));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("turn_attack_necessary_speed"), 1.1));
    param_config::update_float_2(-*WEAPON_KIND_WARIO_WARIOBIKE, slots.clone(), (hash40("param_wariobike"), hash40("turn_attack_ready_frames"), 1.0));
    param_config::update_float_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_hi"), hash40("start_y_spd_add"), -0.8));
    param_config::update_float_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_hi"), hash40("start_y_acl"), 0.03));
    param_config::update_float_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_hi"), hash40("jump_y_spd_mul"), 1.2));
    param_config::update_float_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_lw"), hash40("gass_max_time"), 40.0));
    param_config::update_float_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_lw"), hash40("gass_middle_time"), 5.0));
    param_config::update_float_2(*FIGHTER_KIND_WARIO, slots.clone(), (hash40("param_special_lw"), hash40("gass_large_time"), 20.0));
}