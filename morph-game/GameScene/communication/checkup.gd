extends Node
class_name Checkup

static func get_freq() -> float:
	var T: float = 30.0
	return 1/T

static func get_payload(player: Player) -> PackedByteArray:
	const const_message: String = "CHECKUP";
	print(player.stats.hp)
	var data_type_raw: String = \
		"%s|%f|%f|%f|%f|%f|%f|%f" % \
		[ \
			player.stats.player_eternal_id, \
			player.stats.hp, \
			player.body_mesh.rotation.x, \
			player.body_mesh.rotation.y, \
			player.body_mesh.rotation.z, \
			player.global_position.x, \
			player.global_position.y, \
			player.global_position.z \
		]
	var payload: String = "%s|%s" % [const_message,data_type_raw]
	return payload.to_utf8_buffer()
