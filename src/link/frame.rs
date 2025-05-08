use super::*;

// OPFF
pub unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let frame = MotionModule::frame(boma);
        let damage = DamageModule::damage(boma, 0);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let posx = PostureModule::pos_x(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            MIPHAS_GRACE[ENTRY_ID] = true;
            DARUKS_PROTECTION[ENTRY_ID] = true;
            REVALIS_GALE[ENTRY_ID] = true;
            URBOSAS_FURY[ENTRY_ID] = true;
            // PLAY A RANDOM SPIRIT VOICE
            let rand = smash::app::sv_math::rand(hash40("agent"), 3) as u64;
            if !SoundModule::is_playing(boma, Hash40::new("se_link_spirit_ready01")) &&
                !SoundModule::is_playing(boma, Hash40::new("se_link_spirit_ready02")) && 
                !SoundModule::is_playing(boma, Hash40::new("se_link_spirit_ready03")) &&
                !SoundModule::is_playing(boma, Hash40::new("se_link_spirit_ready04")) { 
                if rand == 0 {
                    SoundModule::play_se(boma, Hash40::new("se_link_spirit_ready01"), true, false, false, false, enSEType(0));
                }
                else if rand == 1 {
                    SoundModule::play_se(boma, Hash40::new("se_link_spirit_ready02"), true, false, false, false, enSEType(0));
                }
                else if rand == 2 {
                    SoundModule::play_se(boma, Hash40::new("se_link_spirit_ready03"), true, false, false, false, enSEType(0));
                }
                else if rand == 3 {
                    SoundModule::play_se(boma, Hash40::new("se_link_spirit_ready04"), true, false, false, false, enSEType(0));
                }
            }
        }

        // MIPHAS GRACE
        if MIPHAS_GRACE[ENTRY_ID] == true {
            if damage >= 100.0 { 
                SoundModule::play_se(boma, Hash40::new("se_item_fairybottle_fairy"), true, false, false, false, enSEType(0));
                SoundModule::play_se(boma, Hash40::new("se_link_spirit_activate"), true, false, false, false, enSEType(0));
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 5.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -5.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 10.0, 15.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -10.0, 15.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 15.0, 20.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -15.0, 20.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 20.0, 25.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -20.0, 25.0, 0.0, 0, 0, 0, 0.8, false);
                DamageModule::add_damage(boma, -damage, 0);
                MIPHAS_GRACE[ENTRY_ID] = false;
            }
        }

        // CONTROL LINKS AERIAL UP SPECIAL
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if situation_kind == *SITUATION_KIND_AIR {
                if UP_B_USED[ENTRY_ID] {
                    if REVALIS_GALE[ENTRY_ID] { 
                        if !SoundModule::is_playing(boma, Hash40::new("se_link_spirit_activate")) {
                            REVALIS_GALE[ENTRY_ID] = false;
                            SoundModule::play_se(boma, Hash40::new("se_link_spirit_activate"), true, false, false, false, enSEType(0));
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 5.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -5.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 10.0, 15.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -10.0, 15.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 15.0, 20.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -15.0, 20.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 20.0, 25.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -20.0, 25.0, 0.0, 0, 0, 0, 0.8, false);
                            StatusModule::change_status_request_from_script(boma, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, true);
                        }
                    } 
                    else {
                        StatusModule::change_status_request_from_script(boma, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, true);
                    }
                }
                else {
                    StatusModule::change_status_request_from_script(boma, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, true);
                }
            }
        }

        // ALLOW LINK TO USE AERIAL UP B AFTER TOUCHING THE GROUND OR GRABBING LEDGE
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            UP_B_USED[ENTRY_ID] = false;
        }

        // GIVE LINK BACK HIS UP B AFTER GETTING HIT
        if DamageModule::reaction(boma, 0) > 1.0 {
            UP_B_USED[ENTRY_ID] = false;
        }

        // UP AIR AND DOWN AIR STALL
        if motion_kind == hash40("attack_air_hi") || motion_kind == hash40("attack_air_lw") { 
            if frame >= 14.0 && frame <= 55.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    if FLOAT_TIMER[ENTRY_ID] > 0 {
                        MotionModule::set_rate(boma, 0.0);
                        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        if xpos > 0.0  {
                            PostureModule::set_pos_2d(boma, &Vector2f {x: posx + 1.2, y: PostureModule::pos_y(boma)});
                        }        
                        else if xpos < 0.0  {
                            PostureModule::set_pos_2d(boma, &Vector2f {x: posx - 1.2, y: PostureModule::pos_y(boma)});
                        }
                        else {
                            PostureModule::set_pos_2d(boma, &Vector2f {x: posx, y: PostureModule::pos_y(boma)});
                        }
                        FLOAT_TIMER[ENTRY_ID] -= 1;
                    }
                    else {
                        MotionModule::set_rate(boma, 1.0);
                        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
                else {
                    MotionModule::set_rate(boma, 1.0);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
        }
        else {
            FLOAT_TIMER[ENTRY_ID] = 60;
        }
    }
}

// ON START
pub unsafe extern "C" fn link_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        UP_B_USED[ENTRY_ID] = false;
        EQUIPPED[ENTRY_ID] = false;
        MIPHAS_GRACE[ENTRY_ID] = true;
        DARUKS_PROTECTION[ENTRY_ID] = true;
        REVALIS_GALE[ENTRY_ID] = true;
        URBOSAS_FURY[ENTRY_ID] = true;
        DOWN_TILT_COUNT[ENTRY_ID] = 0;
        FLOAT_TIMER[ENTRY_ID] = 60;
    }
}

pub fn install() {
    Agent::new("link")
        .on_line(Main, link_frame)
        .on_start(link_start)
        .install();
}