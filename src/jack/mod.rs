use super::*;

pub mod acmd;
pub mod doyle;
pub mod frame;
pub mod status;
pub mod agent_init;

static mut CURSE_TIMER: [i32; 8] = [0; 8];
static mut MOMENTUM: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    doyle::install();
    frame::install();
    status::install();
    agent_init::install();
}