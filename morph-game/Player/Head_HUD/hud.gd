extends Label3D

@export var stats: PlayerStats

var hud_stat: Dictionary = {
	"hp": "Life - ???",
	"mana": "Mana - ???",
	"money": "Gold - ???",
	"lvl": "Level - ???"
}

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	print(stats)
	stats.update_hp.connect(_update_hp)
	_update_hp(stats.hp, stats.max_hp)
	
	stats.update_mana.connect(_update_mana)
	_update_mana(stats.mana, stats.max_mana)
	
	stats.update_money.connect(_update_money)
	_update_money(stats.money)
	
	stats.update_level.connect(_update_level)
	_update_level(stats.level)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _update_hp(current_hp: int, current_max_hp: int) -> void:
	self.hud_stat["hp"] = "Life - %.2f/%.2f" % [current_hp, current_max_hp]
	self.update_hud()
	
func _update_mana(current_mana: int, current_max_mana: int) -> void:
	self.hud_stat["mana"] = "Mana - %d/%d" % [current_mana, current_max_mana]
	self.update_hud()
	
func _update_money(current_money: float) -> void:
	self.hud_stat["money"] = "Money - %.2f" % [current_money]
	self.update_hud()
	
func _update_level(current_level: int) -> void:
	self.hud_stat["lvl"] = "Level - %d" % [current_level]
	self.update_hud()

func update_hud() -> void:
	self.text = "%s \n %s \n %s \n %s" % [self.hud_stat["hp"], self.hud_stat["mana"], self.hud_stat["money"], self.hud_stat["lvl"]]
