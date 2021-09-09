use smash::lua2cpp::L2CAgentBase;
use smashline::*;

#[acmd_script(agent = "elight", script = "game_attack100start", category = ACMD_GAME)]
unsafe fn elight_game_attack100start(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.25)
		frame(Frame=9)
		FT_MOTION_RATE(FSM=1)
	});
}

pub fn install() {
	smashline::install_acmd_scripts!(
		elight_game_attack100start,
	);
}
