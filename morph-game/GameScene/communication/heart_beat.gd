extends Node
class_name HeartBeat

static func get_freq() -> float:
	var T: float = 3.0
	return 1/T

static func get_payload(player: Player) -> PackedByteArray:
	const const_message: String = "HEART_BEAT";
	var data_type_raw: String = "%s" % player.stats.player_eternal_id;
	var payload: String = "%s|%s" % [const_message,data_type_raw]
	return payload.to_utf8_buffer()
