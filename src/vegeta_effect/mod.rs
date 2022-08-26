use crate::utils::*;
use crate::vegeta::{CHARGE_TIME};
use crate::vegeta_status::GALICKGUN_ROT;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smashline::*;
use smash_utils::bomaext::BomaExt;
use crate::vegeta::{TEST, TEST2, TEST3};


#[acmd_script(
agent = "lucario",
script = "effect_attack11",
category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {

    });
}

#[acmd_script(
agent = "lucario",
script = "effect_attack12",
category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {

    });
}

#[acmd_script(
agent = "lucario",
script = "effect_attack13",
category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = &mut *fighter.module_accessor;
    let mut r = 1.0;
    let mut g = 1.0;
    let mut b = 1.0;
    if module_accessor.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM) == 3{
        r = 2.0;
        g = 0.2;
        b = 0.5;
    }
    let rot = if module_accessor.lr() > 0.0{
        30
    }
    else{
        -30
    };
    acmd!(lua_state, {
    frame(Frame=23)
    if(is_execute){
            EFFECT(hash40("lucario_hattack_b"), hash40("havel"), 0, 0, 0, 0, 0, rot, 1.2, false)
            LAST_EFFECT_SET_COLOR(/*R*/ r, /*G*/ g, /*B*/ b)
    }
    });
}

#[acmd_script(agent = "lucario", script = "effect_attackdash", category = ACMD_GAME)]
unsafe fn effect_vegeta_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=7)
        if(is_excute){
            EFFECT_FLIP_ALPHA(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 1.5, 11.5, 3.5, 9, -10, 0, 0.8, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ, 0.7)
        }
        frame(Frame=30)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_script(
agent = "lucario",
scripts = ["effect_attacks3", "effect_attacks3hi", "effect_attacks3lw"],
category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
    frame(Frame=6)
    if(is_excute){
        EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), 0, 13, -2, 0, -70, 75, 1.0, true, EF_FLIP_XY, 1)
        LAST_EFFECT_SET_RATE(0.8)
    }
    frame(Frame=35)
    if(is_excute){
        EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("toel"), 0, 8, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 1.5)
    }
    });
}

#[acmd_script(agent = "lucario", scripts = ["effect_bigbangatk", "effect_bigbangatk_air"], category = ACMD_EFFECT)]
unsafe fn effect_vegeta_bigbangatk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = &mut *fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    let mut r = 1.0;
    let mut g = 2.0;
    let mut b = 1.5;
    if module_accessor.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM) == 3{
        r = 2.0;
        g = 0.5;
        b = 1.0;
    }
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_execute){
            EFFECT(hash40("lucario_hadoudan"), hash40("top"), 18, 17, 0, 0, 0, 0, 3.0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ r, /*G*/ g, /*B*/ b)
        }
        frame(Frame=27)
        if(is_execute){
            EFFECT_OFF_KIND(hash40("lucario_hadoudan"), true, true)
        }
    });
}

#[acmd_script(
agent = "lucario",
script = "effect_attacklw3",
category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {

    });
}

#[acmd_script(
agent = "lucario",
script = "effect_attackhi3",
category = ACMD_EFFECT)]
unsafe fn vegeta_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = &mut *fighter.module_accessor;
    let mut r = 1.0;
    let mut g = 1.0;
    let mut b = 1.0;
    if module_accessor.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM) == 3{
        r = 2.0;
        g = 0.2;
        b = 0.5;
    }
    acmd!(lua_state, {
        frame(Frame=13)
        if(is_execute){
            EFFECT_FOLLOW(hash40("lucario_hattack_b"), hash40("top"), 0, 27.0, 0, 0, 90, 90, 1.0, true)
            LAST_EFFECT_SET_COLOR(r, g, b)
        }
    });
}

#[acmd_script(agent = "lucario_auraball", scripts = ["effect_start", "effect_shoot", "effect_charge", "effect_chargemax"], category = ACMD_EFFECT)]
unsafe fn effect_vegeta_projectile(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", scripts = ["effect_galickgun_fire", "effect_galickgun_fire_air"], category = ACMD_EFFECT)]
unsafe fn effect_vegeta_galickgun_fire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {
    frame(Frame=11)
    if(is_execute){
        EFFECT(hash40("lucario_final_beam"), hash40("rot"), 5, 0, 0, 0, 0, GALICKGUN_ROT[entry_id], 0.3, false)
        LAST_EFFECT_SET_COLOR(/*R*/ 0.5, /*G*/ 0.3, /*B*/ 0.6)

    }
    frame(Frame=35)
    if(is_execute){
        EFFECT_OFF_KIND(hash40("lucario_final_beam"), true, true)
        EFFECT(hash40("lucario_final_beam_end"), hash40("rot"), 5, 0, 0, 0, 0, GALICKGUN_ROT[entry_id], 0.3, false)
        LAST_EFFECT_SET_COLOR(/*R*/ 0.5, /*G*/ 0.3, /*B*/ 0.6)
    }
    });
}

#[acmd_script(agent = "lucario", script = "effect_specialhimove", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_specialhimove(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = &mut *fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    let a = read_rgb_from_file();
    let current_form = module_accessor.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    acmd!(lua_state, {
        if(is_execute){
	        EFFECT_FOLLOW(hash40("lucario_sinsoku_hadou2"), hash40("rot"), 0.0, 15.0, 0.0, 0, 0, 270, 5, true)
            LAST_EFFECT_SET_ALPHA(0.6)
            rust{
            let eff = EffectModule::get_last_handle(module_accessor) as u32;
            if current_form == 1{
                EffectModule::set_rgb(module_accessor, eff, 0.7, 0.7, 0.0);
            }
            else if current_form == 3 {
                EffectModule::set_rgb(module_accessor, eff, 5.0, 0.7, 5.0);
            }
            else if current_form != 2{
                 EffectModule::set_rgb(module_accessor, eff, 0.3, 0.3, 0.3);
            }
            }
        }
    });
}

