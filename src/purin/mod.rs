use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut CHARGE_MUL: [f32; 8] = [0.0; 8];
static mut METRONOME: [u64; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_front"), 0, 200.0));
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_front_max"), 0, 200.0));
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_back"), 0, 200.0));
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_h"), 0, 200.0));
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_air_front"), 0, 200.0));
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_air_back"), 0, 200.0));
    param_config::update_float_2(*FIGHTER_KIND_PURIN, slots.clone(), (hash40("item_range_air_h"), 0, 200.0));
}
