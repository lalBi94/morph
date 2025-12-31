use std::{process::Command};

pub fn open_game_windows(with_cmd: bool) -> ()
{
    let path: &'static str = 
        if with_cmd  { r"C:\Users\PC\Desktop\morph\morph-game\build.console.exe"} 
        else { r"C:\Users\PC\Desktop\morph\morph-game\build.exe" } ;

    Command::new("cmd")
        .args(&["/C", "start", "", path])
        .spawn()
        .expect("Impossible de lancer le programme");
}