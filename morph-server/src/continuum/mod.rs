pub mod monster;
use std::{collections::HashMap, net::UdpSocket, sync::{Arc, RwLock}, thread::{self, JoinHandle}, time::Instant};
use crate::{communication::Communication, continuum::monster::{MonsterCamp, MonsterCampId}};

type SocketID = u16;

const SOCKET_BEGIN_PORT: u16 = 6500;

pub struct Continuum
{
    begin: Instant,
    is_day: bool,
    monsters_camps_position: HashMap<MonsterCampId, MonsterCamp>,
    sockets: Arc<RwLock<HashMap<SocketID, Communication>>>,
    sockets_generator_id: u16
}

impl Continuum
{
    pub fn create() -> Self
    {
        Self {
            begin: Instant::now(),
            is_day: false,
            monsters_camps_position: HashMap::new(),
            sockets: Arc::new(RwLock::new(HashMap::new())),
            sockets_generator_id: 0
        }
    }

    pub fn remove_socket(&mut self, socket_id: SocketID)
    -> ()
    {
        let s_writer = self.sockets.write();
        
        if let Ok(mut sockets) = s_writer
        {
            sockets.remove(&socket_id);
        }
    }

    pub fn get_communication_at(&self, socket_id: SocketID)
    -> Option<Communication>
    {
        let read_sockets = self.sockets.read();

        if let Ok(rs) = read_sockets
        {
            let communication: Option<&Communication> = rs.get(&socket_id);
            if let Some(com) = communication
            {
                Some(com.duplicate_self())
            } else 
            {
                None
            }
        } 
        else 
        {
            None
        }
    }

    pub fn create_socket(&mut self) 
    -> Option<UdpSocket>
    {
        let communication: Result<Communication, std::io::Error> = 
            Communication::create(SOCKET_BEGIN_PORT + self.sockets_generator_id);
        
        if let Ok(com) = communication
        {
            let s_writer = self.sockets.write();

            if let Ok(mut s) = s_writer
            {
                s.insert(
                    self.sockets_generator_id, 
                    com.duplicate_self()
                );

                self.sockets_generator_id += 1;

                Some(com.duplicate_socket())
            } 
            else
            {
                None
            }
        }
        else 
        {
            None
        }
    } 
}