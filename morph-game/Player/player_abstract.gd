extends Node3D
class_name AbstractPlayer

@export var gridmap: GridMap
@export var dead_zone: Area3D
@export var terrain: Terrain3D
@export var game: morph_client

var champion_is_loaded: bool = false
var player: Player
var stats: PlayerStats

func init(stats_p: PlayerStats) -> void:
	self.stats = stats_p

func load_champion() -> Player:
	var ps: PackedScene = self.stats.champion.charc
	
	if ps.can_instantiate():
		var pm: Player = ps.instantiate() as Player;
		pm.gridmap = self.gridmap
		pm.dead_zone = self.dead_zone
		pm.set_stats(self.stats)
		pm.set_not_me(false)
		pm.terrain = self.terrain
		pm.game = game
	
		self.add_child(pm)
		
		champion_is_loaded = true
		return pm
	
	return null

func _ready() -> void:
	self.player = load_champion()

func _process(_delta) -> void:
	print(self.player.global_position)
