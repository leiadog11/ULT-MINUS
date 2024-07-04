use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*,
    smashline::Priority::*
};

const FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD : i32 = 0x200000E2;
const FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_GROUND_CHECK : i32 = 0x200000E3;
const FIGHTER_GANON_GENERATE_ARTICLE_GSWORD : i32 = 0x2;
const WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR: i32 = 0;
const FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE: i32 = 0x1ED;
const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START: i32 = 0x1EE;
const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2: i32 = 0x1EF;
pub const SITUATION_KIND:                  i32 = 0x16;
pub const PREV_SITUATION_KIND:                  i32 = 0x17;

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
unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s4_s2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s4_hold2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw42") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw4_hold2") {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }   

        if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi2_start") ||
        MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi2") {
            VisibilityModule::set_whole(fighter.module_accessor, true);
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        }

        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
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

//START
unsafe extern "C" fn ganon_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD);
    }
}

//--------------------GROUND ATTACKS------------------------

//JAB
unsafe extern "C" fn ganon_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 11.0, 361, 74, 0, 41, 4.4, 0.0, 2.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 11.0, 361, 74, 0, 41, 5.0, 0.0, 2.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulder"), 11.0, 361, 74, 0, 41, 5.0, 10.0, 12.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 3.5, 0.0, 14.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
}

//DOWN TILT
unsafe extern "C" fn ganon_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 14.0, 90, 70, 0, 30, 3.0, 0.0, 0.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 14.0, 90, 70, 0, 30, 4.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 14.0, 120, 60, 0, 50, 4.8, 8.5, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//UP TILT
unsafe extern "C" fn ganon_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.812);
    frame(agent.lua_state_agent, 17.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 11.0, 285, 100, 0, 35, 6.0, 7.0, -1.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 11.0, 285, 90, 0, 22, 3.5, 7.0, -1.0, 0.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("legr"), 11.0, 285, 100, 0, 35, 4.0, 5.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("legr"), 11.0, 285, 100, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

unsafe extern "C" fn ganon_effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 12.5, 2, 11, -40, -84, 1.2, 0, 0, 0, 0, 0, 0, true, 0.65);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 11, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ganon_sound_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_attack_03"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_s"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

unsafe extern "C" fn ganon_expression_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//DASH ATTACK
unsafe extern "C" fn ganon_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 70, 85, 0, 50, 7.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 80, 60, 0, 45, 4.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.3);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//-----------------AERIALS------------------

//NAIR 
unsafe extern "C" fn ganon_attackairn(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_n2"), 0.0, 1.0, false, 0.0, false, false);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 55, 30, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.0, 70, 30, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 7.0, 55, 110, 0, 50, 4.3, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneer"), 7.0, 100, 30, 0, 20, 4.3, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 5.25, 55, 30, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.25, 70, 30, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 5.25, 55, 110, 0, 50, 4.3, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneer"), 5.25, 100, 30, 0, 20, 4.3, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 361, 106, 0, 25, 7.8, 6.5, 0.0, -3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 361, 106, 0, 25, 7.0, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 12.0, 361, 106, 0, 25, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 361, 106, 0, 25, 6.0, 6.0, 0.0, -3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 9.0, 361, 106, 0, 25, 5.3, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 9.0, 361, 106, 0, 25, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


//NAIR 2
unsafe extern "C" fn ganon_attackairn2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 45, 5.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 45, 5.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 45, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.5, 100, 80, 0, 45, 5.0, 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 7.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 7.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 7.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 7.0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 90, 70, 0, 42, 4.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 90, 70, 0, 42, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 90, 70, 0, 42, 4.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 6.0, 90, 70, 0, 42, 4.0, 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 7.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 7.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 7.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 7.0, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ganon_effect_attackairn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);    
    }
}

unsafe extern "C" fn ganon_sound_attackairn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_ll"));
    }
}

unsafe extern "C" fn ganon_expression_attackairn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

