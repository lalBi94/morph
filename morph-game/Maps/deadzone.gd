extends Area3D

@export var respawn_box_position: Vector3
@export var is_falled: bool = false

func _ready() -> void:
	body_entered.connect(_on_body_entered)

func _on_body_entered(body: Node) -> void:
	print("fall")
	if body is CharacterBody3D:
		body.global_position = respawn_box_position
		body.emit_signal("fall")
	
