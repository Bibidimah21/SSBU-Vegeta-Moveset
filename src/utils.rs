use smash::app::FighterEntryID;
use smash::lib::lua_const::*;
use smash::{
    app::{
        lua_bind,
        lua_bind::{StatusModule::*, *},
        sv_animcmd::{frame, wait},
        BattleObjectModuleAccessor,
    },
    hash40,
    lib::{lua_const::*, L2CValue},
    lua2cpp::{L2CAgentBase, L2CFighterCommon},
    phx::{Hash40, Vector3f},
    *,
};
use smash_utils::bomaext::BomaExt;
use smash_utils::utils::FIGHTER_MANAGER;
use smashline::*;

pub const FIGHTER_VEGETA_STATUS_KIND_SUPERDASHKICK: i32 = 0x1eb;
pub const FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_START: i32 = 0x1ec;
pub const FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_HOLD: i32 = 0x1ed;
pub const FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_FIRE: i32 = 0x1ee;


pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL: i32 = 0x100000c6;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_CHARGE_FRAME: i32 = 0x100000c7;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_LEFT_EFFECT_HANDLE: i32 = 0x100000c8;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_RIGHT_EFFECT_HANDLE: i32 = 0x100000c9;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE_EFFECT_HANDLE: i32 = 0x100000cA;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE: i32 = 0x100000cB;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM: i32 = 0x100000cC;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER: i32 = 0x100000cD;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER: i32 = 0x100000cE;

pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_EFFECT_SIZE: i32 = 0x53;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_HITBOX_SIZE: i32 = 0x54;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_DAMAGE: i32 = 0x55;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL: i32 = 0x56;

pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_KIBLAST_RAPIDFIRE: i32 = 0x200000e5;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_AMAZING_IMPACT: i32 = 0x200000e6;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_S4_EFFECT: i32 = 0x200000e7;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT: i32 = 0x200000e8;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_SSJ: i32 = 0x200000e9;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_SSJB: i32 = 0x200000eA;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_SSJBE: i32 = 0x200000eB;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_UE: i32 = 0x200000eC;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_FORM_TIMER_END: i32 = 0x200000eD;
pub const FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_AIR_GALICK_GUN: i32 = 0x200000eE;

pub const ZERO_VECTOR:Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
static mut QCF_COUNTER: [f32; 8] = [0.0; 8];
static mut QCB_COUNTER: [f32; 8] = [0.0; 8];
static mut IS_DOWN_QCF: [bool; 8] = [false; 8];
static mut IS_SIDE_QCF: [bool; 8] = [false; 8];
static mut IS_STRAIGHT_QCF: [bool; 8] = [false; 8];
static mut IS_DOWN_QCB: [bool; 8] = [false; 8];
static mut IS_SIDE_QCB: [bool; 8] = [false; 8];
static mut IS_STRAIGHT_QCB: [bool; 8] = [false; 8];
pub static mut r:f32 = 0.0;
pub static mut g:f32 = 0.0;
pub static mut b:f32 = 0.0;


#[repr(C)]
pub struct ModelColorType(pub i32);

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind31ModelModule__set_color_rgb_implEPNS_26BattleObjectModuleAccessorEfffNS_16MODEL_COLOR_TYPEE"]
    pub fn set_color_rgb(
        arg1: *mut BattleObjectModuleAccessor,
        arg2: f32,
        arg3: f32,
        arg4: f32,
        arg5: ModelColorType,
    );
}

pub unsafe fn get_attackers(boma: &mut BattleObjectModuleAccessor) -> Vec<usize>{
    let attacker_ids = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    let mut players:Vec<usize> = vec![];
    for i in 0..8{
        if attacker_ids & (1 << i) == 0 {
            continue;
        }
        players.push(i as usize)
    }
    players
}

pub unsafe fn get_attacked_players(boma: &mut BattleObjectModuleAccessor) -> Vec<usize>{
    let mut attacked_players:Vec<usize> = vec![];
    for i in 0..7{
        let player = &mut *get_module_accessor_by_entry_id(i);
        if get_attackers(player).contains(&boma.entry_id()){
            attacked_players.push(i as usize);
        }
    }
    attacked_players
}

pub fn get_module_accessor_by_entry_id(
    entry_id: i32,
) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(
            smash::app::Fighter::get_id_from_entry_id(entry_id),
        )
    }
}

pub unsafe fn enable_jump(module_accessor: &mut BattleObjectModuleAccessor){
    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
        else if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        }
    }
    if ControlModule::is_enable_flick_jump(module_accessor){
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
            else if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
        }
    }
}


pub fn is_grounded(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };
    situation_kind == SITUATION_KIND_GROUND
}

pub unsafe fn set_position_lock(entry_id: i32) {
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), true);
}

pub unsafe fn unset_position_lock(entry_id: i32) {
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), false);
}

