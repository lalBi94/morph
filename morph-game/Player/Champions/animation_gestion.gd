extends AnimationTree

@export var speed: float
@export var is_on_floor: bool

var is_moving = false
var is_sprinting = false
var is_sneaking = false

func _physics_process(_delta: float) -> void:
	is_moving = speed > 0.0
	is_sprinting = speed > 14.0
	
	if Input.is_action_just_pressed("morph_zoom"):
		is_sneaking = true
	elif Input.is_action_just_released("morph_zoom"):
		is_sneaking = false
