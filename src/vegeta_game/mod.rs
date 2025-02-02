use crate::utils::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash::phx::*;
use smashline::*;
use crate::vegeta::{TEST, TEST2, TEST3};
use smash::phx::Vector3f;
use smash_utils::{utils::*,  bomaext::BomaExt};
use crate::vegeta::*;
use crate::utils::FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_AMAZING_IMPACT;

#[acmd_script(
agent = "lucario",
script = "game_attack11",
category = ACMD_GAME)]
unsafe fn vegeta_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=3)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=20, FKB=0, BKB=30, Size=3.0, X=0.0, Y=9.5, Z=5.0, X2=0.0, Y2=9.5, Z2=10.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frame=2)
    if(is_execute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=13)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
    }
    });
}

#[acmd_script(
agent = "lucario",
script = "game_attack12",
category = ACMD_GAME)]
unsafe fn vegeta_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=3)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=20, Size=3.0, X=0.0, Y=10.0, Z=4.5, X2=0.0, Y2=10.0, Z2=12.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame(0, 5.0, false)
    }
    wait(Frame=2)
    if(is_execute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    });
}

#[acmd_script(
agent = "lucario",
script = "game_attack13",
category = ACMD_GAME)]
unsafe fn vegeta_attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_aura");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=7)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=80, KBG=40, FKB=40, BKB=40, Size=4.0, X=0.0, Y=9.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame(0, 10.0, false)
    }
    frame(Frame=22)
     if(is_execute){
        AttackModule::clear_all()
     }
    frame(Frame=23)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=8.0, X=0.0, Y=12.0, Z=10.0, X2=0.0, Y2=22.0, Z2=27.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=5.0, X=0.0, Y=24.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=5.0, X=0.0, Y=5.0, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_PUNCH)

    }
    wait(Frame=4)
    if(is_execute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_script(agent = "lucario", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn vegeta_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=50, KBG=65, FKB=0, BKB=80, Size=4.2, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=9.0, Z2=7.2, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=80, KBG=60, FKB=0, BKB=80, Size=3.5, X=0.0, Y=9.0, Z=8.5, X2=0.0, Y2=9.0, Z2=7.2, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=6)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(
agent = "lucario",
scripts = ["game_attacks3", "game_attacks3hi", "game_attacks3lw"],
category = ACMD_GAME)]
unsafe fn vegeta_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=40, FKB=40, BKB=40, Size=4.0, X=0.0, Y=13.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=40, FKB=40, BKB=40, Size=4.0, X=0.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=40, FKB=40, BKB=40, Size=4.0, X=0.0, Y=8.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=40, FKB=40, BKB=40, Size=4.0, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=23.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=1, Frames=23.0, Unk=false)
    }
    wait(Frames=2)
    if(is_execute){
        AttackModule::clear_all()
    }
    frame(Frame=35)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.0, X=0.0, Y=14.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.0, X=0.0, Y=14.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.0, X=0.0, Y=8.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.0, X=0.0, Y=8.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_execute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_script(
agent = "lucario",
scripts = ["game_kiblastleft", "game_kiblastright", "game_kiblastairleft", "game_kiblastairright"],
category = ACMD_GAME)]
unsafe fn vegeta_kiblast(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_execute){
            ArticleModule::generate_article(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, 0)
            ArticleModule::shoot_exist(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
        }
    });
}



#[acmd_script(
agent = "lucario",
script = "game_attacklw3",
category = ACMD_GAME)]
unsafe fn vegeta_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=87, KBG=55, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=87, KBG=55, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=7.0, Angle=87, KBG=55, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_execute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_script(
agent = "lucario",
script = "game_attackhi3",
category = ACMD_GAME)]
unsafe fn vegeta_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_aura");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=13)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=2.5, Angle=90, KBG=125, FKB=0, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=2.5, Angle=90, KBG=125, FKB=0, BKB=0, Size=3.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=90, KBG=80, FKB=0, BKB=74, Size=7.0, X=0.0, Y=33.0, Z=1.0, X2=0.0, Y2=43.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=90, KBG=80, FKB=0, BKB=74, Size=5.0, X=0.0, Y=33.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=90, KBG=80, FKB=0, BKB=74, Size=5.0, X=0.0, Y=33.0, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("shoulderl"), Damage=2.5, Angle=90, KBG=100, FKB=0, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("arml"), Damage=2.5, Angle=90, KBG=100, FKB=0, BKB=0, Size=3.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=6)
    if(is_execute){
        AttackModule::clear_all()
    }
    });
}