pub unsafe fn change_motion(module_accessor: *mut BattleObjectModuleAccessor, anim: &str) {
    MotionModule::change_motion(
        module_accessor,
        Hash40::new(anim),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );
}
pub unsafe fn get_entry_id(module_accessor: *mut BattleObjectModuleAccessor) -> usize {
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

pub unsafe fn disable_gravity(module_accessor: *mut BattleObjectModuleAccessor) {
    KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

pub unsafe fn enable_gravity(module_accessor: *mut BattleObjectModuleAccessor) {
    KineticModule::enable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}
pub unsafe fn is_qcf(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    let entry_id = get_entry_id(module_accessor);
    let controller_x = ControlModule::get_stick_x(module_accessor);
    let controller_y = ControlModule::get_stick_y(module_accessor);
    let lr = PostureModule::lr(module_accessor);
    //println!("Controller X: {} Controller Y: {}", controller_x, controller_y);
    //println!("{} {} {}", IS_DOWN_QCF[entry_id], IS_SIDE_QCF[entry_id], IS_STRAIGHT_QCF[entry_id]);
    if controller_y < -0.7 {
        IS_DOWN_QCF[entry_id] = true;
    }
    if IS_DOWN_QCF[entry_id] {
        QCF_COUNTER[entry_id] += 1.0;
    }
    //println!("{} {} {}", IS_DOWN_QCF[entry_id], IS_SIDE_QCF[entry_id], IS_STRAIGHT_QCF[entry_id]);
    if QCF_COUNTER[entry_id] < 15.0 {
        if lr > 0.0 {
            if controller_y > -0.6 && controller_x > 0.2 {
                IS_SIDE_QCF[entry_id] = true;
            }
            if controller_y > -0.2 && controller_x > 0.7 {
                IS_STRAIGHT_QCF[entry_id] = true;
            }
        } else {
            if controller_y > -0.6 && controller_x < -0.2 {
                IS_SIDE_QCF[entry_id] = true;
            }
            if controller_y > -0.2 && controller_x < -0.7 {
                IS_STRAIGHT_QCF[entry_id] = true;
            }
        }

        if IS_DOWN_QCF[entry_id] && IS_SIDE_QCF[entry_id] && IS_STRAIGHT_QCF[entry_id] {
            IS_STRAIGHT_QCF[entry_id] = false;
            IS_SIDE_QCF[entry_id] = false;
            IS_DOWN_QCF[entry_id] = false;
            QCF_COUNTER[entry_id] = 0.0;
            return true;
        }
    } else {
        IS_DOWN_QCF[entry_id] = false;
        IS_SIDE_QCF[entry_id] = false;
        IS_STRAIGHT_QCF[entry_id] = false;
        QCF_COUNTER[entry_id] = 0.0;
    }
    IS_STRAIGHT_QCF[entry_id] = false;
    IS_SIDE_QCF[entry_id] = false;
    false
}

pub unsafe fn is_qcb(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    let entry_id = get_entry_id(module_accessor);
    let controller_x = ControlModule::get_stick_x(module_accessor);
    let controller_y = ControlModule::get_stick_y(module_accessor);
    let lr = PostureModule::lr(module_accessor);

    if controller_y < -0.7 {
        IS_DOWN_QCB[entry_id] = true;
    }
    if IS_DOWN_QCB[entry_id] {
        QCB_COUNTER[entry_id] += 1.0;
    }
    if QCB_COUNTER[entry_id] < 15.0 {
        if lr > 0.0 {
            if controller_y > -0.6 && controller_x < -0.2 {
                IS_SIDE_QCB[entry_id] = true;
            }
            if controller_y > -0.2 && controller_x < -0.7 {
                IS_STRAIGHT_QCB[entry_id] = true;
            }
        } else {
            if controller_y > -0.6 && controller_x > 0.2 {
                IS_SIDE_QCB[entry_id] = true;
            }
            if controller_y > -0.2 && controller_x > 0.7 {
                IS_STRAIGHT_QCB[entry_id] = true;
            }
        }
        if IS_DOWN_QCB[entry_id] && IS_SIDE_QCB[entry_id] && IS_STRAIGHT_QCB[entry_id] {
            IS_STRAIGHT_QCB[entry_id] = false;
            IS_SIDE_QCB[entry_id] = false;
            IS_DOWN_QCB[entry_id] = false;
            QCB_COUNTER[entry_id] = 0.0;
            return true;
        }
    } else {
        IS_DOWN_QCB[entry_id] = false;
        IS_SIDE_QCF[entry_id] = false;
        IS_STRAIGHT_QCB[entry_id] = false;
        QCB_COUNTER[entry_id] = 0.0;
    }
    IS_STRAIGHT_QCB[entry_id] = false;
    IS_SIDE_QCB[entry_id] = false;
    false
}
