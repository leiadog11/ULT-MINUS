use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod parasail;

const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START : i32 = 0x1F1;
const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE : i32 = 0x1F2;
const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP : i32 = 0x1F3;
const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING : i32 = 0x1F4;

const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START : i32 = 0x1F5;
const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE : i32 = 0x1F6;
const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP : i32 = 0x1F7;

static mut UP_B_USED: [bool; 8] = [false; 8];
static mut EQUIPPED: [bool; 8] = [false; 8];
static mut MIPHAS_GRACE: [bool; 8] = [true; 8];
static mut DARUKS_PROTECTION: [bool; 8] = [true; 8];
static mut REVALIS_GALE: [bool; 8] = [true; 8];
static mut URBOSAS_FURY: [bool; 8] = [true; 8];
static mut DOWN_TILT_COUNT: [i32; 8] = [0; 8];
static mut FLOAT_TIMER: [i32; 8] = [0; 8];
static mut EFFECT_ON: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    parasail::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_LINK, slots.clone(), (hash40("run_speed_max"), 0, 1.934));
    param_config::update_float_2(*FIGHTER_KIND_LINK, slots.clone(), (hash40("run_accel_add"), 0, 0.046));
    param_config::update_float_2(*FIGHTER_KIND_LINK, slots.clone(), (hash40("dash_speed"), 0, 2.38));
    param_config::update_float_2(*FIGHTER_KIND_LINK, slots.clone(), (hash40("air_speed_x_stable"), 0, 1.224));
    param_config::update_int_2(*FIGHTER_KIND_LINK, slots.clone(), (hash40("s4_combo_max"), 0, 1));

    // VL PARAMS
    param_config::update_int_2(-*WEAPON_KIND_LINK_SWORD_BEAM, slots.clone(), (hash40("param_swordbeam"), hash40("min_life"), 52));
    param_config::update_int_2(-*WEAPON_KIND_LINK_SWORD_BEAM, slots.clone(), (hash40("param_swordbeam"), hash40("max_life"), 60));
}