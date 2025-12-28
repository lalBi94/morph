use std::{collections::HashMap, net::UdpSocket, sync::{Arc, RwLock}, time::{Duration, Instant}};

use crate::{communication::{Communication, Payload}, player::Player};

pub fn client_recepetion(
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: UdpSocket
)
-> ()
{
    let mut buf: [u8; 1024] = [0u8; 1024];
    let mut index: usize = 0;
    
    loop 
    {
        let perf: Instant = Instant::now();
        println!("-------------------[Client Reception n.{}]-------------------", index);

        match socket.recv_from(&mut buf) 
        {
            Ok((amt, src)) => 
            {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                let mut payload_iter: std::str::Split<'_, char> = msg.split('|');
                let mut payload: Option<Payload> = None;

                if let Some(v) = payload_iter.next() {
                    match v {
                        "HERE" => {
                        }

                        "CHECKUP" => {
                            payload = Communication::interpret_checkup(payload_iter)
                        }

                        _ => {}
                    }
                }
                
                if let Some(v) = payload {
                    match &v {
                        Payload::CheckUpPayload(player_check_payload) => {
                            let mut clients_write = clients.write().unwrap();

                            if !clients_write.contains_key(&player_check_payload.get_eid())
                            {
                                let player: Player = Player::create(src.to_string(), player_check_payload.get_eid());
                                clients_write.insert(player_check_payload.get_eid(), Arc::new(RwLock::new(player)));
                            } else 
                            {
                                if let Some(p) = clients_write.get(&player_check_payload.get_eid()) {
                                    let mut p_write = p.write().unwrap();
                                    p_write.seen();
                                    p_write.set_coords(player_check_payload.get_coords());
                                    p_write.set_hp(player_check_payload.get_hp());
                                    p_write.set_rotation(player_check_payload.get_rotation());
                                }
                                println!("Reçu de {}: {}", &player_check_payload.get_eid(), msg);
                            }
                        },
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(_e) => {}
        }

        println!(
            "[{}us]----------[END Client Reception n.{}]-------------------", 
            perf.elapsed().as_micros(), index
        );
        
        std::thread::sleep(Duration::from_millis(1));

        index += 1;
    }
}