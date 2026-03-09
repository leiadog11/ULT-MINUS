use super::*;

// OPFF
pub unsafe extern "C" fn zelda_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        
    }
}

// ON START
pub unsafe extern "C" fn zelda_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        
    }
}

pub fn install() {
    Agent::new("zelda")
        .on_line(Main, zelda_frame)
        .on_start(zelda_start)
        .install();
}