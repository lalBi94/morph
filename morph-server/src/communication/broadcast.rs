use core::time;
use std::{
    collections::HashMap, net::UdpSocket, sync::{
        Arc, 
        RwLock
    }, thread::{self, JoinHandle, sleep, yield_now}, time::{
        Duration, 
        Instant
    }
};

use crate::{communication::payload::{}, player::{
    Player
}};

const DISTANCE_TO_BE_NEAR: f64 = 20.0;

pub struct PeriodicPool
{
    periodics: HashMap<String, Periodic>,
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: UdpSocket,
}

impl PeriodicPool
{
    pub fn create(
        socket: UdpSocket, 
        clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>
    ) -> Self
    {
        Self {
            periodics: HashMap::new(),
            socket,
            clients
        }
    }

    pub fn run_periodics(mut periodic_pool: PeriodicPool) -> JoinHandle<()>
    {
        thread::spawn(move ||
        {
            loop {
                for periodic in periodic_pool.periodics.iter_mut()
                {
                    if periodic.1.have_to_be_call()
                    {
                        (periodic.1.handler)(
                            Arc::clone(&periodic_pool.clients),
                            &periodic_pool.socket
                        );
                        periodic.1.reset_last_seen();
                        println!("> Periodic {} run & reset  [OK]", periodic.0);
                    }
                }

                sleep(Duration::from_millis(1));
                yield_now();
            }
        })
    }

    pub fn add_event(
        &mut self,
        name: &str, 
        periodic: Periodic
    )
    -> ()
    {
        let inserting: Option<Periodic> = 
            self.periodics.insert(name.to_string(), periodic);
        
        if let None = inserting
        {
            println!("> Event {} registered. [OK]", &name);
        } else 
        {
            println!("> Event {} already registered [ERROR]", &name);
        }
    }
}

pub struct Periodic
{
    hz: f64,
    handler: fn(Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, &UdpSocket) -> (),
    last_seen: Instant
}

impl Periodic
{
    pub fn create(
        hz: f64, 
        handler: fn(Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, &UdpSocket) -> ()
    )
    -> Self
    {
        Self { hz, handler, last_seen: Instant::now() }
    }

    pub fn reset_last_seen(&mut self) -> ()
    {
        self.last_seen = Instant::now()
    }

    pub fn have_to_be_call(&self) -> bool 
    {
        let broadcast_interval = Duration::from_secs_f64(1.0 / self.hz);
        self.last_seen.elapsed() >= broadcast_interval 
    }
}

// pub fn broadcast(
//     clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
//     socket: UdpSocket
// ) -> () 
// {
//     let broadcast_frequency_hz: f64 = 20.0;
//     let broadcast_interval: Duration = Duration::from_secs_f64(1.0 / broadcast_frequency_hz);
//     let mut last_broadcast: Instant = Instant::now();
//     let mut index: usize = 0;

//     loop {
//         if last_broadcast.elapsed() >= broadcast_interval 
//         {
//             last_broadcast = Instant::now();
//             broadcast_process(Arc::clone(&clients), &socket, index);
//             index += 1;
//         }
        
//         std::thread::sleep(Duration::from_millis(10));
//     }
// }

// fn broadcast_process(
//     clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
//     socket: &UdpSocket,
//     tag: usize
// ) -> () 
// {
//     let perf: Instant = Instant::now();
//     println!("-------------------[Broadcast n.{}]-------------------", tag);
    
//     let clients_read = clients.read().unwrap();

//     let keys: Vec<String> = clients_read.keys().cloned().collect();

//     for k in keys.iter() {
//         let cl = clients_read.get(k);

//         if let Some(p) = cl
//         {
//             let player_cl = p.read();

//             if let Ok(pl) = player_cl
//             {
//             }
//         }
//     }

//     println!(
//         "> Players known they neighbors"
//     );

//     println!(
//         "> Players are ready for broadcast."
//     );

//     println!(
//         "[{}us]-----------[END Broadcast n.{}]-------------------", 
//         perf.elapsed().as_micros(), tag);
// }