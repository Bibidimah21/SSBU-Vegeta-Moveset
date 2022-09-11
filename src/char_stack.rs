use smash::hash40;
use smash::lib::lua_const::*;
use smash_script::macros::is_excute as other_is_excute;
use smash::app::AttackHeight;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash::app::lua_bind::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_battle_object::module_accessor;
use smash::app::{self, sv_information};
use smash::app::*;
use smash::app::utility::get_category;
use smash::app::sv_battle_object::entry_id;
use smash::app::lua_bind::ModelModule::set_mesh_visibility;
use smash_script::macros::*;
use smash::lib::L2CAgent;
use smash::lib::L2CValue;
use smash::app::stage::get_stage_id;
use smash::lib::lua_const::StageID::CampaignMap;
use smash::lib::lua_const::StageID::SP_Edit;
use smash::app::boss_private::is_multi_play_mode;
use std::{thread, time};
use smash::app::smashball::is_training_mode;
use std::ffi::CStr;
use std::ffi::CString;
use arcropolis_api::*;
use std::sync::atomic::{ Ordering, AtomicBool };

static mut FILE_IS_LOADED: bool = false;
pub static mut FIGHTER_NAME: [u64;9] = [0;9];
pub static mut MODEL_DATA_POS: [u64;9] = [0;9];

static mut NAME_CHECK: [bool;9] = [false;9];
pub static mut MESH_CHECK: [bool;9] = [false;9];


pub unsafe fn read_tag(addr: u64) -> String {
    let mut s: Vec<u8> = vec![];

    let mut addr = addr as *const u16;
    loop {
        if *addr == 0_u16 {
            break;
        }
        s.push(*(addr as *const u8));
        addr = addr.offset(1);
    }
    // No null terminator needed

    std::str::from_utf8(&s).unwrap().to_owned()
}

pub unsafe fn get_player_number(module_accessor:  &mut app::BattleObjectModuleAccessor) -> usize {
    let player_number;
    if app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

pub unsafe fn is_VEGETA(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
    if fighter_kind == *FIGHTER_KIND_LUCARIO && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("VEGETA") {
        return true;
    }
    else {
        return false;
    }
}

#[smashline::fighter_reset]
fn global_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
    for i in 0..8 {
        NAME_CHECK[i] = false;
        MESH_CHECK[i] = false;
        }	
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_LUCARIO)]
pub fn VEGETA_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = fighter.module_accessor;
        let fighter_kind = app::utility::get_kind(&mut *fighter.module_accessor);
        let boma = module_accessor as *mut app::BattleObjectModuleAccessor as u64;
        if NAME_CHECK[get_player_number(&mut *fighter.module_accessor)] == false {
            let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
            let name_base = text + 0x52c3758;
            FIGHTER_NAME[get_player_number(&mut *fighter.module_accessor)] = hash40(&read_tag(name_base + 0x260 * get_player_number(&mut *fighter.module_accessor) as u64 + 0x8e));
            NAME_CHECK[get_player_number(&mut *fighter.module_accessor)] = true;
            MESH_CHECK[get_player_number(&mut *fighter.module_accessor)] = true;
            if is_VEGETA(&mut *fighter.module_accessor, fighter_kind) {
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("gamemodel"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_eye"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_stuckouteye1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_stuckouteye2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_halfblink1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_halfblink2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_halfblink3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_blink"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup4"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup5"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup6"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup7"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_faceup8"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_facen"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_talk"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_ouch"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_down"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_patterna"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_heavyattack"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_fall"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_capture"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_heavyouch"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_ottotto"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_furafura"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_cliff"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_catch"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_result_faceup"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_attack"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_result_mouth"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_attack2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_escape"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_attack3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_lleg"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_rleg"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("senaka"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("head"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("ura"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_bodyl"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_bodyr"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_larm"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("LUCARIO_rarm"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_facen"),true);
            }
            else {
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETAgamemodel"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_eye"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_stuckouteye1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_stuckouteye2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_halfblink1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_halfblink2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_halfblink3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_blink"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup4"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup5"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup6"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup7"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_faceup8"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_facen"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_talk"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_ouch"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_down"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_patterna"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_heavyattack"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_fall"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_capture"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_heavyouch"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_ottotto"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_furafura"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_cliff"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_catch"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_result_faceup"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_attack"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_result_mouth"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_attack2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_escape"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_attack3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_lleg"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_rleg"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_bodyl"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_bodyr"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_larm"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("VEGETA_rarm"),false);
            }
        }
        let model_data = *((*((boma + 0x78) as *const u64) + 0x10) as *const u64);
        MODEL_DATA_POS[get_player_number(&mut *fighter.module_accessor)] = model_data;
    }
}


