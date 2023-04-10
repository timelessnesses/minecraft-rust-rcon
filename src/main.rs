// ! Rust RCON thingy
// ! Supposed to be builded for the tthc.minecraft.timelessnesses.me
// ! But I like TOML

use rcon;
use toml;
use std;
use tokio;
use ctrlc;

#[tokio::main]
async fn main(){
    let config: toml::Value = toml::from_str(include_str!("../config.toml")).unwrap();

    let mut server = <rcon::Connection<tokio::net::TcpStream>>::builder()
    .enable_minecraft_quirks(true)
    .connect(format!("{}:{}", config["server"]["ip"].as_str().expect("Server IP isn't specified"), config["server"]["port"].as_str().unwrap_or("25575")), config["server"]["password"].as_str().unwrap())
    .await.unwrap();

    println!("Connected to the server! ({})\nRun `exit` command to exit the program or CTRL + C!", config["server"]["ip"].as_str().unwrap());

    ctrlc::set_handler(move || {
        println!("Bye!");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        println!("> ");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        if command == "exit" {
            break; // Nice thing that we don't need to clean up!
        }
        let res = server.cmd(command).await.unwrap();
        println!("Server Response: {}", res);
    }
    println!("Bye!")
}