#[acmd_script(
agent = "lucario",
script = "game_galickgun_fire",
category = ACMD_GAME)]
unsafe fn vegeta_galickgun_fire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    let damage = 0.2 + (CHARGE_TIME[entry_id] / 130.0);
    acmd!(lua_state, {
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=damage, Angle=361, KBG=50, FKB=10, BKB=10, Size=5.0, X=0.0, Y=10.0, Z=10.0, X2=0.0, Y2=9.0, Z2=205, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
    }
    frame(Frame=35)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=70, Size=5.0, X=0.0, Y=10.0, Z=10.0, X2=0.0, Y2=9.0, Z2=205, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
    }
    frame(Frame=36)
    if(is_excute){
       AttackModule::clear_all()
    }
    });
}

#[acmd_script(
agent = "lucario",
script = "game_galickgun_fire_air",
category = ACMD_GAME)]
unsafe fn vegeta_galickgun_fire_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    let damage = 0.2 + (CHARGE_TIME[entry_id] / 130.0);
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=damage, Angle=300, KBG=50, FKB=10, BKB=10, Size=5.0, X=0.0, Y=5.0, Z=9.0, X2=0.0, Y2=-132, Z2=143, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
    }
    frame(Frame=35)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=70, Size=5.0, X=0.0, Y=5.0, Z=9.0, X2=0.0, Y2=-132, Z2=143, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
    }
    frame(Frame=36)
    if(is_excute){
       AttackModule::clear_all()
    }
    });
}

#[acmd_script(agent = "lucario", scripts = ["game_bigbangatk", "game_bigbangatk_air"], category = ACMD_GAME)]
unsafe fn vegeta_bigbangatk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.7, Angle=40, KBG=10, FKB=10, BKB=0, Size=6.0, X=0.0, Y=16.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=1, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_ENERGY)
            AttackModule::set_add_reaction_frame(0, 2.0, false)
        }
        frame(Frame=25)
        if(is_execute){
            AttackModule::clear_all()
            ArticleModule::generate_article(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, 0)
            ArticleModule::shoot_exist(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
        }
    });
}