unsafe extern "C" fn ganon_landingairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//FAIR
unsafe extern "C" fn ganon_attackairf(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_f2"), 0.0, 1.0, false, 0.0, false, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 17.0, 361, 93, 0, 20, 4.0, -1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 18.0, 361, 93, 0, 20, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//FAIR 2
unsafe extern "C" fn ganon_attackairf2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 90, 0, 20, 3.3, 0.0, 7.5, 14.5, Some(0.0), Some(6.5), Some(14.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 60, 100, 0, 20, 3.3, 0.0, 7.5, 14.5, Some(0.0), Some(6.5), Some(14.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 45, 83, 0, 30, 7.8, 0.0, 9.5, 14.0, Some(0.0), Some(14.5), Some(14.6), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 13.0, 45, 83, 0, 30, 4.0, 0.0, 12.0, -2.0, Some(0.0), Some(4.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
        AttackModule::clear(agent.module_accessor, 2, false);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 11.0, 45, 83, 0, 30, 3.0, 0.0, 12.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn ganon_effect_attackairf2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);  
    }
}

unsafe extern "C" fn ganon_sound_attackairf2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_smash_l01"));
    }
}

unsafe extern "C" fn ganon_expression_attackairf2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

unsafe extern "C" fn ganon_landingairf(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("landing_air_f2"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn ganon_landingairf2(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


//BAIR
unsafe extern "C" fn ganon_attackairb(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_b2"), 0.0, 1.0, false, 0.0, false, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 86, 0, 40, 4.0, 0.0, 10.4, -10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.5, 361, 86, 0, 40, 4.5, 0.0, 9.1, -15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 17.0, 361, 86, 0, 40, 3.0, 0.0, 12.6, -7.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//BAIR 2
unsafe extern "C" fn ganon_attackairb2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.4);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 110, 0, 38, 7.0, 0.0, 8.5, -15.0, Some(0.0), Some(8.5), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 110, 0, 38, 7.0, 0.0, 16.0, -15.0, Some(0.0), Some(16.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ganon_effect_attackairb2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);  
    }
}

unsafe extern "C" fn ganon_sound_attackairb2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_smash_h01"));
    }
}

unsafe extern "C" fn ganon_expression_attackairb2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

unsafe extern "C" fn ganon_landingairb(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("landing_air_b2"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn ganon_landingairb2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//UP AIR
unsafe extern "C" fn ganon_attackairhi(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_hi2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 13.0, 361, 100, 0, 35, 4.8, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 361, 100, 0, 35, 5.8, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 12.0, 30, 80, 0, 30, 4.8, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 30, 80, 0, 30, 5.8, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 8.0, 0, 70, 0, 20, 4.8, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 0, 70, 0, 20, 5.8, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//UP AIR 2
unsafe extern "C" fn ganon_attackairhi2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 75, 105, 0, 32, 5.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 75, 105, 0, 32, 5.5, 0.0, 4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 75, 105, 0, 32, 5.5, 0.0, 8.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 12.0, 75, 105, 0, 32, 5.5, 0.0, 12.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ganon_effect_attackairhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);  
    }
}

unsafe extern "C" fn ganon_sound_attackairhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_smash_l02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
}

unsafe extern "C" fn ganon_expression_attackairhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

unsafe extern "C" fn ganon_landingairhi(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("landing_air_hi2"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn ganon_landingairhi2(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.5);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}


//DOWN AIR
unsafe extern "C" fn ganon_attackairlw(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_air_lw2"), 0.0, 1.0, false, 0.0, false, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 4.5, 4.5, 12.5, 0.0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 270, 100, 0, 20, 7.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 361, 100, 0, 20, 6.0, 0.0, 10.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 4.5, 4.5, 12.5, 11.0);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//DOWN AIR 2
unsafe extern "C" fn ganon_attackairlw2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 9.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 3.0, 0.0, 0.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -1.0, 7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -2.0, 11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -3.0, 14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 3.0, 0.0, -2.0, -2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -7.0, -1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -10.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -13.0, 1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 3.0, 0.0, 2.0, -3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 1.0, -11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -2.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -5.0, -14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -8.0, -15.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 4.0, -7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 6.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, 5.0, -16.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, 4.0, -19.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, 3.0, -22.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 8.0, -8.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ganon_effect_attackairlw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 7, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);  
    }
}

