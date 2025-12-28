use std::collections::HashMap;
use crate::player::PlayerCheckPayload;

#[derive(Debug)]
pub struct PlayerEnvironementState 
{
    players: HashMap<String, PlayerCheckPayload>
}

impl PlayerEnvironementState 
{
    pub fn create() 
    -> Self 
    {
        Self {
            players: HashMap::new()
        }
    }

    pub fn clear_all(&mut self) -> () {
        self.players.clear();
    }

    pub fn get_neighbords_len(&self) -> usize {
        self.players.len()
    }

    pub fn add_or_update_player_around_player(&mut self, other_player: PlayerCheckPayload) -> () 
    {
        self.players.insert(other_player.get_eid(), other_player);
    }

    pub fn get_key_set(&self) -> Vec<String>
    {
        self.players.keys().cloned().collect()
    }

    pub fn get_payload_string_at(&self, at: &String) -> Option<String>
    {
        if let Some(v) = self.players.get(at)
        {
            Some(format!("{}|{}", "BRD_NEIGH".to_string(), v.get_payload_string()))
        } 
        else 
        {
            None
        }
    }
}