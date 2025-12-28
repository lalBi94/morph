use std::{
    collections::HashMap, 
    net::{
        SocketAddr, 
        UdpSocket
    }, 
    sync::{
        Arc, 
        RwLock
    }, 
    time::{
        Duration, 
        Instant
    }
};

use crate::player::{
    Player, 
    PlayerCheckPayload
};

pub fn broadcast(
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: UdpSocket
) -> () 
{
    let broadcast_frequency_hz: f64 = 20.0;
    let broadcast_interval: Duration = Duration::from_secs_f64(1.0 / broadcast_frequency_hz);
    let mut last_broadcast: Instant = Instant::now();
    let mut index: usize = 0;

    loop {
        if last_broadcast.elapsed() >= broadcast_interval 
        {
            last_broadcast = Instant::now();
            broadcast_process(Arc::clone(&clients), &socket, index);
            index += 1;
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn broadcast_process(
    clients: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: &UdpSocket,
    tag: usize
) -> () 
{
    let perf: Instant = Instant::now();
    println!("-------------------[Broadcast n.{}]-------------------", tag);
    
    let clients_read = clients.read().unwrap();

    let keys: Vec<String> = clients_read.keys().cloned().collect();
    let mut relation: HashMap<String, Vec<usize>> = HashMap::new(); 

    /*
     * Get near players
     */
    for client in 0..keys.len() 
    {
        for other_client in 0..keys.len() 
        {
            if let Some(pl) = clients_read.get(&keys[client]) {
                let player_read = pl.read().unwrap();

                if let Some(pl2) = clients_read.get(&keys[other_client])
                {
                    let player2_read = pl2.read().unwrap();

                    if player_read.get_id() == player2_read.get_id() {
                        continue;
                    }

                    if let Some(v) = player_read.get_coords() {
                        if let Some(w) = player2_read.get_coords() {
                            let dist: f64 = v.get_distance_between(w);

                            if dist < 5.0 {
                                println!(
                                    "> {:?} enter in {:?} vision distance={:?}", 
                                    player_read.get_id(), player2_read.get_id(), dist
                                );

                                relation
                                    .entry(keys[client].clone())
                                    .or_insert_with(Vec::new)
                                    .push(other_client);
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "> Players known they neighbors"
    );
    
    /*
     * Client modifications before broadscast
     */
    for client in clients_read.iter()
    {
        if let Some(pl) = clients_read.get(client.0) {
            let mut player_write = pl.write().unwrap();
            let is_not_here: bool = player_write.elapsed_last_seen() > Duration::from_secs(10);
            
            if !player_write.is_deconnected() && is_not_here
            {
                println!(
                    "> Player {:?} disconnect. time={:?}s",
                    player_write.get_id(), player_write.elapsed_last_seen().as_secs()
                );
                player_write.deconnect();
            }

            player_write.clear_players_around();

            if relation.contains_key(client.0) 
            {
                if let Some(r) = relation.get(client.0)
                {
                    for neighbors in r.iter() {
                        if let Some(pl) = clients_read.get(&keys[*neighbors])
                        {
                            let pl_read = pl.read().unwrap();

                            player_write.add_player_around_player(
                                PlayerCheckPayload::create(
                                    pl_read.get_id(), 
                                    pl_read.get_hp().unwrap(), 
                                    pl_read.get_coords().unwrap(), 
                                    pl_read.get_rotation().unwrap()
                                )
                            );
                        }   
                    }
                }
            }
        }
    }

    println!(
        "> Players are ready for broadcast."
    );

    /*
     * Broadcast the game state
     */
    for client in clients_read.keys() 
    {
        if let Some(pl) = clients_read.get(client) {
            let player_read = pl.read().unwrap();
            if player_read.is_deconnected() { continue; }

            if let Ok(addr) = player_read.get_addr().parse::<SocketAddr>() 
            {
                /*
                 * Send near players position at single player
                 */
                let keys_op: Vec<String> = player_read
                    .get_environnement_state()
                    .get_key_set();

                for kop in keys_op 
                {
                    let other_payload = player_read
                        .get_environnement_state()
                        .get_payload_string_at(&kop);

                    if let Some(v) = other_payload
                    {
                        let _ = socket.send_to(v.as_bytes(), addr);
                    }
                }

                /*
                 * 
                 */
            }
        }
    }

    println!(
        "[{}us]-----------[END Broadcast n.{}]-------------------", 
        perf.elapsed().as_micros(), tag);
}