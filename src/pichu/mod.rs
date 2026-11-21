use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod dengekidama;

static mut BLOWN_UP: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    dengekidama::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("walk_accel_mul"), 0, 0.1775));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("walk_speed_max"), 0, 1.602));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("ground_brake"), 0, 0.13));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("dash_speed"), 0, 2.58));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("run_accel_mul"), 0, 0.16));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("run_speed_max"), 0, 2.692));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("jump_speed_x"), 0, 1.2));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("jump_y"), 0, 36.95));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("air_accel_x_mul"), 0, 0.22));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("air_speed_x_stable"), 0, 1.429));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("dive_speed_y"), 0, 2.8));
    param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), (hash40("scale"), 0, 0.9));

    // VL PARAMS
    param_config::update_float_2(-*WEAPON_KIND_PICHU_DENGEKIDAMA, slots.clone(), (hash40("param_dengekidama"), hash40("life_"), 9000.0));
    param_config::update_float_2(-*WEAPON_KIND_PICHU_DENGEKIDAMA, slots.clone(), (hash40("param_dengekidama"), hash40("speed_"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_PICHU_DENGEKI, slots.clone(), (hash40("param_dengeki"), hash40("move_life_"), 124.0));
    param_config::update_float_2(-*WEAPON_KIND_PICHU_DENGEKI, slots.clone(), (hash40("param_dengeki"), hash40("move_rate_"), 2.4));
    param_config::update_float_2(-*WEAPON_KIND_PICHU_KAMINARI, slots.clone(), (hash40("param_kaminari"), hash40("speed_"), -5.9));
}