#[macro_use] extern crate lazy_static;

use std::net::{TcpListener, SocketAddr, Ipv4Addr, IpAddr, TcpStream};
use std::io::{self, BufReader, BufWriter};
use std::thread;
use ansi_term::Colour::*;
use bollard::{Docker, image::{ListImagesOptions}};
use tokio::prelude::*;
use tokio::runtime::Runtime;

use core::protocol::local;
use core::PacketStream;

// use daemonize::{Daemonize, DaemonizeError};
use std::fs::File;

/// The port used to communicate with the CLI
static PORT_LOCAL: u16 = 9895;
/// The protocol version used to communicate with the CLI
static PROTOCOL_VERSION_LOCAL: u8 = 0;

/// The docker image to use for minecraft servers
static DOCKER_IMAGE: &str = "itzg/minecraft-server";
static DOCKER_TAG: &str = "latest";
lazy_static! {
    static ref CONFIG_FILE: File = File::create("/tmp/manage-d.conf").unwrap();
}

fn main() {
    // let stdout = File::create("/tmp/daemon.out").unwrap();
    // let stderr = File::create("/tmp/daemon.err").unwrap();

    // let daemonize = Daemonize::new()
    //     .pid_file("/tmp/test.pid") // Every method except `new` and `start`
    //     .chown_pid_file(true)      // is optional, see `Daemonize` documentation
    //     .working_directory("/tmp") // for default behaviour.
    //     // .user("nobody")
    //     // .group("daemon") // Group name
    //     // .group(2)        // or group id.
    //     .umask(0o777)    // Set umask, `0o027` by default.
    //     .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
    //     .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
    //     .privileged_action(|| "Executed before drop privileges");

    // match daemonize.start() {
    //     Ok(_) => start(),
    //     Err(e) => {
    //         match e {
    //             DaemonizeError::LockPidfile(_) => eprintln!("Process already running, exiting"),
    //             _ => eprintln!("Error, {error}. DaemonizeError::{error:?}", error = e)
    //         }
    //     },
    // }
    start();
}

fn start() {
    // println!("Success, daemonized");
    println!("{}", Blue.paint("Connecting to docker"));

    // Connect to docker
    #[cfg(unix)]
    let docker = Docker::connect_with_unix_defaults().unwrap();
    #[cfg(windows)]
    let docker = Docker::connect_with_named_pipe_defaults().unwrap();
    println!("{}", Green.paint("Connected to docker"));

    // Create a new tokio runtime
    let mut runtime = Runtime::new().unwrap();

    // Get the docker version
    let docker_version = runtime.block_on(docker.version()).unwrap();
    println!("Docker Version: {}", docker_version.ApiVersion);

    // // Make sure the itzg/minecraft-server docker image exists
    // println!("{}", Blue.paint("Looking for the `itzg/minecraft-server` docker image"));
    // let images = runtime.block_on(docker.list_images(Some(ListImagesOptions::<String> {
    //     all: true,
    //     ..Default::default()
    // }))).unwrap();

    // // Get the names of the images
    // let image_names: Vec<String> = images.into_iter().map(|image| image.repo_tags.unwrap()).flatten().collect();
        
    // let image = format!("{}:{}", DOCKER_IMAGE, DOCKER_TAG);

    // let image_exists = image_names.contains(&image);

    // if image_exists {
    //     println!("{}", Green.italic().paint("MC Docker image found"));
    // } else {
    //     println!("{}", Red.italic().paint("MC Docker image not found, downloading it now"));
    //     docker.create_image(Some());
    //     // let download = runtime.block_on(docker.images().pull(&PullOptions::builder().image(DOCKER_IMAGE).tag("latest").build()).into_future()).expect("Pe");
    //     // println!("{}\n{}", Yellow.paint("Downloaded"), download);
    // }

    // runtime.shutdown_now().wait().unwrap();

    println!("{}", Yellow.paint(format!("Listening for CLI connections on port {}", PORT_LOCAL)));
    
    let listener = TcpListener::bind(
        SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(127,0,0,1)
            ),
            PORT_LOCAL
        )
    ).expect("Unable to bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            },
            Err(e) => eprintln!("{}", Red.paint(format!("{:?}", e)))
        }
    }
}

fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    println!("{} {}", Blue.paint("New CLI connection:"), stream.peer_addr().unwrap());

    let mut packet_stream = PacketStream::new(BufReader::new(&stream), BufWriter::new(&stream));

    // Welcome client
    packet_stream.write_packet(local::ToCLI::Welcome {
        protocol_version: PROTOCOL_VERSION_LOCAL
    })?;

    while packet_stream.is_open() {
        let packet: local::ToManageD = packet_stream.read_packet()?;

        match packet {
            local::ToManageD::Ping(x) => packet_stream.write_packet(local::ToCLI::Pong(x))?
        };

        println!("{} {}",
            RGB(50, 150, 150).paint("[Packet Recieved]"),
            RGB(150, 150, 150).paint(format!("{:#?}", packet)));
    }
    println!("{} {}", Yellow.paint("Closed CLI connection: "), stream.peer_addr()?);

    Ok(())
}