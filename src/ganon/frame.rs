use super::*;

static mut FLOAT : [i32; 8] = [0; 8]; 
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut JUMPSQUAT_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 15; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 60; //Frames this bitch can float (In frames, 60 frames = 1 second)
static mut CAN_DOUBLE_JUMP: [i32; 8] = [0; 8];
static mut X_MAX : f32 = 1.208; //Max Horizontal movespeed
static mut X_ACCEL_ADD : f32 = 0.02; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 0.0; //Max Vertical movespeed
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.06;

// OPFF
pub unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

        if motion_kind == hash40("attack_s4_s2") || motion_kind == hash40("attack_s4_hold2") || 
        motion_kind == hash40("attack_lw42") || motion_kind == hash40("attack_lw4_hold2") {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }   

        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        if motion_kind != hash40("special_hi2_start") || motion_kind != hash40("special_hi2") {
            VisibilityModule::set_whole(fighter.module_accessor, true);
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        }

        // GROUND CHECK FOR UP B 2
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_GROUND_CHECK);
        }

        //FLOAT
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
		let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
		let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

        if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_AIR {
            FLOAT[ENTRY_ID] = 0;
            START_FLOAT[ENTRY_ID] = false;
            CHECK_FLOAT[ENTRY_ID] = 0;
        };
        //Effect Code
        if FLOAT[ENTRY_ID] % 5 == 0 && FLOAT[ENTRY_ID] > 1{
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, true);
        };
        //Float Code
        if FLOAT[ENTRY_ID] == 1{
            if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            };
        };
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) && ControlModule::get_stick_y(fighter.module_accessor) < -0.5 {
            CAN_DOUBLE_JUMP[ENTRY_ID] = 1;
        } else {
            CAN_DOUBLE_JUMP[ENTRY_ID] = 0;
        }
        if situation_kind == *SITUATION_KIND_AIR && (!(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && status_kind != *FIGHTER_STATUS_KIND_FALL_SPECIAL){
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && stick_y < -0.5 {
                CHECK_FLOAT[ENTRY_ID] += 1;
            } else {
                CHECK_FLOAT[ENTRY_ID] = 0;
            };
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && stick_y < -0.5 {
                CHECK_FLOAT[ENTRY_ID] = CHECK_FLOAT_MAX;
            };
            if (CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX || JUMPSQUAT_FLOAT[ENTRY_ID]) && FLOAT[ENTRY_ID] == 0 {
                START_FLOAT[ENTRY_ID] = true;
            };
        };
        if status_kind == *FIGHTER_STATUS_KIND_JUMP && JUMPSQUAT_FLOAT[ENTRY_ID] {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        };
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            JUMPSQUAT_FLOAT[ENTRY_ID] = false;
        }
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::get_stick_y(fighter.module_accessor) < -0.5 {
                JUMPSQUAT_FLOAT[ENTRY_ID] = true;
                WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            } else {
                JUMPSQUAT_FLOAT[ENTRY_ID] = false;
                CHECK_FLOAT[ENTRY_ID] = 0;
            };
        } else {
            JUMPSQUAT_FLOAT[ENTRY_ID] = false;
        };
        if [
            *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_SPECIAL_N, 
            *FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END,
            FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2
        ].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
            FLOAT[ENTRY_ID] = 1;
        };
        if FLOAT[ENTRY_ID] > 1 {
            FLOAT[ENTRY_ID] -= 1;
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            };
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                FLOAT[ENTRY_ID] = 1;
            };
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                FLOAT[ENTRY_ID] = 1;
            };
            let mut y_add = 0.0;
            let mut x_add = 0.0;
            if stick_x > 0.2 {
                x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
                if speed_x > X_MAX || speed_x < -X_MAX{{}
                    x_add = 0.0;
                };
            };
            if stick_x < -0.2 {
                x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
                if speed_x > X_MAX || speed_x < -X_MAX{
                    x_add = 0.0;
                };
            };
            if stick_y > 0.2 {
                y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
                if speed_y > Y_MAX || speed_y < -Y_MAX{
                    y_add = 0.0;
                };
            };
            if stick_y < -0.2 {
                y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
                if speed_y > Y_MAX || speed_y < -Y_MAX{
                    y_add = 0.0;
                };
            };
            if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
                if speed_y > 0.0 {
                    y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
                } else if speed_y < 0.0{
                    y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
                };
                let mut x_add = 0.0;
                if speed_x > 0.0 {
                    x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
                } else if speed_x < 0.0{
                    x_add = X_ACCEL_MUL + X_ACCEL_ADD;
                };
            };
            x_add = (stick_x)*X_ACCEL_MUL;
            y_add = (stick_y)*X_ACCEL_MUL;
            if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
                x_add = 0.0;
            };
            if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
                x_add = 0.0;
            };
            if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                y_add = 0.0;
            };
            if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
                y_add = 0.0;
            };
            X[ENTRY_ID] += x_add;
            Y[ENTRY_ID] += y_add;
            macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID]+0.075, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        } else {
            X[ENTRY_ID] = 0.0;
            Y[ENTRY_ID] = 0.0;
            KineticModule::resume_energy_all(fighter.module_accessor);
        };
        if START_FLOAT[ENTRY_ID] == true {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_GROUND_CHECK);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ganon_appear01"), true, false, false, false, enSEType(0));
            FLOAT[ENTRY_ID] = FLOAT_MAX;
            START_FLOAT[ENTRY_ID] = false;
            ControlModule::clear_command(fighter.module_accessor, false);
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        };
    }
}

// ON START
pub unsafe extern "C" fn ganon_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD);
    }
}

pub fn install() {
    Agent::new("ganon")
        .on_line(Main, ganon_frame)
        .on_start(ganon_start)
        .install();
}