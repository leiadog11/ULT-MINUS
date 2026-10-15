use super::*;

pub mod acmd;
pub mod doyle;
pub mod frame;
pub mod status;
pub mod agent_init;

const FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH: i32 = 0x1EA;
const FIGHTER_JACK_STATUS_KIND_SPECIAL_S_SLIDE: i32 = 0x1EB;

static mut CURSE_TIMER: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    doyle::install();
    frame::install();
    status::install();
    agent_init::install();
}