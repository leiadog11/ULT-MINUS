use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod blaster_bullet;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    blaster_bullet::install();
}