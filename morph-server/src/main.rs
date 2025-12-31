use std::thread::{self, JoinHandle};
use morph_server::{communication::Communication, terminal::{open_game_windows}};

const IS_DEV_GAME_MODE: bool = true;

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

fn main() -> std::io::Result<()> 
{
    const PORT: u16 = 6000;
    let morph_communication: Communication =
        Communication::create(PORT)?;
    println!("On {}", morph_communication.get_port());

    let br: JoinHandle<()> = morph_communication.run_broadcast_thread();
    let cr: JoinHandle<()> = morph_communication.run_client_reception();
    
    if IS_DEV_GAME_MODE
    {
        let gms: JoinHandle<()> = game_dev_treat();
        let _ = gms.join();
    }

    let _ = br.join();
    let _ = cr.join();

    Ok(())
}
