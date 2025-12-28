pub mod broadcast;
pub mod client_reception;

use std::{
    collections::HashMap, 
    net::UdpSocket, 
    sync::{Arc, RwLock}, 
    thread::{self, JoinHandle}
};

use crate::{
    communication::{broadcast::broadcast, client_reception::client_recepetion}, 
    coords::Coords, 
    player::{Player, PlayerCheckPayload}
};

pub enum Payload 
{
    CheckUpPayload(PlayerCheckPayload)
}

pub struct Communication 
{
    port: u16,
    socket: UdpSocket,
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>,
}

impl Communication 
{
    pub fn create(
        port: u16
    ) 
    -> std::io::Result<Self>
    {
        let socket = UdpSocket::bind(("0.0.0.0", port))?;
        socket.set_nonblocking(true)?;

        Ok(
            Self {
                port,
                socket,
                clients: Arc::new(RwLock::new(HashMap::new()))
            }
        )
    }

    pub fn duplicate_clients(
        &self
    ) 
    -> Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>
    {
        Arc::clone(&self.clients)
    }

    pub fn duplicate_socket(
        &self
    ) 
    -> UdpSocket
    {
        self.socket.try_clone().unwrap()
    }
    
    pub fn get_port(
        &self
    ) 
    -> u16
    {
        self.port
    }

    pub fn interpret_checkup(
        mut payload: std::str::Split<'_, char>
    ) 
    -> Option<Payload>
    {
        let eternal_id: Option<&str> = payload.next();
        let hp: Option<&str> = payload.next();

        let rotate_x: Option<&str> = payload.next();
        let rotate_y: Option<&str> = payload.next();
        let rotate_z: Option<&str> = payload.next();

        let position_x: Option<&str> = payload.next();
        let position_y: Option<&str> = payload.next();
        let position_z: Option<&str> = payload.next();

        if eternal_id.is_none() || hp.is_none() || rotate_x.is_none() || 
            rotate_y.is_none() || rotate_z.is_none() || position_x.is_none() || 
            position_y.is_none() || position_z.is_none() 
        {
            return None;
        }

        let eternal_id: String = eternal_id.unwrap().to_string();
        
        let hp: f32 = hp.unwrap().parse::<f32>().unwrap_or(-1.0);
        
        let rotate_x: f64 = rotate_x.unwrap().parse::<f64>().unwrap_or(0.0);
        let rotate_y: f64 = rotate_y.unwrap().parse::<f64>().unwrap_or(0.0);
        let rotate_z: f64 = rotate_z.unwrap().parse::<f64>().unwrap_or(0.0);

        let position_x: f64 = position_x.unwrap().parse::<f64>().unwrap_or(0.0);
        let position_y: f64 = position_y.unwrap().parse::<f64>().unwrap_or(0.0);
        let position_z: f64 = position_z.unwrap().parse::<f64>().unwrap_or(0.0);

        let rotation: Coords = Coords::create(rotate_x, rotate_y, rotate_z);
        let position: Coords = Coords::create(position_x, position_y, position_z);

        return Some(Payload::CheckUpPayload(
            PlayerCheckPayload::create(eternal_id, hp, position, rotation)
        ));
    }

    pub fn run_broadcast_thread(
        &self
    ) 
    -> JoinHandle<()>
    {
        let clients_shared: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>=
            self.duplicate_clients();
        
        let socket_shared: UdpSocket =
            self.duplicate_socket();
        
        thread::spawn(
            move || 
                broadcast(clients_shared, socket_shared)
        )
    }

    pub fn run_client_reception(
        &self
    )
    -> JoinHandle<()>
    {
        let clients_shared: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>=
            self.duplicate_clients();
        
        let socket_shared: UdpSocket =
            self.duplicate_socket();
        
        thread::spawn(
            move || 
                client_recepetion(clients_shared, socket_shared)
        )
    }
}