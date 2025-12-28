use std::time::{Duration, Instant};
use crate::{coords::Coords, player_environement_state::PlayerEnvironementState};

#[derive(Debug)]
pub struct PlayerCheckPayload {
    id: String,
    hp: f32,
    coords: Coords,
    rotation: Coords
}

impl PlayerCheckPayload {
    pub fn create(id: String, hp: f32, coords: Coords, rotation: Coords) -> Self 
    {
        Self { id, hp, coords, rotation }
    }

    pub fn get_payload_string(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.hp,
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
            self.coords.x,
            self.coords.y,
            self.coords.z
        )
    }

    pub fn get_hp(&self) -> f32 {
        self.hp
    }

    pub fn get_rotation(&self) -> Coords {
        Coords::owned_coords(&self.rotation)
    }

    pub fn get_coords(&self) -> Coords {
        Coords::owned_coords(&self.coords)
    }

    pub fn get_eid(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Debug)]
pub struct Player 
{
    id: String,
    addr: String,
    hp: Option<f32>,
    coords: Option<Coords>,
    rotation: Option<Coords>,
    is_deconnected: bool,
    last_seen: Instant,
    environement_state: PlayerEnvironementState
}

impl Player {
    pub fn create(addr: String, id: String) -> Self 
    {
        Self {
            id,
            addr,
            hp: None,
            coords: None,
            is_deconnected: false,
            last_seen: Instant::now(),
            rotation: None,
            environement_state: PlayerEnvironementState::create()
        }
    }

    pub fn set_hp(&mut self, value: f32) -> ()
    {
        self.hp = Some(value);
    }

    pub fn get_ref(&mut self) -> &Player 
    {
        self
    }

    pub fn get_hp(&self) -> Option<f32> 
    {
        self.hp
    }

    pub fn clear_players_around(&mut self) -> ()
    {
        self.environement_state.clear_all()
    }

    pub fn get_environnement_state(&self) -> &PlayerEnvironementState
    {
        &self.environement_state
    }

    pub fn add_player_around_player(&mut self, player_cp: PlayerCheckPayload) -> () 
    {
        self.environement_state.add_or_update_player_around_player(
            player_cp
        );
    }

    pub fn seen(&mut self) -> () 
    {
        self.last_seen = Instant::now();
        self.is_deconnected = false;
    }
        
    pub fn set_rotation(&mut self, rotation: Coords) -> () 
    {
        self.rotation = Some(rotation);
    }

    pub fn set_coords(&mut self, coords: Coords) -> () 
    {
        self.coords = Some(coords);
    }

    pub fn deconnect(&mut self) -> () 
    {
        self.is_deconnected = true;
    }

    pub fn is_dead(&self) -> Option<bool> 
    {
        if let Some(v) = self.hp {
            return Some(v <= 0.0);
        }

        None
    }

    pub fn get_addr(&self) -> String 
    {
        self.addr.to_string()
    }

    pub fn elapsed_last_seen(&self) -> Duration 
    {
        self.last_seen.elapsed()
    }

    pub fn get_rotation(&self) -> Option<Coords> 
    {
        if let Some(v) = &self.rotation
        {
            Some(Coords::owned_coords(v))
        } else { None }
    }


    pub fn get_coords(&self) -> Option<Coords> 
    {
        if let Some(v) = &self.coords
        {
            Some(Coords::owned_coords(v))
        } else { None }
    }

    pub fn is_deconnected(&self) -> bool 
    {
        self.is_deconnected
    }

    pub fn get_id(&self) -> String 
    {
        self.id.to_string()
    }
}