unsafe extern "C" fn ganon_sound_attackairlw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_smash_s01"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
}

unsafe extern "C" fn ganon_expression_attackairlw2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 7);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl_l"), 18, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ganon_landingairlw(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("landing_air_lw2"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn ganon_landingairlw2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//---------------SMASH ATTACKS-----------------

//FORWARD SMASH CHARGE
unsafe extern "C" fn ganon_attacks4charge(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_s4_hold2"), 0.0, 1.0, false, 0.0, false, false);
    }
}


//FORWARD SMASH 2 CHARGE
unsafe extern "C" fn ganon_effect_attacks4charge2(agent: &mut L2CAgentBase) {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
    }
}

//FORWARD SMASH
unsafe extern "C" fn ganon_attacks4(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 19.0, 40, 65, 0, 41, 4.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 19.0, 40, 65, 0, 41, 4.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 19.0, 40, 65, 0, 41, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 19.0, 40, 65, 0, 41, 4.5, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 19.0, 40, 65, 0, 41, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//FORWARD SMASH 2
unsafe extern "C" fn ganon_attacks42(agent: &mut L2CAgentBase) {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 53, 55, 0, 46, 5.8, 2.2, 0.7, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 20.0, 53, 55, 0, 46, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 65.0);
}

unsafe extern "C" fn ganon_effect_attacks42(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("arml"), 0, 12, 18.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ganon_sound_attacks42(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_special_h02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smashswing_03"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn ganon_expression_attacks42(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    execute(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//DOWN SMASH CHARGE
unsafe extern "C" fn ganon_attacklw4charge(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 4.0);
    if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_lw4_hold2"), 0.0, 1.0, false, 0.0, false, false);
    }
}

//DOWN SMASH CHARGE 2
unsafe extern "C" fn ganon_effect_attacklw4charge2(agent: &mut L2CAgentBase) {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    macros::FT_MOTION_RATE(agent, 4.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
    }
}

//DOWN SMASH
unsafe extern "C" fn ganon_attacklw4(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("attack_lw42"), 0.0, 1.0, false, 0.0, false, false);
    }
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 160, 90, 90, 0, 3.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 175, 92, 95, 0, 3.0, 0.0, 4.0, 19.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 147, 90, 90, 0, 4.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 150, 92, 95, 0, 4.0, 0.0, 4.0, 19.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 35, 81, 0, 61, 5.0, 0.0, 4.8, -21.0, Some(0.0), Some(4.8), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

//DOWN SMASH 2
unsafe extern "C" fn ganon_attacklw42(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    execute(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 25, 87, 0, 30, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 12.0, 25, 87, 0, 30, 3.3, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 14.0, 20, 88, 0, 30, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 14.0, 20, 88, 0, 30, 3.3, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.7);
}

unsafe extern "C" fn ganon_effect_attacklw42(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13, -1, 35, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, 15, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 12.5, 0, 35, -180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, -15, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn ganon_sound_attacklw42(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_attack07"));
    }
}

unsafe extern "C" fn ganon_expression_attacklw42(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 25);
    }
}


//-----------------SPECIALS-------------------

//NEUTRAL B
unsafe extern "C" fn ganon_specialn(agent: &mut L2CAgentBase) {
    let facing = PostureModule::lr(agent.module_accessor);
    if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        if facing == 1.0 {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_lw_l"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_GANON_GENERATE_ARTICLE_GSWORD, false, -1);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD);
    }
}

unsafe extern "C" fn ganon_effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        
    }
}