#[acmd_script(
agent = "lucario",
scripts = ["game_galickgun_start", "game_galickgun_hold"],
category = ACMD_GAME)]
unsafe fn vegeta_galickgun_hold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario_auraball", script = "game_shoot", category = ACMD_GAME)]
unsafe fn vegeta_projectile(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn vegeta_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let mut damage = 10.0;
    let mut kbg = 100;
    let mut bkb = 20;
    let mut collision_sound = *COLLISION_SOUND_ATTR_PUNCH;
    if boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_AMAZING_IMPACT){
        damage = 3.0;
        collision_sound = *COLLISION_SOUND_ATTR_BAT;
    }
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=13)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=damage, Angle=290, KBG=kbg, FKB=0, BKB=bkb, Size=5.0, X=0.0, Y=11.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=collision_sound, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=damage, Angle=290, KBG=kbg, FKB=0, BKB=bkb, Size=5.0, X=0.0, Y=11.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=collision_sound, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=damage, Angle=290, KBG=kbg, FKB=0, BKB=bkb, Size=5.0, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=collision_sound, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=damage, Angle=290, KBG=kbg, FKB=0, BKB=bkb, Size=5.0, X=0.0, Y=9.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=collision_sound, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=16)
        if(is_execute){
            AttackModule::clear_all()
        }
        frame(Frame=57)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn vegeta_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(frame=10)
        if(is_execute){
            REVERSE_LR()
        }
        frame(Frame=12)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=97, FKB=0, BKB=27, Size=5.0, X=0.0, Y=3.0, Z=-2.0, X2=0.0, Y2=8.0, Z2=-2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=97, FKB=0, BKB=27, Size=5.0, X=0.0, Y=3.0, Z=-5.0, X2=0.0, Y2=8.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=97, FKB=0, BKB=27, Size=5.0, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=8.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=97, FKB=0, BKB=27, Size=5.0, X=0.0, Y=3.0, Z=-9.0, X2=0.0, Y2=8.0, Z2=-9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=97, FKB=0, BKB=27, Size=5.0, X=0.0, Y=3.0, Z=-11.0, X2=0.0, Y2=8.0, Z2=-11.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_execute){
            AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn vegeta_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=6)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=9.5, X=0.0, Y=11.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.0, X=0.0, Y=9.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.0, X=0.0, Y=9.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.0, X=0.0, Y=9.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=12)
        if(is_execute){
            AttackModule::clear_all()
        }
        frame(Frame=31)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn vegeta_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=9.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=19.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=10.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=16)
        if(is_execute){
            AttackModule::clear_all()
        }
        frame(Frame=34)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn vegeta_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED)
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            SET_SPEED_EX(0, 2, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
        }
        frame(Frame=11)
        if(is_execute){
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            SET_SPEED_EX(2.0, -6, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=300, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.0, Y=6.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=300, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=300, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.0, Y=8.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=30)
        if(is_execute){
            AttackModule::clear_all()
            WorkModule::off_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            KineticModule::resume_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
        }
        frame(Frame=38)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_landingairlw", category = ACMD_GAME)]
unsafe fn vegeta_landingairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=2.5, Angle=60, KBG=110, FKB=0, BKB=70, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=2.5, Angle=60, KBG=110, FKB=0, BKB=70, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("footl"), Damage=2.5, Angle=60, KBG=110, FKB=0, BKB=70, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn vegeta_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_aura");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=17.0, Angle=361, KBG=92, FKB=0, BKB=34, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=17.0, Angle=361, KBG=92, FKB=0, BKB=34, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("handr"), Damage=17.0, Angle=361, KBG=92, FKB=0, BKB=34, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_execute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn vegeta_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=11)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=15.0, Angle=75, KBG=82, FKB=0, BKB=32, Size=5.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=15.0, Angle=75, KBG=82, FKB=0, BKB=32, Size=5.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=15.0, Angle=75, KBG=82, FKB=0, BKB=32, Size=5.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=4)
        if(is_execute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn vegeta_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_execute){
            ArticleModule::remove_exist(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_execute){
            ArticleModule::generate_article(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, 0)
            ArticleModule::shoot_exist(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
        }
        frame(Frame=20)
        if(is_execute){
            REVERSE_LR()
        }
        frame(Frame=29)
        if(is_execute){
            ArticleModule::generate_article(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, 0)
            ArticleModule::shoot_exist(*FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
        }
    });
}



#[acmd_script(agent = "lucario", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn vegeta_catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.3, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}


#[acmd_script(agent = "lucario", script = "game_superdashkick", category = ACMD_GAME)]
unsafe fn vegeta_superdashkick(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_execute){
            SA_SET(State=SITUATION_KIND_AIR)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_win2", category = ACMD_GAME)]
unsafe fn win_2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    acmd!(lua_state, {

    });
}

#[acmd_script(agent = "lucario", script = "game_win2wait", category = ACMD_GAME)]
unsafe fn win_wait_2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    acmd!(lua_state, {

    });
}
#[acmd_script(agent = "lucario", scripts = ["game_win1", "game_win3", "game_win1wait", "game_win3wait"], category = ACMD_GAME)]
unsafe fn win_wait(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    acmd!(lua_state, {

    });
}


#[acmd_script(agent = "lucario", script = "game_throwf", category = ACMD_GAME)]
unsafe fn vegeta_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=45, KBG=120, FKB=0, BKB=55, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=33)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=50, KBG=65, FKB=0, BKB=50, Size=6.0, X=0.0, Y=12.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
            AttackModule::set_catch_only_all(true, false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(frame=63)
        if(is_execute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=80, KBG=100, FKB=0, BKB=70, Size=6.0, X=0.0, Y=13.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_execute){
            AttackModule::clear_all()
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn vegeta_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=68, KBG=70, FKB=0, BKB=65, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=29)
        if(is_excute){
            AttackModule::clear_all()
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }

    });
}

