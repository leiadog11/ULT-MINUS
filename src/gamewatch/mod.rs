use super::*;

pub mod acmd;
pub mod frame;
pub mod octopus;
pub mod status;
pub mod bomb;
pub mod breath;
pub mod rescue;

const WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACK_AIR_N: i32 = 0x6;
const WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACK_AIR_B: i32 = 0x7;
const WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACK_HI3: i32 = 0x8;

static mut BOMB_OUT: [bool; 8] = [false; 8];
static mut DAMAGE_ROT: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    bomb::install();
    frame::install();
    octopus::install();
    breath::install();
    rescue::install();
    status::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, slots.clone(), (hash40("jump_speed_x"), 0, 1.28));
    param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, slots.clone(), (hash40("air_speed_x_stable"), 0, 1.21));

    // VL PARAMS
    param_config::update_float_2(-*WEAPON_KIND_GAMEWATCH_BREATH, slots.clone(), (hash40("param_breath"), hash40("speed_y"), -8.5));
    param_config::update_int_2(-*WEAPON_KIND_GAMEWATCH_BOMB, slots.clone(), (hash40("param_bomb"), hash40("life"), 9000));
    param_config::update_float_2(-*WEAPON_KIND_GAMEWATCH_BOMB, slots.clone(), (hash40("param_bomb"), hash40("speed_y"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, slots.clone(), (hash40("param_special_lw"), hash40("air_speed_mul_x"), 1.5));
}