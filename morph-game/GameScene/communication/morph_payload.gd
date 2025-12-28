extends Node
class_name MorphPayload

enum PayloadType {
	BRD_NEIGH = 0
}

static func process_server_payload(payload: PackedStringArray) -> Dictionary:
	match payload[0]:
		"BRD_NEIGH":
			var eternal_id: String = payload[1]
			var hp: float = payload[2].to_float()
			
			var rotation: Vector3 = Vector3(
				payload[3].to_float(),
				payload[4].to_float(),
				payload[5].to_float()
			)
			var coords: Vector3 = Vector3(
				payload[6].to_float(),
				payload[7].to_float(),
				payload[8].to_float()
			)
			
			print("BRD_NEIGH -> ", eternal_id, " ", hp, " ", " ", coords, " ", rotation)
			
			return {
				"type": PayloadType.BRD_NEIGH,
				"content": {
					"id": eternal_id,
					"hp": hp,
					"coords": coords,
					"rotation": rotation
				}
			}
		_:
			return {}
