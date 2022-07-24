use crate::utils::*;
use crate::vegeta::CHARGE_TIME;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash::phx::Vector2f;
use smash_utils::bomaext::BomaExt;
use smashline::*;

#[acmd_script(
agent = "lucario",
script = "sound_attack11",
category = ACMD_SOUND)]
unsafe fn sound_vegeta_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        if(is_execute){
            PLAY_SE(hash40("vc_lucario_attack01"))
        }
    });
}

#[acmd_script(
agent = "lucario",
script = "sound_attack12",
category = ACMD_SOUND)]
unsafe fn sound_vegeta_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        if(is_execute){
            PLAY_SE(hash40("vc_lucario_attack02"))
        }
    });
}

#[acmd_script(
agent = "lucario",
script = "sound_attack13",
category = ACMD_SOUND)]
unsafe fn sound_vegeta_attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_execute){
            PLAY_SE(hash40("vc_lucario_attack03"))
        }
        frame(Frame=23)
        if(is_execute){
            PLAY_SE(hash40("se_lucario_attackl_h"))
        }
    });
}


#[acmd_script(agent = "lucario", scripts = ["sound_bigbangatk", "sound_bigbangatk_air"], category = ACMD_SOUND)]
unsafe fn sound_vegeta_bigbangatk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_execute){
            PLAY_SE(hash40("vc_lucario_001"))
        }
        frame(Frame=25)
        if(is_execute){
            PLAY_SE(hash40("se_lucario_attackl_h"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attackairf", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = module_accessor.entry_id();
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attackairb", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = module_accessor.entry_id();

    acmd!(lua_state, {
        frame(Frame=8)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attackairn", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attackairhi", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attackairlw", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(
agent = "lucario",
scripts = ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"],
category = ACMD_SOUND)]
unsafe fn sound_vegeta_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(
agent = "lucario",
script = "sound_attacklw3",
category = ACMD_SOUND)]
unsafe fn sound_vegeta_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
    });
}

#[acmd_script(
agent = "lucario",
script = "sound_attackhi3",
category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_execute){
            PLAY_SEQUENCE(hash40("seq_lucario_rnd_attack"))
        }
        frame(Frame=13)
        if(is_execute){
            PLAY_SE(hash40("se_lucario_attackl_h"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attacks4", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_execute){
            PLAY_SE(hash40("vc_lucario_attack05"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attackhi4", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_execute){
            PLAY_SE(hash40("vc_lucario_attack03"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_attacklw4", category = ACMD_SOUND)]
unsafe fn sound_vegeta_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("vc_lucario_attack02"))
        }
        frame(Frame=15)
        if(is_execute){
            PLAY_SE(hash40("se_lucario_attackl_h"))
        }
        frame(Frame=29)
        if(is_execute){
            PLAY_SE(hash40("se_lucario_attackl_h"))
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts! {
        sound_vegeta_attack11,
        sound_vegeta_attack12,
        sound_vegeta_attack13,
        sound_vegeta_bigbangatk,
        sound_vegeta_attackairf,
        sound_vegeta_attackairb,
        sound_vegeta_attackairn,
        sound_vegeta_attackairhi,
        sound_vegeta_attackairlw,
        sound_vegeta_attacks3,
        sound_vegeta_attackhi3,
        sound_vegeta_attacklw3,
        sound_vegeta_attacks4,
        sound_vegeta_attackhi4,
        sound_vegeta_attacklw4
    };
}
