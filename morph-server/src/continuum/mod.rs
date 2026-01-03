pub mod monster;
use std::{collections::HashMap, fmt::format, net::UdpSocket, sync::{Arc, RwLock}, thread::{self, JoinHandle}, time::Instant};
use crate::{communication::Communication, continuum::monster::{MonsterCamp, MonsterCampId}, player::Player};

type SocketID = u16;
type EternalId = String;

const SERVER_DESTROY_SOCKET_EVENT_NAME: &str = "DLTS"; // send client -> delete socket
const SERVER_ADD_SOCKET_EVENT_NAME: &str = "ADDS"; //
const SOCKET_BEGIN_PORT: u16 = 6500;

pub struct Population
{
    pid: SocketID,
    players: HashMap<EternalId, Arc<Player>>,
    socket: UdpSocket
}

impl Population
{
    pub fn create(id: SocketID) -> Self
    {
        Self
        {
            pid: id,
            players: HashMap::new(),
            socket: UdpSocket::bind(format!("0.0.0.0:{}", id))
                .expect("Failed to created socket to population.")
        }
    }

    pub fn get_players_len(&self)
    -> usize
    {
        self.players.len()
    }

    pub fn get_socket(&self) -> UdpSocket
    {
        self.socket.try_clone().unwrap()
    }
}

pub struct Continuum
{
    begin: Instant,
    is_day: bool,
    monsters_camps_position: HashMap<MonsterCampId, MonsterCamp>,
    populations: Arc<RwLock<HashMap<SocketID, Arc<RwLock<Population>>>>>,
    sockets_generator_id: u16
}

impl Continuum
{
    pub fn process_population(
        popu: Arc<RwLock<HashMap<SocketID, Arc<RwLock<Population>>>>>
    ) -> JoinHandle<()>
    {
        thread::spawn( move || {
            loop
            {
                let mut deleting_queue: Vec<&SocketID> = Vec::new();

                let popu_read = popu.read();

                if let Ok(v) = popu_read
                {
                    for po in v.iter()
                    {
                        let po_read = po.1.read();
                        if let Ok(vv) = po_read
                        {
                            // delete if empty or one
                            if vv.get_players_len() < 2
                            {
                                deleting_queue.push(po.0);
                            }

                            if vv.get_players_len() == 1
                            {
                                vv.get_socket()
                            }
                        }
                    }
                }
            }
        })
    }

    pub fn create() -> Self
    {
        Self {
            begin: Instant::now(),
            is_day: false,
            monsters_camps_position: HashMap::new(),
            populations: Arc::new(RwLock::new(HashMap::new())),
            sockets_generator_id: SOCKET_BEGIN_PORT
        }
    }

    pub fn remove_population(&mut self, socket_id: SocketID)
    -> ()
    {
        let s_writer = self.populations.write();
        
        if let Ok(mut sockets) = s_writer
        {
            sockets.remove(&socket_id);
        }
    }

    pub fn get_population_at(&self, socket_id: SocketID)
    -> Option<Arc<RwLock<Population>>>
    {
        let read_sockets = self.populations.read();

        if let Ok(rs) = read_sockets
        {
            let popu: Option<&Arc<RwLock<Population>>> = rs.get(&socket_id);
            if let Some(po) = popu
            {
                Some(Arc::clone(po))
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

    pub fn create_population(&mut self) 
    -> Option<SocketID>
    {
        let population = Population::create(self.sockets_generator_id);
        
        let s_writer = self.populations.write();

        if let Ok(mut s) = s_writer
        {
            s.insert(
                self.sockets_generator_id, 
                Arc::new(RwLock::new(population))
            );

            self.sockets_generator_id += 1;
            Some(self.sockets_generator_id-1)
        }
        else
        {
            None
        }
    } 
}