unsafe extern "C" fn ganon_sound_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        
    }
}

unsafe extern "C" fn ganon_expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {

    }
}

//SIDE B START
unsafe extern "C" fn ganon_specialsstart(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 6.8, 6.7, 7.5, 3.8);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 7.0, 7.5, 7.5);
    }
}

//SIDE B
unsafe extern "C" fn ganon_specials(agent: &mut L2CAgentBase) {
    let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        if opponent_boma == agent.module_accessor {
            opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 361, 90, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_bury_r"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_bury_r"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 31.0); 
    if macros::is_excute(agent) {
        DamageModule::add_damage(opponent_boma, 5.0, 0);
        DamageModule::add_damage(agent.module_accessor, -5.0, 0);
    }
    frame(agent.lua_state_agent, 38.0); 
    if macros::is_excute(agent) {
        DamageModule::add_damage(opponent_boma, 5.0, 0);
        DamageModule::add_damage(agent.module_accessor, -5.0, 0);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        DamageModule::add_damage(agent.module_accessor, -5.0, 0);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_FALL);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

unsafe extern "C" fn ganon_effect_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn ganon_sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ganon_special_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s03"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn ganon_expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AERIAL SIDE B
unsafe extern "C" fn ganon_specialairs(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.0, 361, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::SLOW_OPPONENT(agent, 3.0, 40.0);
    }
}

//DOWN B
unsafe extern "C" fn ganon_speciallw(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("kneer"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 6.0, 8.5, 9.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 2.0, 6.0, 8.5, 10.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 45, 65, 0, 65, 3.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0, 45, 65, 0, 65, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
}

//DOWN B CONTINUE
unsafe extern "C" fn ganon_speciallwhold(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("kneer"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    macros::SET_SPEED_EX(agent, 2.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 45, 65, 0, 65, 3.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0, 45, 65, 0, 65, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
}

unsafe extern "C" fn ganon_effect_speciallwhold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneer"), 12, -1.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        macros::FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ganon_expression_speciallwhold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//UP B 2
unsafe extern "C" fn ganon_sound_specialhi2_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_assist_vanish"));
    }
}

unsafe extern "C" fn ganon_effect_specialhi2_start(agent: &mut L2CAgentBase) { 
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn ganon_sound_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

unsafe extern "C" fn ganon_effect_specialhi2(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

    }
}

//---------------GRABS--------------------

//GRAB
unsafe extern "C" fn ganon_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("handl"), 4.0, 1.0, 8.0, 4.0, Some(9.0), Some(9.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//DASH GRAB
unsafe extern "C" fn ganon_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("handl"), 4.0, 1.0, 8.0, 4.0, Some(9.0), Some(9.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//PIVOT GRAB
unsafe extern "C" fn ganon_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("handl"), 4.0, 1.0, 8.0, 4.0, Some(9.0), Some(9.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}


//-----------------THROWS-----------------------

//UP THROW
unsafe extern "C" fn ganon_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 90, 145, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 10.0, 140, 100, 0, 30, 4.3, 4.8, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 10.0, 140, 100, 0, 30, 3.8, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 10.0, 140, 100, 0, 30, 3.7, -0.8, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, -1, 28);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 4.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//BACK THROW
unsafe extern "C" fn ganon_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 70, 35, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footr"), 5.0, 70, 100, 0, 30, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.0, 70, 100, 0, 30, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("legr"), 5.0, 70, 100, 0, 30, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 22, 20);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 2.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.7);
}

//FORWARD THROW
unsafe extern "C" fn ganon_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 10, 50, 0, 48, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 5.0, 80, 100, 0, 30, 5.0, 4.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 5.0, 80, 100, 0, 30, 2.4, 2.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 5.0, 80, 100, 0, 30, 2.4, -0.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 17, 9);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//DOWN THROW
unsafe extern "C" fn ganon_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 66, 30, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 11, 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//--------------TAUNTS-----------------

