use {
	smash::{
		app::sv_animcmd,
		lua2cpp::L2CAgentBase,
		phx::Hash40
	},
	smash_script::*,
	smashline::*
};

unsafe extern "C" fn rosetta_sound_batswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_rosetta_attack05"));
	}
}

unsafe extern "C" fn rosetta_sound_itemswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		macros::PLAY_SE(fighter, Hash40::new("vc_rosetta_attack05"));
	}
}

unsafe extern "C" fn rosetta_sound_win3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_rosetta_win03"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_swing_m_win03"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 107.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_tico_return_win03"));
	}
}

pub fn install() {
	Agent::new("rosetta")
		.sound_acmd("sound_batswing4", rosetta_sound_batswing4)
		.sound_acmd("sound_clubswing4", rosetta_sound_itemswing4)
		.sound_acmd("sound_lipstickswing4", rosetta_sound_itemswing4)
		.sound_acmd("sound_starrodswing4", rosetta_sound_itemswing4)
		.sound_acmd("sound_win3", rosetta_sound_win3)
		.install();
}
