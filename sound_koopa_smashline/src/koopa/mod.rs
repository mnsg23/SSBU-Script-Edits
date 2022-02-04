use {
	smash::{
		app::{
			lua_bind::*,
			sv_animcmd::*,
			sv_math,
			sv_module_access
		},
		hash40,
		lib::lua_const::*,
		lua2cpp::L2CAgentBase,
		phx::Hash40
	},
	smash_script::*,
	smashline::*
};

#[acmd_script(agent = "koopa", scripts = ["sound_appealhil", "sound_appealhir"], category = ACMD_SOUND)]
unsafe fn koopa_sound_appealhi(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopa_attack07"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attack12", category = ACMD_SOUND)]
unsafe fn koopa_sound_attack12(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_swing_m"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackairb", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackairb(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackair_b01"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackairf", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackairf(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackairhi", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackairhi(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackair_h01"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackairlw", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackairlw(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackairn", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackairn(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_n01"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackdash", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackdash(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackdash"));
	}
	wait(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::PLAY_LANDING_SE(fighter, Hash40::new("se_koopa_landing02"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attackhi3", category = ACMD_SOUND)]
unsafe fn koopa_sound_attackhi3(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_attacklw3", category = ACMD_SOUND)]
unsafe fn koopa_sound_attacklw3(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l01"));
	}
	wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"], category = ACMD_SOUND)]
unsafe fn koopa_sound_attacks3(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01"));
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", script = "sound_batswing4", category = ACMD_SOUND)]
unsafe fn koopa_sound_batswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_itemheavyget", category = ACMD_SOUND)]
unsafe fn koopa_sound_itemheavyget(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_koopa_heavyget"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_itemheavythrowb4", category = ACMD_SOUND)]
unsafe fn koopa_sound_itemheavythrowb4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", script = "sound_itemheavythrowf4", category = ACMD_SOUND)]
unsafe fn koopa_sound_itemheavythrowf4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", script = "sound_itemheavythrowhi4", category = ACMD_SOUND)]
unsafe fn koopa_sound_itemheavythrowhi4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", script = "sound_itemheavythrowlw4", category = ACMD_SOUND)]
unsafe fn koopa_sound_itemheavythrowlw4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_itemlightthrowb4", "sound_itemlightthrowairb4"], category = ACMD_SOUND)]
unsafe fn koopa_sound_itemlightthrowb4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_itemlightthrowf4", "sound_itemlightthrowairf4"], category = ACMD_SOUND)]
unsafe fn koopa_sound_itemlightthrowf4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_itemlightthrowhi4", "sound_itemlightthrowairhi4"], category = ACMD_SOUND)]
unsafe fn koopa_sound_itemlightthrowhi4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_itemlightthrowlw4", "sound_itemlightthrowairlw4"], category = ACMD_SOUND)]
unsafe fn koopa_sound_itemlightthrowlw4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_lipstickswing4", "sound_starrodswing4"], category = ACMD_SOUND)]
unsafe fn koopa_sound_itemswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	}
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack05"));
	}
}

#[acmd_script(agent = "koopa", scripts = ["sound_jumpback", "sound_jumpfront"], category = ACMD_SOUND)]
unsafe fn koopa_sound_jump(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 2);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_jump01"));
		}
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) == false {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_jump01"));
		}
	}
}

#[acmd_script(agent = "koopa", script = "sound_ottotto", category = ACMD_SOUND)]
unsafe fn koopa_sound_ottotto(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 5);
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_ottotto"));
		}
	}
}

#[acmd_script(agent = "koopa", script = "sound_throwb", category = ACMD_SOUND)]
unsafe fn koopa_sound_throwb(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	wait(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_throwf", category = ACMD_SOUND)]
unsafe fn koopa_sound_throwf(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	wait(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
	}
	wait(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_throwhi", category = ACMD_SOUND)]
unsafe fn koopa_sound_throwhi(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
		}
		if sound == 1 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
		}
		if sound == 2 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
		}
		if sound == 3 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
		}
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	wait(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_smash_l01"));
	}
	wait(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_common_throw_02"));
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
		sv_module_access::sound(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
}

#[acmd_script(agent = "koopa", script = "sound_win1", category = ACMD_SOUND)]
unsafe fn koopa_sound_win1(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01"));
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_special_n01_win01"));
	}
	frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_landing02"));
	}
	frame(fighter.lua_state_agent, 115.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01_02"));
	}
	frame(fighter.lua_state_agent, 123.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
	}
}

#[acmd_script(agent = "koopa", script = "sound_win3", category = ACMD_SOUND)]
unsafe fn koopa_sound_win3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01_02"));
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n02_win03"));
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01_win03"));
	}
	frame(fighter.lua_state_agent, 67.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n03_win03"));
	}
	frame(fighter.lua_state_agent, 99.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_koopa_special_n02_win03"));
	}
	frame(fighter.lua_state_agent, 124.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_landing02"));
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		koopa_sound_appealhi,
		koopa_sound_attack12,
		koopa_sound_attackairb,
		koopa_sound_attackairf,
		koopa_sound_attackairhi,
		koopa_sound_attackairlw,
		koopa_sound_attackairn,
		koopa_sound_attackdash,
		koopa_sound_attackhi3,
		koopa_sound_attacklw3,
		koopa_sound_attacks3,
		koopa_sound_batswing4,
		koopa_sound_itemheavyget,
		koopa_sound_itemheavythrowb4,
		koopa_sound_itemheavythrowf4,
		koopa_sound_itemheavythrowhi4,
		koopa_sound_itemheavythrowlw4,
		koopa_sound_itemlightthrowb4,
		koopa_sound_itemlightthrowf4,
		koopa_sound_itemlightthrowhi4,
		koopa_sound_itemlightthrowlw4,
		koopa_sound_itemswing4,
		koopa_sound_jump,
		koopa_sound_ottotto,
		koopa_sound_throwb,
		koopa_sound_throwf,
		koopa_sound_throwhi,
		koopa_sound_win1,
		koopa_sound_win3,
	);
}
