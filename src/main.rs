use nix::unistd::{chown, Uid};
use std::env;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use users::{get_current_uid, get_user_by_name, get_user_by_uid};

fn cage_subcommand(cage: Cage) -> Result<(), std::io::Error> {
    match cage {
        Cage::Lock { username, path } => {
            let restricted = format!("{} ALL=(ALL:ALL) {} cage safeword\n", username, path);
            let file = format!("/etc/sudoers.d/{}", username);
            fs::write(file, restricted)?;
            println!("{} has been caged", username)
        }
        Cage::Unlock { username } => {
            let restored = format!("{} ALL=(ALL:ALL) ALL\n", username);
            let file = format!("/etc/sudoers.d/{}", username);
            fs::write(file, restored)?;
            println!("{} has been released", username)
        }
        Cage::Safeword {} => match env::var("SUDO_USER") {
            Ok(username) => {
                let restored = format!("{} ALL=(ALL:ALL) ALL\n", username);
                let file = format!("/etc/sudoers.d/{}", username);
                fs::write(file, restored)?;
                println!("{} has used their safeword", username)
            }
            Err(e) => println!("Safeword needs to be run with sudo: {}", e),
        },
    }
    Ok(())
}

fn mantra_subcommand(mantra: Mantra) -> Result<(), std::io::Error> {
    match mantra {
        Mantra::Assign {
            username,
            mantra,
            path,
        } => {
            setup_mantra_dir(&path, &username)?;
            println!("mantra assign: {}", mantra);
        }
        Mantra::Recite { path } => {
            if Path::new(&path).join(cur_user()).exists() {
                println!("mantra recite");
            } else {
                println!("wait for your programmer to set up your mantra directory");
            }
        }
        Mantra::Show { username, path } => {
            if Path::new(&path).join(&username).exists() {
                println!("mantra show");
            } else {
                println!("mantra directory does not exist");
            }
        }
        Mantra::Safeword { path } => {
            println!("safeword {}", path);
        }
    }
    Ok(())
}

fn setup_mantra_dir(mantra_dir: &str, username: &str) -> Result<(), std::io::Error> {
    let mantra_path = Path::new(mantra_dir).join(username);
    if !mantra_path.exists() {
        //questions for puppy can i avoid this clone?
        fs::create_dir_all(mantra_path.clone())?;
        // questions for puppy why doesn't nix::unistd error not allow ?
        chown(&mantra_path, Some(user_id(username)), None)?;
    }
    Ok(())
}

fn user_id(username: &str) -> Uid {
    Uid::from_raw(get_user_by_name(username).expect("No such user").uid())
}

fn cur_user() -> String {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    String::from(user.name().to_string_lossy())
}

fn main() -> Result<(), std::io::Error> {
    match DronePlay::from_args() {
        DronePlay::Cage(cage) => {
            cage_subcommand(cage)?;
        }
        DronePlay::Mantra(mantra) => {
            mantra_subcommand(mantra)?;
        }
    }
    Ok(())
}

const MONTRA_DEFAULT: &str = "/var/lib/droneplay-mantra/";

#[derive(StructOpt, Debug)]
#[structopt(name = "mantra")]
enum Mantra {
    #[structopt(name = "assign")]
    Assign {
        #[structopt()]
        username: String,
        #[structopt()]
        mantra: String,
        #[structopt(short, long, default_value=MONTRA_DEFAULT)]
        path: String,
    },
    #[structopt(name = "recite")]
    Recite {
        #[structopt(short, long, default_value=MONTRA_DEFAULT)]
        path: String,
    },
    #[structopt(name = "show")]
    Show {
        #[structopt()]
        username: String,
        #[structopt(short, long, default_value=MONTRA_DEFAULT)]
        path: String,
    },
    #[structopt(name = "safeword")]
    Safeword {
        #[structopt(short, long, default_value=MONTRA_DEFAULT)]
        path: String,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cage")]
enum Cage {
    #[structopt(name = "lock")]
    Lock {
        #[structopt()]
        username: String,
        #[structopt(short, long, default_value = "/usr/local/bin/droneplay")]
        path: String,
    },
    #[structopt(name = "unlock")]
    Unlock {
        #[structopt()]
        username: String,
    },
    #[structopt(name = "safeword")]
    Safeword {},
}

#[derive(StructOpt, Debug)]
#[structopt(name = "droneplay", about = "the droneplay cli")]
enum DronePlay {
    Cage(Cage),
    Mantra(Mantra),
}

