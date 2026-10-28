use super::*;

pub mod acmd;
pub mod frame;
pub mod blaster_bullet;

pub fn install() {
    acmd::install();
    frame::install();
    blaster_bullet::install();
}