//DOWN TAUNT
unsafe extern "C" fn ganon_appeallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD);
    }

}

//UP TAUNT
unsafe extern "C" fn ganon_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) { 
        for _ in 0..15 {
            macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 0.5, 361, 30, 0, 20, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_KICK);
            wait(agent.lua_state_agent, 2.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) { 
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 55, 60, 0, 60, 9.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

//----------------PROJECTILES----------------------

//GSWORD - Regular
unsafe extern "C" fn ganon_gsword_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 68, 20, 0, 35, 2.4, 0.0, 14.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 5, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn ganon_gsword_effect_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_trace"), Hash40::new("haver"), 0, 1.5, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("tex_ganon_sword1"), Hash40::new("haver"), 0, 1.5, 0, 0, 0, 0, 1, true);
    }
}


//-------------------STATUS--------------------

/////GSWORD - REGULAR

//PRE
unsafe extern "C" fn ganon_gsword_regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

//MAIN
unsafe extern "C" fn ganon_gsword_regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let life = 300;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    weapon.fastshift(L2CValue::Ptr(ganon_gsword_regular_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn ganon_gsword_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let facing = PostureModule::lr(weapon.module_accessor);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;

    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);

    // Declare acceleration and max speed
    speed_x = if facing == 1.0 {0.5} else {-0.5};
    let accel_y: f32 = -0.0054; // Adjusted for controlled y movement
    let speed_max_y: f32 = 1.0; // Adjusted max speed for y movement

     // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();

    if status_frame == 1.0 {
        speed_y = 0.5;
    }

    speed_y += accel_y;
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: PostureModule::rot_x(weapon.module_accessor, 0) + 10.0, y: 0.0, z: 0.0}, 0);


    // Set speed
    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);


    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        gsword_remove(weapon);
        return 0.into();
    }

    if weapon.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
        gsword_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn gsword_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}


/////NEUTRAL B

//INIT
unsafe extern "C" fn ganon_specialn_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//PRE
unsafe extern "C" fn ganon_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_NONE,
            *GROUND_CORRECT_KIND_KEEP as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            (
                *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
                *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
                *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
            ) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
            0
        );

    return 0.into();
}

//MAIN
unsafe extern "C" fn ganon_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialn_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn ganon_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::frame(fighter.module_accessor) >= 43.0 {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
            return 1.into();
        }   
    }

    return 0.into();
}

//END
unsafe extern "C" fn ganon_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}


///////////DOWN B 

//PRE
unsafe extern "C" fn ganon_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

//MAIN
unsafe extern "C" fn ganon_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_CLIFF_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_FALL_ONOFF);

    let ganon_kick_hit_reduct_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("ganon_kick_hit_reduct_count")); 
    WorkModule::set_int(fighter.module_accessor, ganon_kick_hit_reduct_count, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_REDUCTION_LEFT);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_SPEED_COEFFICIENT);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_END_SITUATION);

    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);

        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);

        let lr = PostureModule::lr(fighter.module_accessor); 
        WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_START_LR);

        let ganon_kick_speed_coeff = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("ganon_kick_speed_coeff")); 
        WorkModule::set_float(fighter.module_accessor, ganon_kick_speed_coeff, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_speciallw_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn ganon_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {  
    if MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE.into(), false.into());
            return 1.into();
        }
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) &&
       MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_lw") {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1.into();
    }
    
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION); 
    
    if start_situation != *SITUATION_KIND_GROUND {
        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_DOWN).try_into().unwrap()); 
    
        if is_touch {
        let curr_frame = MotionModule::frame(fighter.module_accessor); 
            if curr_frame != 2.0 {
              WorkModule::set_int(fighter.module_accessor, *FIGHTER_GANON_KICK_END_SITUATION_AG, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_END_SITUATION);
              fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
              return 1.into(); 
            }
          return 0.into();
        }
    }
    else {
        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT).try_into().unwrap()); 
        if is_touch {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
                return 1.into(); 
            } 
        }

        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT).try_into().unwrap()); 
        if is_touch {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
                return 1.into(); 
            }
        } 
    }
    
        if StatusModule::is_changing(fighter.module_accessor) {
          if fighter.global_table[PREV_SITUATION_KIND] != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
          }
    
          if fighter.global_table[PREV_SITUATION_KIND] == *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                return 0.into();
            } 
          }
        }
        else {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
    
    return 0.into();
}

