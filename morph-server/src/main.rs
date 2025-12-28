use std::thread::JoinHandle;
use morph_server::communication::Communication;

fn main() -> std::io::Result<()> 
{
    const PORT: u16 = 6000;
    let morph_communication: Communication =
        Communication::create(PORT)?;
    println!("On {}", morph_communication.get_port());

    let br: JoinHandle<()> = morph_communication.run_broadcast_thread();
    let cr: JoinHandle<()> = morph_communication.run_client_reception();
    
    let _ = br.join();
    let _ = cr.join();

    Ok(())
}
