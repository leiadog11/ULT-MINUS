use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod wickedweaveleg;
pub mod bat;
pub mod specialnbullet;

const FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_BAT: i32 = 0x1FA;
const FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_END: i32 = 0x1FB;
const WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_S4: i32 = 0x1EA;
const WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_HI4: i32 = 0x1EB;
const WEAPON_BAYONETTA_BAT_STATUS_KIND_SPECIAL_HI: i32 = 0x1EC;

static mut RECEIVED_FINAL_SMASH: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    wickedweaveleg::install();
    bat::install();
    specialnbullet::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("jump_speed_x"), 0, 1.25));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("jump_speed_x_max"), 0, 1.6));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("jump_initial_y"), 0, 31.45));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("jump_y"), 0, 49.0));
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("attack_combo_max"), 0, 1));
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("s3_combo_max"), 0, 1));
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("jump_count_max"), 0, 1));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("height"), 0, 9.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_front"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_front_max"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_back"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_h"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_air_front"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_air_back"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_air_h"), 0, 0.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("item_range_air_h"), 0, 0.0));
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("escape_air_landing_frame"), 0, 5));

    // VL PARAMS
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("param_private"), hash40("attack_air_f_combo_max"), 1));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("param_special_lw"), hash40("base_slow_frame_max"), 300.0));
    param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("param_special_lw"), hash40("base_slow_frame_min"), 300.0));
    param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, slots.clone(), (hash40("param_special_lw"), hash40("slow_frame_max"), 300));
}