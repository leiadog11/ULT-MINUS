use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bowarrow;
pub mod chariot;

static mut SHIELD_LIFE: [i32; 8] = [0; 8];
static mut BREAK_WAIT_TIME: [i32; 8] = [0; 8];
static mut SHIELD_ON: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bowarrow::install();
    chariot::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_PIT, slots.clone(), (hash40("run_speed_max"), 0, 1.928));
    param_config::update_float_2(*FIGHTER_KIND_PIT, slots.clone(), (hash40("air_speed_x_stable"), 0, 1.135));

    // VL PARAMS
    param_config::update_float_2(-*WEAPON_KIND_PIT_BOWARROW, slots.clone(), (hash40("param_bowarrow"), hash40("speed"), 2.0));
    param_config::update_float_2(-*WEAPON_KIND_PIT_BOWARROW, slots.clone(), (hash40("param_bowarrow"), hash40("speed_up"), 4.5));
    param_config::update_float_2(-*WEAPON_KIND_PIT_BOWARROW, slots.clone(), (hash40("param_bowarrow"), hash40("control_angle"), 16.0));
    param_config::update_int_2(-*WEAPON_KIND_PIT_BOWARROW, slots.clone(), (hash40("param_bowarrow"), hash40("life"), 150));
    param_config::update_int_2(-*WEAPON_KIND_PIT_BOWARROW, slots.clone(), (hash40("param_bowarrow"), hash40("is_penetration"), 1));
}