use core::time;
use std::{collections::HashMap, net::UdpSocket, sync::{Arc, RwLock}, thread::yield_now, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use crossterm::event::Event;

use crate::{communication::{events::{ClientEvent, PhysicalEvent}}, coords::Coords, player::Player};

const SPAWN_COORDS: Coords = Coords { x: -171.2462, y: 2.546574, z: 33.07719 };

pub fn client_recepetion(
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: UdpSocket,
)
-> ()
{
    let interception_frequency_hz: f64 = 60.0;
    let interception_interval: Duration = Duration::from_secs_f64(1.0 / interception_frequency_hz);
    let mut last_broadcast: Instant = Instant::now();

    loop 
    {
        if last_broadcast.elapsed() >= interception_interval 
        {
            last_broadcast = Instant::now();
            client_recepetion_process(
                Arc::clone(&clients), 
                &socket
            );
        }
        
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn client_recepetion_process(
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: &UdpSocket
)
-> ()
{
    let mut buf: [u8; 4096] = [0u8; 4096];
    
    let perf: Instant = Instant::now();
    println!("-------------------[Snapshot n.{}]-------------------", 0);

    let frequency_hz: f64 = 60.0;
    let interval: Duration = Duration::from_secs_f64(1.0 / frequency_hz);
    let mut last_call: Instant = Instant::now();

    loop
    {
        if last_call.elapsed() >= interval {
            println!("> End reception");
            last_call = Instant::now();
            break;
        } 

        match socket.recv_from(&mut buf)
        {
            Ok((amt, src)) => 
            {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                println!("{msg}");
                let mut payload_iter: std::str::Split<'_, char> = msg.split('|');
                let mut payload: Option<ClientEvent> = None;

                if let Some(v) = payload_iter.next() {
                    let parsed: ClientEvent = ClientEvent::parse_string(v.to_string());
                    payload = Some(parsed);
                }
                
                if let Some(v) = payload 
                {
                    let re_to_string: String = payload_iter.collect::<Vec<&str>>().join("|");
                    v.process(re_to_string, todo!());
                }
            }
            Err(ref e) => if e.kind() == std::io::ErrorKind::WouldBlock {}
        }
    }

    println!(
        "[{}us]----------[END Snapshot Reception n.{}]-------------------", 
        perf.elapsed().as_micros(), 0
    );
}