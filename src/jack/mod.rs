use super::*;

pub mod acmd;
pub mod doyle;
pub mod frame;
pub mod status;
pub mod agent_init;

const FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH: i32 = 0x1EA;
const FIGHTER_JACK_STATUS_KIND_SPECIAL_S_SLIDE: i32 = 0x1EB;

static mut CURSE_TIMER: [i32; 8] = [0; 8];
static mut HAS_FINAL_SMASH: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    doyle::install();
    frame::install();
    status::install();
    agent_init::install();

    let mut slots: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // FIGHTER PARAMS
    param_config::update_int_2(*FIGHTER_KIND_JACK, slots.clone(), (hash40("attack_combo_max"), 0, 1));
}