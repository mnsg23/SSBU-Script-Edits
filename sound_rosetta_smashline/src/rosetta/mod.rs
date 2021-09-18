use smash::phx::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

#[acmd_script(agent = "rosetta", script = "sound_batswing4", category = ACMD_SOUND)]
unsafe fn rosetta_sound_batswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_rosetta_attack05"));
	}
}

#[acmd_script(agent = "rosetta", script = "sound_clubswing4", category = ACMD_SOUND)]
unsafe fn rosetta_sound_clubswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		macros::PLAY_SE(fighter, Hash40::new("vc_rosetta_attack05"));
	}
}

#[acmd_script(agent = "rosetta", script = "sound_lipstickswing4", category = ACMD_SOUND)]
unsafe fn rosetta_sound_lipstickswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		macros::PLAY_SE(fighter, Hash40::new("vc_rosetta_attack05"));
	}
}

#[acmd_script(agent = "rosetta", script = "sound_starrodswing4", category = ACMD_SOUND)]
unsafe fn rosetta_sound_starrodswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		macros::PLAY_SE(fighter, Hash40::new("vc_rosetta_attack05"));
	}
}

#[acmd_script(agent = "rosetta", script = "sound_win3", category = ACMD_SOUND)]
unsafe fn rosetta_sound_win3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_rosetta_win03"));
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_swing_m_win03"));
	}
	frame(fighter.lua_state_agent, 107.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_tico_return_win03"));
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		rosetta_sound_batswing4,
		rosetta_sound_clubswing4,
		rosetta_sound_lipstickswing4,
		rosetta_sound_starrodswing4,
		rosetta_sound_win3,
	);
}
