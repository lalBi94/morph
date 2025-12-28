extends AnimationTree

@export var speed: float
var is_moving = false
var is_sprinting = false
var is_sneaking = false

func _physics_process(delta: float) -> void:
	is_moving = speed > 0.0
	is_sprinting = speed > 7.0
	
	if Input.is_action_just_pressed("morph_zoom"):
		is_sneaking = true
	elif Input.is_action_just_released("morph_zoom"):
		is_sneaking = false