//END
unsafe extern "C" fn ganon_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {  
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    WorkModule::set_float(fighter.module_accessor, sum_speed_x, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_SPEED);
    return 0.into();
}


/////////DOWN B CONTINUE

//PRE
unsafe extern "C" fn ganon_speciallw_continue_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

//MAIN
unsafe extern "C" fn ganon_speciallw_continue_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_speciallw_continue_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn ganon_speciallw_continue_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE.into(), false.into());
        return 1.into();
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1.into();
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        return 1.into();
    }

    let ypos = ControlModule::get_stick_y(fighter.module_accessor);
    
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR &&
    ypos < -0.5 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

//END
unsafe extern "C" fn ganon_speciallw_continue_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}


/////////////SPECIAL HI
unsafe extern "C" fn ganon_specialhi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_GROUND_CHECK) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START.into(), false.into());
        return 1.into();
    }
    return 0.into();
}



/////////////SPECIAL HI 2 START

//PRE
unsafe extern "C" fn ganon_specialhi2_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

//MAIN
unsafe extern "C" fn ganon_specialhi2_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2_start"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialhi2_start_main_loop as *const () as _));
    return 0.into();
}


//MAIN LOOP
unsafe extern "C" fn ganon_specialhi2_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) { 
        VisibilityModule::set_whole(fighter.module_accessor, false);
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

//END
unsafe extern "C" fn ganon_specialhi2_start_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}



/////////SPECIAL HI 2

//PRE
unsafe extern "C" fn ganon_specialhi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