#[skyline::hook(offset= 0x35d2f50)] //mesh control function hook
pub unsafe fn set_mesh_visibility_lvl_2_replace(model_data: u64,mesh: u64,visibility: u8) -> u64 {
    let mut new_mesh = mesh;
	    let mut player_number = 8;
		for i in 0..8 {
			if MODEL_DATA_POS[i] == model_data {
				player_number = i;
				break;
			}
		}
        if !MESH_CHECK[player_number] {
            original!()(model_data,new_mesh,visibility);
        }
        else {
            if FIGHTER_NAME[player_number] == hash40("VEGETA") {
                if mesh == hash40("LUCARIO_facen") {
                    new_mesh = hash40("VEGETA_facen");
                }
                if mesh == hash40("LUCARIO_talk") {
                    new_mesh = hash40("VEGETA_talk");
                }
                if mesh == hash40("LUCARIO_ouch") {
                    new_mesh = hash40("VEGETA_ouch");
                }
                if mesh == hash40("LUCARIO_down") {
                    new_mesh = hash40("VEGETA_down");
                }
                if mesh == hash40("LUCARIO_patterna") {
                    new_mesh = hash40("VEGETA_patterna");
                }
                if mesh == hash40("LUCARIO_heavyattack") {
                    new_mesh = hash40("VEGETA_heavyattack");
                }
                if mesh ==hash40("LUCARIO_fall") {
                    new_mesh = hash40("VEGETA_fall");
                }
                if mesh == hash40("LUCARIO_capture") {
                    new_mesh = hash40("VEGETA_capture");
                }
                if mesh == hash40("LUCARIO_heavyouch") {
                    new_mesh = hash40("VEGETA_heavyouch");
                }
                if mesh == hash40("LUCARIO_ottotto") {
                    new_mesh = hash40("VEGETA_ottotto");
                }
                if mesh == hash40("LUCARIO_furafura") {
                    new_mesh = hash40("VEGETA_furafura");
                }
                if mesh == hash40("LUCARIO_cliff") {
                    new_mesh = hash40("VEGETA_cliff");
                }
                if mesh == hash40("LUCARIO_catch") {
                    new_mesh = hash40("VEGETA_catch");
                }
                if mesh == hash40("LUCARIO_attack") {
                    new_mesh = hash40("VEGETA_attack");
                }
                if mesh == hash40("LUCARIO_result_mouth") {
                    new_mesh = hash40("VEGETA_result_mouth");
                }
                if mesh == hash40("LUCARIO_attack2") {
                    new_mesh = hash40("VEGETA_attack2");
                }
                if mesh == hash40("LUCARIO_escape") {
                    new_mesh = hash40("VEGETA_escape");
                }
                if mesh == hash40("LUCARIO_attack3") {
                    new_mesh = hash40("VEGETA_attack3");
                }
                if mesh == hash40("LUCARIO_halfblink1") {
                    new_mesh = hash40("VEGETA_halfblink1");
                }
                if mesh == hash40("LUCARIO_halfblink2") {
                    new_mesh = hash40("VEGETA_halfblink2");
                }
                if mesh == hash40("LUCARIO_halfblink3") {
                    new_mesh = hash40("VEGETA_halfblink3");
                }
                if mesh == hash40("LUCARIO_faceup1") {
                    new_mesh = hash40("VEGETA_faceup1");
                }
                if mesh == hash40("LUCARIO_faceup2") {
                    new_mesh = hash40("VEGETA_faceup2");
                }
                if mesh == hash40("LUCARIO_faceup3") {
                    new_mesh = hash40("VEGETA_faceup3");
                }
                if mesh == hash40("LUCARIO_faceup4") {
                    new_mesh = hash40("VEGETA_faceup4");
                }
                if mesh == hash40("LUCARIO_faceup5") {
                    new_mesh = hash40("VEGETA_faceup5");
                }
                if mesh == hash40("LUCARIO_faceup6") {
                    new_mesh = hash40("VEGETA_faceup6");
                }
                if mesh == hash40("LUCARIO_faceup7") {
                    new_mesh = hash40("VEGETA_faceup7");
                }
                if mesh == hash40("LUCARIO_faceup8") {
                    new_mesh = hash40("VEGETA_faceup8");
                }
                if mesh == hash40("LUCARIO_blink") {
                    new_mesh = hash40("VEGETA_blink");
                }
                if mesh == hash40("LUCARIO_eye") {
                    new_mesh = hash40("VEGETA_eye");
                }
                if mesh == hash40("LUCARIO_stuckouteye1") {
                    new_mesh = hash40("VEGETA_stuckouteye1");
                }
                if mesh == hash40("LUCARIO_stuckouteye2") {
                    new_mesh = hash40("VEGETA_stuckouteye2");
                }
            }
        }
        original!()(model_data,new_mesh,visibility)
}

#[acmd_script( agent = "LUCARIO", script = "game_fly", category = ACMD_GAME )]
unsafe fn VEGETA_laser(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_VEGETA(fighter.module_accessor, *FIGHTER_KIND_LUCARIO) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hit_3"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        original!(fighter)
    }
}

#[smashline::installer]
pub fn install() {
    install_agent_frames!(VEGETA_opff);
    install_agent_resets!(global_reset);
    skyline::install_hook!(set_mesh_visibility_lvl_2_replace);
    smashline::install_acmd_scripts!(
        VEGETA_laser,
    );
}
