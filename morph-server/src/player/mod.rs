use std::time::{Duration, Instant};

use crate::coords::Coords;

#[derive(Debug)]
pub struct Player 
{
    id: String,
    addr: String,
    is_deconnected: bool,
    last_position: Coords,
    last_seen: Instant
}

impl Player {
    pub fn create(addr: String, id: String) -> Self 
    {
        Self {
            id,
            addr,
            is_deconnected: false,
            last_seen: Instant::now(),
            last_position: Coords::create(0.0, 0.0, 0.0)
        }
    }

    pub fn set_last_position(&mut self, coords: Coords)
    -> ()
    {
        self.last_position = coords;
    }

    pub fn get_last_position(&self)
    -> Coords
    {
        Coords::owned_coords(&self.last_position)
    }

    pub fn seen(&mut self) -> () 
    {
        self.last_seen = Instant::now();
        self.is_deconnected = false;
    }

    pub fn deconnect(&mut self) -> () 
    {
        self.is_deconnected = true;
    }

    pub fn get_addr(&self) -> String 
    {
        self.addr.to_string()
    }

    pub fn elapsed_last_seen(&self) -> Duration 
    {
        self.last_seen.elapsed()
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