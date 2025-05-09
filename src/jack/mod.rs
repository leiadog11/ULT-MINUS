use super::*;


pub mod acmd;
pub mod frame;
pub mod status;
pub mod fire;
pub mod fire2;
pub mod wing;

static mut CURSE_TIMER: [i32; 8] = [0; 8];


pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fire::install();
    fire2::install();
    wing::install();
}