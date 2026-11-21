use super::*;

mod acmd;
mod special_s;

pub fn install() {
    acmd::install();
    special_s::install();
}