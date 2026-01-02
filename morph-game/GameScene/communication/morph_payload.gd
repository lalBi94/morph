extends Node
class_name MorphPayload

enum PayloadType {
	Snapshot,
	Correction,
	Neighbors
}

class spayload:
	var type: PayloadType
	var content
	
	func set_type(typ: PayloadType) -> void:
		self.type = typ
		
	func set_content(ctt) -> void:
		self.content = ctt

static func process_server_payload(payload: PackedStringArray) -> spayload:
	var pyl: spayload = spayload.new()
	
	match payload[0]:
		"BRD_NEIGHBORS":
			pyl.set_type(PayloadType.Neighbors)
			var data_for_one_player_count = 13
			var internal_ptf_key_cursor: int = 0
			var stock_ptf: Array[Dictionary] = []
			
			var current_ptf_treat = {
				"id": null,
				"hp": null,
				"coords_x": null,
				"coords_y": null,
				"coords_z": null,
				"rotation_x": null,
				"rotation_y": null,
				"rotation_z": null,
				"velocity": null,
				"velocity_space_x": null,
				"velocity_space_y": null,
				"velocity_space_z": null,
				"is_disconnected": null,
			}
			
			var referencial_word_order = {
				0: ["String", "id"],
				1: ["double", "hp"],
				2:[ "double", "coords_x"],
				3: ["double", "coords_y"],
				4: ["double", "coords_z"],
				5: ["double", "rotation_x"],
				6: ["double", "rotation_y"],
				7: ["double", "rotation_z"],
				8: ["double", "velocity"],
				9: ["double", "velocity_space_x"],
				10: ["double", "velocity_space_y"],
				11: ["double", "velocity_space_z"],
				12: ["bool", "is_disconnected"],
			}
			
			for i in range(1, payload.size()):
				var current_word: String = payload[i]
				var word_type = referencial_word_order[internal_ptf_key_cursor][0]
				var word_key  = referencial_word_order[internal_ptf_key_cursor][1]
				var current_data = null
				
				match word_type:
					"String": current_data = current_word;
					"double": current_data = current_word.to_float();
					"bool": current_data = current_word == "true";
					_: return;
	
				current_ptf_treat[word_key] = current_data
				internal_ptf_key_cursor += 1
				
				if internal_ptf_key_cursor >= data_for_one_player_count:
					stock_ptf.append(current_ptf_treat.duplicate(true))
					internal_ptf_key_cursor = 0
					current_ptf_treat = {
						"id": null,
						"hp": null,
						"coords_x": null,
						"coords_y": null,
						"coords_z": null,
						"rotation_x": null,
						"rotation_y": null,
						"rotation_z": null,
						"velocity": null,
						"velocity_space_x": null,
						"velocity_space_y": null,
						"velocity_space_z": null,
						"is_disconnected": null,
					}
					
			pyl.set_content(stock_ptf)
		_:
			pass
			
	return pyl
	
