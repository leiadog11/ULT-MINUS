use super::*;

// OPFF
pub unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        /*
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        */
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