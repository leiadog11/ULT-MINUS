use super::*;

// OPFF
pub unsafe extern "C" fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let arsene_entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        // if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
        //     ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_DOYLE, false, 0);
        //     WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE);
        //     WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESERVE_SUMMON_DISPATCH);
        //     WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
        //     StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SUMMON, false);
        //     FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, FighterEntryID(arsene_entry_id), 100.0);
        // }


        if CURSE_TIMER[ENTRY_ID] > 0 {
            CURSE_TIMER[ENTRY_ID] -= 1; 
        }
    }
}

// ON START
pub unsafe extern "C" fn jack_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        CURSE_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("jack")
        .on_line(Main, jack_frame)
        .on_start(jack_start)
        .install();
}