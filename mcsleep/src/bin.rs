use mcsleep::{AsleepServer, Motd, SleepMode, KickMessage};

fn main() {
    // TODO: Keep MOTD and icon of server, just change the version and the players
    let server = AsleepServer::new(Motd::Raw("A faked Minecraft Server".to_owned()), None, SleepMode::WakeOnConnect, KickMessage::Default, 25565);
    
    server.listen_until_wake();

    println!("WOKE");
}