#[acmd_script(agent = "lucario", script = "game_throwb", category = ACMD_GAME)]
unsafe fn vegeta_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=9.0, Angle=45, KBG=66, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK_IGNORE_THROW(ID=0, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=50, KBG=70, FKB=0, BKB=100, Size=4.0, X=-1.0, Y=6.0, Z=0.0, X2=-3.2, Y2=13.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=40)
        if(is_excute){
            AttackModule::clear_all()
            REVERSE_LR()
        }
        frame(Frame=43)
        if(is_excute){
            CHECK_FINISH_CAMERA(14, 7)
            rust{
                smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.5);
                smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 8.0, y: 0.0, z:0.0});
            }
        }
        frame(Frame=44)
        if(is_excute){
        ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }

    });
}

#[acmd_script(agent = "lucario", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn vegeta_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=84, KBG=66, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=17)
        if(is_excute){
        ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                        WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }

    });
}

#[acmd_script(
agent = "lucario",
script = "game_finalstart",
category = ACMD_GAME)]
unsafe fn vegeta_finalstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
    if(is_execute){
       HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
       FT_START_CUTIN()
       SLOW_OPPONENT(10, 90)
       if(!WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA)){
           REQ_FINAL_START_CAMERA_arg3(hash40("d04finalstart.nuanmb"), false, false)
       }
    }
    frame(Frame=80)
    if(is_excute){
        CAM_ZOOM_OUT()
    }
    frame(Frame=90)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.7, Angle=361, KBG=50, FKB=10, BKB=10, Size=20.0, X=0.0, Y=11.0, Z=20.0, X2=0.0, Y2=11.0, Z2=1000.0, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
    }
    frame(Frame=208)
    if(is_excute){
        AttackModule::clear_all()
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=70, Size=23.0, X=0.0, Y=11.0, Z=20.0, X2=0.0, Y2=11.0, Z2=1000.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_ENERGY)
    }
    frame(Frame=210)
    if(is_excute){
       AttackModule::clear_all()
       HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
    }
    });
}

#[acmd_script(
agent = "lucario",
script = "game_finalairend",
category = ACMD_GAME)]
unsafe fn vegeta_finalairend(fighter: &mut L2CAgentBase) {

}

#[acmd_script(agent = "lucario", script = "game_specialhimove", category = ACMD_GAME)]
unsafe fn vegeta_specialhimove(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=4.0, Angle=38, KBG=100, FKB=0, BKB=70, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            JostleModule::set_status(false)
        }
        frame(Frame=15)
        if(is_excute){
            GroundModule::set_passable_check(true)
        }
    });
}

#[acmd_script(agent = "lucario", scripts = ["game_specialhibound", "game_specialhiend"], category = ACMD_GAME)]
unsafe fn vegeta_specialhibound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        wait(Frames=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=3.0, Angle=38, KBG=100, FKB=0, BKB=70, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "lucario", script = "game_specialairhiend", category = ACMD_GAME)]
unsafe fn vegeta_specialairhiend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let entry_id = boma.entry_id();
    let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
    let mut collision_attr = hash40("collision_attr_normal");
    if current_form == 3{
        collision_attr = hash40("collision_attr_purple");
    }
    acmd!(lua_state, {
        wait(Frames=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=3.0, Angle=38, KBG=100, FKB=0, BKB=70, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=collision_attr, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=23)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X)
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts! {
        vegeta_attack11,
        vegeta_attack12,
        vegeta_attack13,
        vegeta_galickgun_hold,
        vegeta_galickgun_fire,
        vegeta_galickgun_fire_air,
        vegeta_bigbangatk,
        vegeta_projectile,
        vegeta_attackairf,
        vegeta_attackairn,
        vegeta_attackairhi,
        vegeta_attackairlw,
        vegeta_attackairb,
        vegeta_attacks4,
        vegeta_attacklw4,
        vegeta_attackhi4,
       // vegeta_landingairlw,
        vegeta_attacklw3,
        vegeta_catchattack,
        vegeta_superdashkick,
        vegeta_attacks3,
        vegeta_kiblast,
        vegeta_attackhi3,
        vegeta_attackdash,
        win_wait,
        win_2,
        win_wait_2,
        vegeta_throwf,
        vegeta_throwlw,
        vegeta_throwb,
        vegeta_throwhi,
        vegeta_finalstart,
        vegeta_finalairend,
        vegeta_specialhimove,
        vegeta_specialhibound,
        vegeta_specialairhiend
    };
}
