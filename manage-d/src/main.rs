use ansi_term::Colour::*;
use clap::{App, SubCommand};
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::{color, style, cursor};
use std::io::{Write, stdout, stdin};


// use daemonize::{Daemonize, DaemonizeError};
// use std::fs::File;

// mod mc;

fn main() {
    let matches = App::new("MGMT Manage-d")
        .version("0.1")
        .author("DusterTheFirst <me@dusterthefirst.com>")
        .about("The server management daemon for MGMT")
        .after_help("For more information visit: https://mgmt.dusterthefirst.com")
        .subcommand(SubCommand::with_name("configure")
            .about("Configure different parts of the daemon")
            .subcommand(SubCommand::with_name("token")
                .about("Manage the token for connection to the mgmt system")
            )
        )
        .get_matches();

    unimplemented!();

    // TODO: DETECT DAEMON VS CLI START AND ACT ACCORDINGLY
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
    //     Ok(_) => start_daemon(),
    //     Err(e) => {
    //         match e {
    //             DaemonizeError::LockPidfile(_) => eprintln!("Process already running, exiting"),
    //             _ => eprintln!("Error, {error}. DaemonizeError::{error:?}", error = e)
    //         }
    //     },
    // }
}

fn start_daemon() {
    unimplemented!();
}

// fn add_version() {
//     let version_listing = mc::get_versions(false, false).unwrap();

//     let stdin = stdin();
//     let mut stdout = stdout().into_raw_mode().unwrap();

//     print!("{}Minecraft version to use:{}", color::Fg(color::Green), style::Reset);
//     stdout.flush().unwrap();

//     for c in stdin.events() {
//         let evt = c.unwrap();
//         match evt {
//             Event::Key(Key::Char('\t')) => print!("\ne{}", cursor::Up(1)),
//             Event::Key(Key::Esc) | Event::Key(Key::Ctrl('c')) => {
//                 println!("{}{}", cursor::Left(std::u16::MAX), style::Reset);
//                 break;
//             },
//             _ => {}
//         }
//         stdout.flush().unwrap();
//     }

//     // let mut line = String::new();
//     // stdin.read_line(&mut line).unwrap();

//     // Use shortcuts to manage downloaded versions
//     // println!("{:?}", line);
//     // println!("{:X?}", bincode::serialize(&version_listing).unwrap());
// }
