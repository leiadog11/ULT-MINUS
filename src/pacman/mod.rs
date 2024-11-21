use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bigpacman;
pub mod firehydrant;
pub mod trampoline;

const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH : i32 = 0x200000EC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_DOWN_SMASH : i32 = 0x200000ED;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE : i32 = 0x100000C9;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN : i32 = 0x100000CA;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN : i32 = 0x100000BD;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN : i32 = 0x100000BC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN : i32 = 0x100000BB;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN : i32 = 0x100000BA;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bigpacman::install();
    firehydrant::install();
    trampoline::install();
}