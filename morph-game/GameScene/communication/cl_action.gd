extends Node
class_name ClientAction

static func get_freq() -> float:
	var T: float = 20.0
	return 1/T 

static func get_payload(player: Player, me_actions: Array[Player.PossibleAction]) -> PackedByteArray:
	const const_message: String = "CL_ACTION";
	var corp: String = "";
	
	for a in me_actions:
		var act: String = Player.translate_cl_action_to_string(a);
		if act != "Unknown":
			corp += ("|" + act)
	var payload: String = "%s|%s%s" % [const_message, player.stats.player_eternal_id, corp]
	return payload.to_utf8_buffer()
