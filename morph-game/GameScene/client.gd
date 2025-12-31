extends Node3D
class_name morph_client

@export var ab_player: AbstractPlayer
@export var map: GridMap
@export var pl_stats: PlayerStats

@onready var texture_face_champion: TextureRect = $Control/FaceNLevel/TextureRect

const DISTANCE_TO_BE_CONSIDERATE: float = 40.0

var other_players: Dictionary = {}
var udp = PacketPeerUDP.new()

func create_frequencial_call(fn: Callable, freq: float):
	var timer = Timer.new()
	timer.wait_time = freq
	timer.autostart = true
	timer.one_shot = false
	timer.connect("timeout", fn)
	add_child(timer)

func _on_ping_hello():
	send_packet(Hello.get_payload(ab_player.player))
	
func _on_ping_checkup():
	send_packet(Checkup.get_payload(ab_player.player))
	
func _enter_tree() -> void:
	self.ab_player.init(pl_stats)
	var confirm_btn: Button = $Control/DBG_EID/confirm
	confirm_btn.set_stats(pl_stats)
	var lbl: Label = $Control/FaceNLevel/Label
	lbl.text = pl_stats.champion.name

func _ready() -> void:
	texture_face_champion.texture = pl_stats.champion.logo
	
	var err = udp.connect_to_host("127.0.0.1", 6000)
	if err:
		return
	
	create_frequencial_call(_on_ping_hello, Hello.get_freq())
	create_frequencial_call(_on_ping_checkup, Checkup.get_freq())

func send_packet(payload: PackedByteArray):
	if not udp:
		return
	udp.put_packet(payload)
	#print("Payload", ": ", payload, payload.get_string_from_utf8())

func get_distance_between(fcoords: Vector3, scoords: Vector3) -> float:
	return sqrt(
		pow(fcoords.x-scoords.x, 2)+
		pow(fcoords.z-scoords.z, 2)
	)

func show_other_player() -> void:
	for i in other_players:
		var gp_oplayer: Vector3 = other_players[i].content.coords
		var gp_mplayer: Vector3 = ab_player.player.global_position
		var distance_between_me_and_oplayer: float = get_distance_between(gp_mplayer, gp_oplayer)
		
		#Si les dernieres coordonees du joueur sont trop loin de moi et que le joueur est visible, le cacher
		#Sinon, si le joueur est invisible et pres de moi, spawn son perso avec ses data de positionnement et axage
		#Sinon, si le joueur est visible et pres de moi, changer ses data de positionnement et axage
		
		if other_players[i].first_spawn:
			self.add_child(other_players[i].player)
			other_players[i].first_spawn = false
		
		if !other_players[i].first_spawn && (distance_between_me_and_oplayer > DISTANCE_TO_BE_CONSIDERATE) && other_players[i].is_visible:
			print("hide player ", other_players[i].player)
			other_players[i].player.hide()
			other_players[i].is_visible = false
			
		if !other_players[i].first_spawn && !other_players[i].is_visible && (distance_between_me_and_oplayer <= DISTANCE_TO_BE_CONSIDERATE):
			print("showdw ", other_players[i].player)
			other_players[i].player.show()
			other_players[i].is_visible = true
			
		if !other_players[i].first_spawn && other_players[i].is_visible && (distance_between_me_and_oplayer <= DISTANCE_TO_BE_CONSIDERATE):
			print("move player ", other_players[i].player)
			other_players[i].player.position = other_players[i].content.coords
			other_players[i].player.rotation = other_players[i].content.rotation
		
		print(gp_oplayer, " ", gp_mplayer, " dis: ", distance_between_me_and_oplayer);
		print("????",other_players[i])
			
func _process(_delta: float) -> void:
	while udp.get_available_packet_count() > 0:
		var incoming_payload: PackedByteArray = udp.get_packet()
		var payload_string: String = incoming_payload.get_string_from_utf8()
		var splited_payload: PackedStringArray = payload_string.split("|")
		
		var mp: Dictionary = MorphPayload.process_server_payload(splited_payload)
		var mp_type: MorphPayload.PayloadType = mp.type;
		match mp_type:
			MorphPayload.PayloadType.BRD_NEIGH:
				if other_players.has(mp.content.id):
					other_players[mp.content.id].content = mp.content
				else:
					var ps_stats: PlayerStats = load("res://Player/Statistics.tres")
					var ps: PackedScene = load("res://Player/Champions/Perso2/perso_2.tscn")
					
					if ps.can_instantiate():
						var pm: Player = ps.instantiate() as Player
						pm.set_stats(ps_stats)
						pm.gridmap = ab_player.gridmap
						pm.dead_zone = ab_player.dead_zone
						pm.terrain = ab_player.terrain
						pm.set_not_me(true)
						pm.scale = $Player.scale
						
						other_players.set(mp.content.id, {
							"content": mp.content,
							"player": pm,
							"is_visible": false,
							"first_spawn": true
						});
			_:
				pass
	
	show_other_player()
		
		
		
