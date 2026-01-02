use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Player 
{
    id: String,
    addr: String,
    is_deconnected: bool,
    last_seen: Instant
}

impl Player {
    pub fn create(addr: String, id: String) -> Self 
    {
        Self {
            id,
            addr,
            is_deconnected: false,
            last_seen: Instant::now()
        }
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