//MAIN
unsafe extern "C" fn ganon_specialhi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_GROUND_CHECK);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialhi2_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn ganon_specialhi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut stick_choice = 0;

    //right
    if stick_x > 0.5 && stick_y == 0.0 {
        stick_choice = 1;
    }

    //up right
    if stick_x > 0.3 && stick_y > 0.3 {
        stick_choice = 2;
    }

    //up left
    if stick_x < -0.3 && stick_y > 0.3 {
        stick_choice = 3;
    }

    //left
    if stick_x < -0.5 && stick_y == 0.0 {
        stick_choice = 4;
    }

    //down left
    if stick_x < -0.3 && stick_y < -0.3 {
        stick_choice = 5;
    }

    //down
    if stick_x == 0.0 && stick_y < -0.5 {
        stick_choice = 6;
    }

    //down right
    if stick_x > 0.3 && stick_y < -0.3 {
        stick_choice = 7;
    }

    if MotionModule::frame(fighter.module_accessor) == 6.0 {
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let lr = PostureModule::lr(fighter.module_accessor);
        //up
        if stick_choice == 0 {
            macros::SET_SPEED_EX(fighter, x_vel, y_vel + 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //right
        else if stick_choice == 1 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel - 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //up right
        else if stick_choice == 2 {
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //up left
        else if stick_choice == 3 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel - 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //left
        else if stick_choice == 4 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel - 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //down left
        else if stick_choice == 5 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel - 1.3, x_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel + 1.3, x_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //down
        else if stick_choice == 6 {
            macros::SET_SPEED_EX(fighter, x_vel, y_vel - 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //down right
        else if stick_choice == 7 {
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        return 0.into();
    }
    if MotionModule::frame(fighter.module_accessor) >= 7.0 {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        GroundModule::set_passable_check(fighter.module_accessor, false);
        KineticModule::resume_energy_all(fighter.module_accessor);

        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        return 0.into();
    }
    return 0.into();
}

//END
unsafe extern "C" fn ganon_specialhi2_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
      }
      else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
      }
    return 0.into();
}


pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attack11", ganon_attack11, Low)
        .game_acmd("game_attacklw3", ganon_attacklw3, Low)
        .game_acmd("game_attackhi3", ganon_attackhi3, Low)
        .game_acmd("game_attackdash", ganon_attackdash, Low)
        .game_acmd("game_attacks4charge", ganon_attacks4charge, Low)
        .game_acmd("game_attacks4", ganon_attacks4, Low)
        .game_acmd("game_attacks42", ganon_attacks42, Low)
        .game_acmd("game_attacklw4", ganon_attacklw4, Low)
        .game_acmd("game_attacklw4charge", ganon_attacklw4charge, Low)
        .game_acmd("game_attacklw42", ganon_attacklw42, Low)
        .game_acmd("game_attackairn", ganon_attackairn, Low)
        .game_acmd("game_attackairn2", ganon_attackairn2, Low)
        .game_acmd("game_attackairf", ganon_attackairf, Low)
        .game_acmd("game_attackairf2", ganon_attackairf2, Low)
        .game_acmd("game_attackairb", ganon_attackairb, Low)
        .game_acmd("game_attackairb2", ganon_attackairb2, Low)
        .game_acmd("game_attackairhi", ganon_attackairhi, Low)
        .game_acmd("game_attackairhi2", ganon_attackairhi2, Low)
        .game_acmd("game_attackairlw", ganon_attackairlw, Low)
        .game_acmd("game_attackairlw2", ganon_attackairlw2, Low)
        .game_acmd("game_landingairn", ganon_landingairn, Low)
        .game_acmd("game_landingairb", ganon_landingairb, Low)
        .game_acmd("game_landingairf", ganon_landingairf, Low)
        .game_acmd("game_landingairhi", ganon_landingairhi, Low)
        .game_acmd("game_landingairlw", ganon_landingairlw, Low)
        .game_acmd("game_landingairb2", ganon_landingairb2, Low)
        .game_acmd("game_landingairf2", ganon_landingairf2, Low)
        .game_acmd("game_landingairhi2", ganon_landingairhi2, Low)
        .game_acmd("game_landingairlw2", ganon_landingairlw2, Low)
        .game_acmd("game_specialn", ganon_specialn, Low)
        .game_acmd("game_specialsstart", ganon_specialsstart, Low)
        .game_acmd("game_specials", ganon_specials, Low)
        .game_acmd("game_specialairs", ganon_specialairs, Low)
        .game_acmd("game_speciallw", ganon_speciallw, Low)
        .game_acmd("game_speciallwhold", ganon_speciallwhold, Low)
        .game_acmd("game_catch", ganon_catch, Low)
        .game_acmd("game_catchdash", ganon_catchdash, Low)
        .game_acmd("game_catchturn", ganon_catchturn, Low)
        .game_acmd("game_throwlw", ganon_throwlw, Low)
        .game_acmd("game_throwf", ganon_throwf, Low)
        .game_acmd("game_throwb", ganon_throwb, Low)
        .game_acmd("game_throwhi", ganon_throwhi, Low)
        .game_acmd("game_appeallwl", ganon_appeallw, Low)
        .game_acmd("game_appeallwr", ganon_appeallw, Low)
        .game_acmd("game_appealhir", ganon_appealhi, Low)
        .game_acmd("game_appealhil", ganon_appealhi, Low)
        .effect_acmd("effect_attacks42charge2", ganon_effect_attacks4charge2, Low)
        .effect_acmd("effect_attacks42", ganon_effect_attacks42, Low)
        .effect_acmd("effect_attacklw4charge2", ganon_effect_attacklw4charge2, Low)
        .effect_acmd("effect_attacklw42", ganon_effect_attacklw42, Low)
        .effect_acmd("effect_attackairn2", ganon_effect_attackairn2, Low)
        .effect_acmd("effect_attackhi3", ganon_effect_attackhi3, Low)
        .effect_acmd("effect_attackairf2", ganon_effect_attackairf2, Low)
        .effect_acmd("effect_attackairb2", ganon_effect_attackairb2, Low)
        .effect_acmd("effect_attackairhi2", ganon_effect_attackairhi2, Low)
        .effect_acmd("effect_attackairlw2", ganon_effect_attackairlw2, Low)
        .effect_acmd("effect_specialn", ganon_effect_specialn, Low)
        .effect_acmd("effect_speciallwhold", ganon_effect_speciallwhold, Low)
        .effect_acmd("effect_specials", ganon_effect_specials, Low)
        .effect_acmd("effect_specialhi2_start", ganon_effect_specialhi2_start, Low)
        .effect_acmd("effect_specialhi2", ganon_effect_specialhi2, Low)
        .sound_acmd("sound_attacks42", ganon_sound_attacks42, Low)
        .sound_acmd("sound_attacklw42", ganon_sound_attacklw42, Low)
        .sound_acmd("sound_attackairn2", ganon_sound_attackairn2, Low)
        .sound_acmd("sound_attackhi3", ganon_sound_attackhi3, Low)
        .sound_acmd("sound_attackairf2", ganon_sound_attackairf2, Low)
        .sound_acmd("sound_attackairb2", ganon_sound_attackairb2, Low)
        .sound_acmd("sound_attackairhi2", ganon_sound_attackairhi2, Low)
        .sound_acmd("sound_attackairlw2", ganon_sound_attackairlw2, Low)
        .sound_acmd("sound_specialn", ganon_sound_specialn, Low)
        .sound_acmd("sound_specials", ganon_sound_specials, Low)
        .sound_acmd("sound_specialhi2_start", ganon_sound_specialhi2_start, Low)
        .sound_acmd("sound_specialhi2", ganon_sound_specialhi2, Low)
        .expression_acmd("expression_attacks42", ganon_expression_attacks42, Low)
        .expression_acmd("expression_attacklw42", ganon_expression_attacklw42, Low)
        .expression_acmd("expression_attackhi3", ganon_expression_attackhi3, Low)
        .expression_acmd("expression_attackairn2", ganon_expression_attackairn2, Low)
        .expression_acmd("expression_attackairf2", ganon_expression_attackairf2, Low)
        .expression_acmd("expression_attackairb2", ganon_expression_attackairb2, Low)
        .expression_acmd("expression_attackairhi2", ganon_expression_attackairhi2, Low)
        .expression_acmd("expression_attackairlw2", ganon_expression_attackairlw2, Low)
        .expression_acmd("expression_specialn", ganon_expression_specialn, Low)
        .expression_acmd("expression_speciallwhold", ganon_expression_speciallwhold, Low)
        .expression_acmd("expression_specials", ganon_expression_specials, Low)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, ganon_specialn_init)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, ganon_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, ganon_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, ganon_specialn_end)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_end)
        .status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, ganon_speciallw_continue_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, ganon_speciallw_continue_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, ganon_speciallw_continue_end)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_specialhi_init)
        .status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, ganon_specialhi2_start_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, ganon_specialhi2_start_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, ganon_specialhi2_start_end)
        .status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2, ganon_specialhi2_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2, ganon_specialhi2_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2, ganon_specialhi2_end)
        .on_line(Main, ganon_frame)
        .on_start(ganon_start)
        .install();
    Agent::new("ganon_gsword")
        .game_acmd("game_regular", ganon_gsword_regular, Low)
        .effect_acmd("effect_regular", ganon_gsword_effect_regular, Low)
        .status(Pre, WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR, ganon_gsword_regular_pre)
        .status(Main, WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR, ganon_gsword_regular_main)
        .install();
}
