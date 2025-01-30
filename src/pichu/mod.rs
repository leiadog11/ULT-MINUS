use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod dengekidama;
pub mod kaminari;
pub mod cloud;

static mut BLOWN_UP: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    dengekidama::install();
    kaminari::install();
    cloud::install();
}
