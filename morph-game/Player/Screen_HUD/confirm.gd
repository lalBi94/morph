extends Button

@export var ipt_id: LineEdit

var stats: PlayerStats

func set_stats(pstats: PlayerStats) -> void:
	self.stats = pstats

func _ready() -> void:
	print(stats)
	self.pressed.connect(_press_btn)
	_press_btn()

func _press_btn():
	stats.player_eternal_id = ipt_id.text
	print(ipt_id.text)
