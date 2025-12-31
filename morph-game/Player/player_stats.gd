extends Resource
class_name PlayerStats

@export var max_hp: float = 100.0
@export var max_mana: int = 100
@export var min_lvl: int = 1
@export var move_speed: float = 10.0
@export var jump_velocity: float = 6
@export var start_money: float = 50.0

@export var champion: ChampStats
@export var deltaplane_mesh: PackedScene
@export var chest_mesh: PackedScene
@export var player_eternal_id: String

signal update_hp(current_hp, current_max_hp)
signal update_mana(current_mana, current_max_mana)
signal update_money(current_money)
signal update_level(current_level)

# Attributes
var hp: float = -1
var money: float = -1
var level: int = -1
var mana: int = -1

func reset_runtime() -> void:
	self.hp = max_hp
	self.money = start_money
	self.level = min_lvl
	self.mana = max_mana
	refresh_hud(target_attr.ALL)

enum target_attr {
	ALL = 0,
	HP = 1,
	MANA = 2,
	MONEY = 3,
	LEVEL = 4
}

func refresh_hud(tatt: target_attr) -> void:
	if tatt == target_attr.ALL || tatt == target_attr.HP:
		emit_signal("update_hp", self.hp, self.max_hp)
	if tatt == target_attr.ALL || tatt == target_attr.MANA:
		emit_signal("update_mana", self.mana, self.max_mana)
	if tatt == target_attr.ALL || tatt == target_attr.MONEY:
		emit_signal("update_money", self.money)
	if tatt == target_attr.ALL || tatt == target_attr.LEVEL:
		emit_signal("update_level", self.level)

func fall() -> void:
	self.hp = 0.0
	refresh_hud(target_attr.HP)

func walk_on_lava() -> void:
	if self.hp <= 0.0:
		return
	
	self.hp -= 0.15
	refresh_hud(target_attr.HP)
