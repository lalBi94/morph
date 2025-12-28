extends Node3D
class_name morph_client

@export var player: Player
@export var map: GridMap

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
	send_packet(Hello.get_payload(player))
	
func _on_ping_checkup():
	send_packet(Checkup.get_payload(player))

func _ready() -> void:
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

func show_other_player() -> void:
		for i in other_players:
			if !other_players[i].is_visible:
				# degeulasse a changer
				var mesho: MeshInstance3D = MeshInstance3D.new()
				mesho.mesh = load("res://Player/Champions/Perso1/char.res")
				other_players[i].player.add_child(mesho)
				self.add_child(other_players[i].player)
				other_players[i].is_visible = true

			if other_players[i].is_visible:
				other_players[i].player.global_position = other_players[i].content.coords
				other_players[i].player.rotation = other_players[i].content.rotation
				
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
					other_players.set(mp.content.id, {
						"content": mp.content,
						"player": CharacterBody3D.new(),
						"is_visible": false
					});
				
				show_other_player()
			_:
				pass
		
		print(other_players)
		
		
		
