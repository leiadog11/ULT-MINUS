use super::*;

mod attackdash;
mod checkdamage;
mod specialn;

pub fn install() {
    attackdash::install();
    checkdamage::install();
    specialn::install();
}