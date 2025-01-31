use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod gsword;

pub const FIGHTER_GANON_GENERATE_ARTICLE_GSWORD : i32 = 0x2;

pub const WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR: i32 = 0;

pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE: i32 = 0x1ED;
pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START: i32 = 0x1EE;
pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2: i32 = 0x1EF;

static mut SWORD: [bool; 8] = [true; 8];
static mut GROUND_CHECK: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    gsword::install();
}
