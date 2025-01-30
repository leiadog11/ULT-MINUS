use super::*;

//////////////// SPECIAL N

// PRE
unsafe extern "C" fn pacman_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_ATTR_START_TURN,
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

// MAIN
unsafe extern "C" fn pacman_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false); 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
      
      fighter.fastshift(L2CValue::Ptr(pacman_specialn_main_loop as *const () as _ ));
      return 0.into()
}

// MAIN LOOP
unsafe extern "C" fn pacman_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }  

        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, true); 
        fighter.fastshift(L2CValue::Ptr(pacman_specialn_end as *const () as _ ));
        return 0.into()
    }

    return 0.into();
}

// END
unsafe extern "C" fn pacman_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

///////////// SPECIAL N HOLD

// PRE
unsafe extern "C" fn pacman_specialn_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_ATTR_START_TURN,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn pacman_specialn_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), true);
    ITEM_CHOICE[get_entry_id(fighter.module_accessor)] = 0;

    fighter.fastshift(L2CValue::Ptr(pacman_specialn_hold_main_loop as *const () as _ ));
    return 0.into()
}

// MAIN LOOP
unsafe extern "C" fn pacman_specialn_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    let xpos = ControlModule::get_stick_x(fighter.module_accessor);
    let ypos = ControlModule::get_stick_y(fighter.module_accessor);
    let selected_scale = Vector3f{x: 1.5, y: 1.5, z: 1.0};
    let cooldown_scale = Vector3f{x: 0.3, y: 0.3, z: 1.0};
    let left_position = Vector3f{ x:-30.0, y: 10.0, z: 0.0 };
    let facing = PostureModule::lr(fighter.module_accessor);
    if facing == 1.0 {
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("ghost1"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    else {
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("ghost1"), &Vector3f{x: -28.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("ghost1"), &left_position, false, false);
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("bell"), &Vector3f{ x:-7.5, y: 22.0, z: 15.0 }, false, false);
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("apple"), &Vector3f{ x:11.0, y: -20.0, z: 0.0 }, false, false);
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("melon"), &Vector3f{ x:12.0, y: -18.0, z: 0.0 }, false, false);
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("galaxian"), &Vector3f{ x:-2.0, y: 15.0, z: 5.0 }, false, false);
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
            }   
        }

        //key
        if KEY_COOLDOWN[ENTRY_ID] != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), false);
        }
        else {
            if xpos == 0.0 && ypos > 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("key"), &selected_scale);
                if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_pacman_special_n08")) {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pacman_special_n08"), true, false, false, false, enSEType(0));
                }
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);
                ITEM_CHOICE[ENTRY_ID] = 1;
            }
        }

        //apple
        if APPLE_COOLDOWN[ENTRY_ID] != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), false);
        }
        else {
            if xpos > 0.0 && ypos > 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("apple"), &selected_scale);
                if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_pacman_special_n04")) {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pacman_special_n04"), true, false, false, false, enSEType(0));
                }
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);
                ITEM_CHOICE[ENTRY_ID] = 2;
            }
        }

        //melon
        if MELON_COOLDOWN[ENTRY_ID] != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), false);       
        }
        else {
            if xpos > 0.0 && ypos < 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("melon"), &selected_scale);
                if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_pacman_special_n05")) {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pacman_special_n05"), true, false, false, false, enSEType(0));
                }
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);
                ITEM_CHOICE[ENTRY_ID] = 3;
            }
        }

        //galaxian
        if GALAXIAN_COOLDOWN[ENTRY_ID] != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), false);       
        }
        else {
            if xpos < 0.0 && ypos < 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("galaxian"), &selected_scale);
                if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_pacman_special_n06")) {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pacman_special_n06"), true, false, false, false, enSEType(0));
                }
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);
                ITEM_CHOICE[ENTRY_ID] = 4;
            }
        }

        //bell
        if BELL_COOLDOWN[ENTRY_ID] != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), false);        
        }
        else {
            if xpos < 0.0 && ypos > 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("bell"), &selected_scale);
                if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_pacman_special_n07")) {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pacman_special_n07"), true, false, false, false, enSEType(0));
                }
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);
                ITEM_CHOICE[ENTRY_ID] = 5;
            }
        }

        if xpos == 0.0 && ypos == 0.0 {
            ITEM_CHOICE[ENTRY_ID] = 0;
        }
    }
    else {
        //key
        if  ITEM_CHOICE[ENTRY_ID] == 1 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);
            KEY_COOLDOWN[ENTRY_ID] = 180;
        }

        //apple
        else if ITEM_CHOICE[ENTRY_ID] == 2 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
            APPLE_COOLDOWN[ENTRY_ID] = 90;
        }

        //melon
        else if ITEM_CHOICE[ENTRY_ID] == 3 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
            MELON_COOLDOWN[ENTRY_ID] = 90;
        }

        //galaxian
        else if ITEM_CHOICE[ENTRY_ID] == 4 {
            let rand = smash::app::sv_math::rand(hash40("fighter"), 12) as u64;
            if rand != 1 {
                ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
            }
            else {
                ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BOSSGALAGA), 0, 0, false, false);
            }
            GALAXIAN_COOLDOWN[ENTRY_ID] = 120;
        }

        //bell
        else if ITEM_CHOICE[ENTRY_ID] == 5 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
            BELL_COOLDOWN[ENTRY_ID] = 210;
        }

        else {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                fighter.change_status((*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL).into(), false.into());
                return 1.into()
            }
            else {
                fighter.change_status((*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL).into(), false.into());
                return 1.into()
            }   
        }

        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
            return 1.into();
        }   
    }
    return 0.into()
}

// END
unsafe extern "C" fn pacman_specialn_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), false);

    return 0.into();
}

pub fn install() {
    Agent::new("pacman")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_end)
        
        .status(Pre, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, pacman_specialn_hold_pre)
        .status(Main, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, pacman_specialn_hold_main)
        .status(End, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, pacman_specialn_hold_end)
        .install();
}