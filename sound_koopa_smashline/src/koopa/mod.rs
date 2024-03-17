use {
	smash::{
		app::{
			lua_bind::*,
			sv_animcmd,
			sv_math
		},
		hash40,
		lib::lua_const::*,
		lua2cpp::L2CAgentBase,
		phx::Hash40
	},
	smash_script::*,
	smashline::*
};

unsafe extern "C" fn koopa_sound_appealhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopa_attack07"));
	}
}

unsafe extern "C" fn koopa_sound_attack12(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_swing_m"));
	}
}

unsafe extern "C" fn koopa_sound_attackairb(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackair_b01"));
	}
}

unsafe extern "C" fn koopa_sound_attackairf(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
	}
}

unsafe extern "C" fn koopa_sound_attackairhi(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
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
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackair_h01"));
	}
}

unsafe extern "C" fn koopa_sound_attackairlw(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
	}
}

unsafe extern "C" fn koopa_sound_attackairn(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_n01"));
	}
}

unsafe extern "C" fn koopa_sound_attackdash(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackdash"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::PLAY_LANDING_SE(fighter, Hash40::new("se_koopa_landing02"));
	}
}

unsafe extern "C" fn koopa_sound_attackhi3(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
	}
}

unsafe extern "C" fn koopa_sound_attacklw3(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
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
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l01"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
	}
}

unsafe extern "C" fn koopa_sound_attacks3(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 6);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
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

unsafe extern "C" fn koopa_sound_batswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
	}
}

unsafe extern "C" fn koopa_sound_itemheavyget(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_koopa_heavyget"));
	}
}

unsafe extern "C" fn koopa_sound_itemheavythrowb4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
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

unsafe extern "C" fn koopa_sound_itemheavythrowf4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
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

unsafe extern "C" fn koopa_sound_itemheavythrowhi4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
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

unsafe extern "C" fn koopa_sound_itemheavythrowlw4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
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

unsafe extern "C" fn koopa_sound_itemlightthrowb4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
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

unsafe extern "C" fn koopa_sound_itemlightthrowf4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
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

unsafe extern "C" fn koopa_sound_itemlightthrowhi4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
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

unsafe extern "C" fn koopa_sound_itemlightthrowlw4(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
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

unsafe extern "C" fn koopa_sound_itemswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack05"));
	}
}

unsafe extern "C" fn koopa_sound_jump(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 2);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_jump01"));
		}
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) == false {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_jump01"));
		}
	}
}

unsafe extern "C" fn koopa_sound_ottotto(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 5);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if sound == 0 {
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_ottotto"));
		}
	}
}

unsafe extern "C" fn koopa_sound_throwb(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 14.0);
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
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
}

unsafe extern "C" fn koopa_sound_throwf(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 20.0);
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
	sv_animcmd::wait(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
}

unsafe extern "C" fn koopa_sound_throwhi(fighter: &mut L2CAgentBase) {
	let sound = sv_math::rand(hash40("fighter"), 4);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
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
	sv_animcmd::wait(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_smash_l01"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_common_throw_02"));
		sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
	}
}

unsafe extern "C" fn koopa_sound_win1(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_special_n01_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_landing02"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 115.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01_02"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 123.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
	}
}

unsafe extern "C" fn koopa_sound_win3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01_02"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n02_win03"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01_win03"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 67.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n03_win03"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 99.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_koopa_special_n02_win03"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 124.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_koopa_landing02"));
	}
}

pub fn install() {
	Agent::new("koopa")
		.sound_acmd("sound_appealhil", koopa_sound_appealhi)
		.sound_acmd("sound_appealhir", koopa_sound_appealhi)
		.sound_acmd("sound_attack12", koopa_sound_attack12)
		.sound_acmd("sound_attackairb", koopa_sound_attackairb)
		.sound_acmd("sound_attackairf", koopa_sound_attackairf)
		.sound_acmd("sound_attackairhi", koopa_sound_attackairhi)
		.sound_acmd("sound_attackairlw", koopa_sound_attackairlw)
		.sound_acmd("sound_attackairn", koopa_sound_attackairn)
		.sound_acmd("sound_attackdash", koopa_sound_attackdash)
		.sound_acmd("sound_attackhi3", koopa_sound_attackhi3)
		.sound_acmd("sound_attacklw3", koopa_sound_attacklw3)
		.sound_acmd("sound_attacks3", koopa_sound_attacks3)
		.sound_acmd("sound_attacks3hi", koopa_sound_attacks3)
		.sound_acmd("sound_attacks3lw", koopa_sound_attacks3)
		.sound_acmd("sound_batswing4", koopa_sound_batswing4)
		.sound_acmd("sound_itemheavyget", koopa_sound_itemheavyget)
		.sound_acmd("sound_itemheavythrowb4", koopa_sound_itemheavythrowb4)
		.sound_acmd("sound_itemheavythrowf4", koopa_sound_itemheavythrowf4)
		.sound_acmd("sound_itemheavythrowhi4", koopa_sound_itemheavythrowhi4)
		.sound_acmd("sound_itemheavythrowlw4", koopa_sound_itemheavythrowlw4)
		.sound_acmd("sound_itemlightthrowb4", koopa_sound_itemlightthrowb4)
		.sound_acmd("sound_itemlightthrowairb4", koopa_sound_itemlightthrowb4)
		.sound_acmd("sound_itemlightthrowf4", koopa_sound_itemlightthrowf4)
		.sound_acmd("sound_itemlightthrowairf4", koopa_sound_itemlightthrowf4)
		.sound_acmd("sound_itemlightthrowhi4", koopa_sound_itemlightthrowhi4)
		.sound_acmd("sound_itemlightthrowairhi4", koopa_sound_itemlightthrowhi4)
		.sound_acmd("sound_itemlightthrowlw4", koopa_sound_itemlightthrowlw4)
		.sound_acmd("sound_itemlightthrowairlw4", koopa_sound_itemlightthrowlw4)
		.sound_acmd("sound_lipstickswing4", koopa_sound_itemswing4)
		.sound_acmd("sound_starrodswing4", koopa_sound_itemswing4)
		.sound_acmd("sound_jumpback", koopa_sound_jump)
		.sound_acmd("sound_jumpfront", koopa_sound_jump)
		.sound_acmd("sound_ottotto", koopa_sound_ottotto)
		.sound_acmd("sound_throwb", koopa_sound_throwb)
		.sound_acmd("sound_throwf", koopa_sound_throwf)
		.sound_acmd("sound_throwhi", koopa_sound_throwhi)
		.sound_acmd("sound_win1", koopa_sound_win1)
		.sound_acmd("sound_win3", koopa_sound_win3)
		.install();
}
