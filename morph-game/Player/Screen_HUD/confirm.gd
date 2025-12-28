extends Button

@export var ipt_id: LineEdit
@export var player: Player

func _ready() -> void:
	self.pressed.connect(_press_btn)
	_press_btn()

func _press_btn():
	player.stats.player_eternal_id = ipt_id.text
	print(ipt_id.text)
