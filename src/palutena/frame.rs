use super::*;

// OPFF
pub unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

// ON START
pub unsafe extern "C" fn palutena_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("palutena")
        .on_line(Main, palutena_frame)
        .on_start(palutena_start)
        .install();
}