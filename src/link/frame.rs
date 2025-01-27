use super::*;

// OPFF
pub unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);

        // RESPAWN CODE
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            MIPHAS_GRACE = true;
            DARUKS_PROTECTION = true;
            REVALIS_GALE = true;
            URBOSAS_FURY = true;
            // PLAY A RANDOM SPIRIT VOICE
            let rand = smash::app::sv_math::rand(hash40("agent"), 3) as u64;
            if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_link_spirit_ready01")) &&
                !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_link_spirit_ready02")) && 
                !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_link_spirit_ready03")) &&
                !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_link_spirit_ready04")) { 
                if rand == 0 {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_link_spirit_ready01"), true, false, false, false, enSEType(0));
                }
                else if rand == 1 {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_link_spirit_ready02"), true, false, false, false, enSEType(0));
                }
                else if rand == 2 {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_link_spirit_ready03"), true, false, false, false, enSEType(0));
                }
                else if rand == 3 {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_link_spirit_ready04"), true, false, false, false, enSEType(0));
                }
            }
        }

        // CONTROL LINKS AERIAL UP SPECIAL
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if situation_kind == *SITUATION_KIND_AIR {
                if UP_B_USED {
                    if REVALIS_GALE { 
                        if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_link_spirit_activate")) {
                            REVALIS_GALE = false;
                            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_link_spirit_activate"), true, false, false, false, enSEType(0));
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 5.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -5.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 10.0, 15.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -10.0, 15.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 15.0, 20.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -15.0, 20.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 20.0, 25.0, 0.0, 0, 0, 0, 0.8, false);
                            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -20.0, 25.0, 0.0, 0, 0, 0, 0.8, false);
                            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, true);
                        }
                    } 
                    else {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, true);
                    }
                }
                else {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, true);
                }
            }
        }

        // ALLOW LINK TO USE AERIAL UP B AFTER TOUCHING THE GROUND OR GRABBING LEDGE
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            UP_B_USED = false;
        }

        // GIVE LINK BACK HIS UP B AFTER GETTING HIT
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            UP_B_USED = false;
        }
    }
}

// ON START
pub unsafe extern "C" fn link_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        MIPHAS_GRACE = true;
        DARUKS_PROTECTION = true;
        REVALIS_GALE = true;
        URBOSAS_FURY = true;
    }
}

pub fn install() {
    Agent::new("link")
        .on_line(Main, link_frame)
        .on_start(link_start)
        .install();
}