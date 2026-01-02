pub mod broadcast;
pub mod payload;
pub mod client_reception;

use std::{
    collections::HashMap, net::UdpSocket, sync::{Arc, RwLock}, thread::{self, JoinHandle}
};

use crate::{
    communication::{broadcast::{PeriodicPool}, client_reception::client_recepetion}, player::Player
};

pub struct Communication 
{
    port: u16,
    socket: UdpSocket,
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>
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

    pub fn duplicate_self(&self) 
    -> Self 
    {
        Self {
            port: self.port,
            socket: self.duplicate_socket(),
            clients: self.duplicate_clients()
        }
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