use std::fs;
use std::env;
use structopt::StructOpt;

fn main()  -> Result<(), std::io::Error> {
    match DronePlay::from_args() {
        DronePlay::Cage {user, path} => { 
            let restricted= format!("{} ALL=(ALL:ALL) {} safeword\n", user, path);
            let file = format!("/etc/sudoers.d/{}", user);
            fs::write(file, restricted)?;
            println!("{} has been caged", user)
        },
        DronePlay::Safeword {} => {
            match env::var("SUDO_USER") {
                Ok(username) => {
                    let restored = format!("{} ALL=(ALL:ALL) ALL\n", username);
                    let file = format!("/etc/sudoers.d/{}", username);
                    fs::write(file, restored)?;
                    println!("{} has used their safeword", username)
                },
                Err(e) => println!("Safewoord needs to be run with sudo: {}", e)
            }
            
        }
    }
    Ok(())
}
#[derive(StructOpt, Debug)]
#[structopt(name = "droneplay", about = "the droneplay cli")]
enum DronePlay {
    #[structopt(name = "cage")]
    Cage {
        #[structopt()]
        user: String,
        #[structopt(short, long, default_value="/usr/local/bin/droneplay")]
        path: String,
    },
    #[structopt(name = "safeword")]
    Safeword {
    }
}
