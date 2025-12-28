extends LineEdit

func generate_random_eid() -> String:
	var chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
	var result = ""
	for i in range(8):
		var index = randi() % chars.length()
		result += chars[index]
	return result

func _ready() -> void:
	self.text = generate_random_eid()
