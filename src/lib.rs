#![feature(proc_macro_hygiene)]
#![feature(asm)]

use smashline::*;
use smash::app::lua_bind::*;
use smash::lib::{*, lua_const::*};
use smash::lua2cpp::*;
use smash::hash40;

// Makes falcon punch reeaaaaaallllly big and powerful ;)
#[acmd_script(agent = "captain", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME)]
fn falcon_punch(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=15)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN)
        }
        frame(Frame=53)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD)
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=999.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=999, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("shoulderr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=2.5, X=-2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=4.0, X=4.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        if (WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=2.5, X=-3.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=5.0, X=1.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=4.0, X=5.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=5)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

// Changes status to jump status immediately
#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
fn mario_fireball_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        fighter.change_status(
        FIGHTER_STATUS_KIND_JUMP.as_lua_int(),
        false.into()
        );
    }
    1.into()
}

#[installer]
pub fn install() {
    install_acmd_scripts!(
        falcon_punch
    );

    install_status_scripts!(
        mario_fireball_main
    );
}

#[skyline::main(name = "smashline-template")]
pub fn main() {
    install();
}