#[acmd_script(agent = "lucario", script = "effect_attacks4", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_execute){
           // EFFECT_ALPHA(hash40("sys_attack_speedline"), hash40("top"), 0, TEST, TEST2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, 0.45)
        }
    });
}

#[acmd_script(agent = "lucario", script = "effect_attackhi4", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_execute){

        }
    });
}

#[acmd_script(agent = "lucario", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "effect_attackairf", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {
        frame(Frame=13)
        if(is_excute){
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), 0, 9, 0, -3, -11, -113, 1.1, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_RATE(1.3)
        }
    });
}


#[acmd_script(agent = "lucario", script = "effect_attackairn", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {
    frame(Frame=10)
    if(is_execute){
          //EFFECT_FOLLOW(hash40("sys_attack_arc_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 3.0, false)
    }
    });
}

#[acmd_script(agent = "lucario", script = "effect_attackairb", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_execute){
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), 0, 9, 1, 160, 60, 15, 1, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_RATE(2.0)
        }

    });
}

#[acmd_script(agent = "lucario", script = "effect_attackairhi", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_execute){
               EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("toel"), 0, 5, 2, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, 1.5)
        }
    });
}

#[acmd_script(agent = "lucario", script = "effect_attackairlw", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "effect_throwf", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "effect_throwb", category = ACMD_EFFECT)]
unsafe fn effect_vegeta_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            EFFECT(hash40("sys_crown"), hash40("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=16)
        if(is_excute){
            LANDING_EFFECT_FLIP(hash40("sys_whirlwind_l"), hash40("sys_whirlwind_r"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, EF_FLIP_NONE)
        }
        frame(Frame=22)
        if(is_excute){
            EFFECT_FOLLOW_FLIP_COLOR(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("rot"), 0, -2, 0, 0, -90, 0, 1.5, true, EF_FLIP_YZ, 1, 1, 1)
        }
        frame(Frame=26)
        if(is_excute){
            EFFECT_FOLLOW_FLIP_COLOR(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("rot"), 0, 1.5, 0, 0, -90, 0, 1.5, true, EF_FLIP_YZ, 1, 1, 1)
        }
        frame(Frame=29)
        if(is_excute){
            LANDING_EFFECT_FLIP(hash40("sys_whirlwind_l"), hash40("sys_whirlwind_r"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, EF_FLIP_NONE)
        }
        frame(Frame=34)
        if(is_excute){
            EFFECT_FOLLOW_FLIP_COLOR(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("rot"), 0, 0, 0, 0, 0, 0, 1.5, true, EF_FLIP_YZ, 1, 1, 1)
        }
        frame(Frame=44)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash_s"), hash40("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
        }
        frame(Frame=45)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_script(agent = "lucario", script = "effect_win2", category = ACMD_EFFECT)]
unsafe fn effect_win_2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "effect_win2wait", category = ACMD_EFFECT)]
unsafe fn effect_win_2_wait(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", scripts = ["effect_win1", "effect_win3", "effect_win1wait", "effect_win3wait"], category = ACMD_EFFECT)]
unsafe fn effect_win(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "effect_finalstart", category = ACMD_EFFECT)]
unsafe fn effect_final_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = (*module_accessor).entry_id();
    let angle = if (*module_accessor).lr() > 0.0 {
        90
    }
    else {
        270
    };
    acmd!(lua_state, {
    frame(Frame=90)
    if(is_execute){
        EFFECT(hash40("lucario_final_beam"), hash40("rot"), 2, 3, 0, 0, 0, angle, 1.0, false)
        LAST_EFFECT_SET_COLOR(/*R*/ 10.0, /*G*/ 10.0, /*B*/ 0.0)
    }
    frame(Frame=208)
    if(is_execute){
        EFFECT_OFF_KIND(hash40("lucario_final_beam"), true, true)
        EFFECT(hash40("lucario_final_beam_end"), hash40("rot"), 2, 3, 0, 0, 0, angle, 1.0, false)
        LAST_EFFECT_SET_COLOR(/*R*/ 10.0, /*G*/ 10.0, /*B*/ 0.0)
    }
    });
}

#[acmd_script(
agent = "lucario",
script = "effect_finalairend",
category = ACMD_EFFECT)]
unsafe fn effect_vegeta_finalairend(fighter: &mut L2CAgentBase) {

}

pub fn install() {
    smashline::install_acmd_scripts! {
        effect_vegeta_attack11,
        effect_vegeta_attack12,
        effect_vegeta_attack13,
        effect_vegeta_galickgun_fire,
        effect_vegeta_projectile,
        effect_vegeta_attackairn,
        effect_vegeta_attackairf,
        effect_vegeta_attackairhi,
        effect_vegeta_attackairlw,
        effect_vegeta_attackairb,
        effect_vegeta_throwf,
        effect_vegeta_throwb,
        effect_vegeta_attacks3,
        effect_vegeta_attacklw3,
        effect_vegeta_bigbangatk,
        vegeta_attackhi3,
        effect_vegeta_attackhi4,
        effect_vegeta_attacks4,
        effect_vegeta_attacklw4,
        effect_vegeta_attackdash,
        effect_win,
        effect_win_2,
        effect_win_2_wait,
        effect_final_start,
        effect_vegeta_finalairend,
        effect_vegeta_specialhimove
    };
}
