use super::*;

// BIKE FRAME!!!!
unsafe extern "C" fn bike_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);

        // TURN BIKE IN AIR
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            if lr == 1.0 && xpos < -0.5 { 
                macros::REVERSE_LR(fighter);
            }
            else if lr == -1.0 {
                if xpos > 0.5 {
                    macros::REVERSE_LR(fighter);
                }
            }
        }   

        // BIKE JUMP
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && 
        BIKE_JUMP[ENTRY_ID] != 0 {
            PostureModule::pos_y(fighter.module_accessor);
            for _ in 0..10 {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor) + 0.5});
            }
            BIKE_JUMP[ENTRY_ID] -= 1;
        }

        // BLOW UP ITEM
        if StatusModule::status_kind(fighter.module_accessor) == *WEAPON_WARIO_WARIOBIKE_STATUS_KIND_SPECIAL_S_ITEM {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                println!("BLOW UP BIKE!");
            }
        }
    }
}

pub fn install() {
    Agent::new("wario_wariobike")
        .on_line(Main, bike_frame)
        .install();
}