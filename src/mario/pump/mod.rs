use super::*;

pub mod acmd;
pub mod boost;

pub fn install() {
    acmd::install();
    boost::install();
}