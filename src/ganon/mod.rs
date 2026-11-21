use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod gsword;

pub static mut FIGHTER_GANON_GENERATE_ARTICLE_GSWORD: i32 = 0;
pub static mut WEAPON_KIND_GANON_GSWORD: i32 = 0;

const WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR: i32 = 0;

const FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE: i32 = 0x1ED;
const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START: i32 = 0x1EE;
const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2: i32 = 0x1EF;

static mut SWORD: [bool; 8] = [true; 8];
static mut GROUND_CHECK: [bool; 8] = [false; 8];
static mut FORWARD_AMOUNT: [f32; 8] = [0.0; 8];
static mut UP_AMOUNT: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    gsword::install();

    let clone = smashline::clone_weapon("ganon", "gsword", *WEAPON_KIND_MARIO_FIREBALL, false);
    unsafe {
        FIGHTER_GANON_GENERATE_ARTICLE_GSWORD = clone.generate_id;
        WEAPON_KIND_GANON_GSWORD = clone.weapon_id;
    }

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_float_2(*FIGHTER_KIND_GANON, slots.clone(), (hash40("dash_speed"), 0, 1.97));
    param_config::update_float_2(*FIGHTER_KIND_GANON, slots.clone(), (hash40("run_speed_max"), 0, 1.74));
    param_config::update_float_2(*FIGHTER_KIND_GANON, slots.clone(), (hash40("jump_speed_x"), 0, 1.2));
    param_config::update_float_2(*FIGHTER_KIND_GANON, slots.clone(), (hash40("jump_y"), 0, 26.49));
    param_config::update_float_2(*FIGHTER_KIND_GANON, slots.clone(), (hash40("weight"), 0, 124.0));
}
