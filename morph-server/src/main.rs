use std::{collections::HashMap, net::UdpSocket, sync::{Arc, RwLock}, thread::{self, JoinHandle}};
use morph_server::{communication::{Communication, broadcast::{Periodic, PeriodicPool}}, continuum::Continuum, player::Player, terminal::open_game_windows};

const IS_DEV_GAME_MODE: bool = false;

fn game_dev_treat() -> JoinHandle<()>
{
    thread::spawn(move || { 
        open_game_windows(true);

        let args: Vec<String> = std::env::args().collect();
        if let Some(v) = args.get(1)
        {
            let player_count: usize = v.parse::<usize>().unwrap_or_default();

            for i in 0..player_count
            {
                println!("Open %{}", i);
                open_game_windows(false);
            }
        }
    })
}

fn _night_day(
    players: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: &UdpSocket
) -> () {
    println!("> _night_day exec()");
}

fn _monster_appear(
    players: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: &UdpSocket
) -> () {
    println!("> _monster_appear exec()");
}

fn _actif_monster(
    players: Arc<RwLock<HashMap<String, Arc<RwLock<Player>>>>>, 
    socket: &UdpSocket
) -> () {
    println!("> _actif_monster exec()");
}

fn main() -> std::io::Result<()> 
{
    let mut ctn: Continuum = Continuum::create();
    let ctn_shared_src: Arc<RwLock<Continuum>> =
        Arc::new(RwLock::new(ctn));
    let ctn_shared: Arc<RwLock<Continuum>> =
        Arc::clone(&ctn_shared_src);

    const PORT: u16 = 6000;
    let morph_communication: Communication =
        Communication::create(PORT)?;
    println!("On {}", morph_communication.get_port());

    let mut periodics_pool: PeriodicPool = PeriodicPool::create(
        morph_communication.duplicate_socket(), 
        morph_communication.duplicate_clients()
    );

    periodics_pool.add_event(
        "NightDay", 
        Periodic::create(/*0.001665*/ 0.5, _night_day)
    );

    periodics_pool.add_event(
        "MonsterAppear", 
        Periodic::create(/*0.001665*/ 0.2, _monster_appear)
    );

    periodics_pool.add_event(
        "ActifMonster", 
        Periodic::create(1.0, _actif_monster)
    );

    let run_periodics: JoinHandle<()> = PeriodicPool::run_periodics(periodics_pool);

    //let cr: JoinHandle<()> = morph_communication.run_client_reception();
    
    if IS_DEV_GAME_MODE
    {
        let gms: JoinHandle<()> = game_dev_treat();
        let _ = gms.join();
    }

    let _ = run_periodics.join();
    // let _ = br.join();
    // let _ = cr.join();

    Ok(())
}
