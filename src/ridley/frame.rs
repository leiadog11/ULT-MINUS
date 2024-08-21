use super::*;

// OPFF
pub unsafe extern "C" fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA) { 
            let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_boss_core_hit"), true, false, false, false, enSEType(0));
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_fire_m"), true, false, false, false, enSEType(0));
            let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_special_defense_up"), Hash40::new("top"), &dumb, &dumb, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, effect, 0.9, 0.0, 0.5);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA);
        }
    }
}

// ON START
pub unsafe extern "C" fn ridley_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA);
    }
}

pub fn install() {
    Agent::new("ridley")
        .on_line(Main, ridley_frame)
        .on_start(ridley_start)
        .install();
}