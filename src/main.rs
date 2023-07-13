use std::{env, fs, io};
use std::error::Error;
use std::fs::{DirEntry, OpenOptions};
use std::io::Read;
use std::path::Path;
use termimad::crossterm::style::{Attribute::*, Color::*};
use termimad::*;

struct Runs {
    runs: Vec<Run>
}

struct Run {
    name: String,
    file: String,
    path: DirEntry

}

fn main() -> Result<(), Box<dyn Error>> {

    dotenvy::dotenv()?;
    let run_path = env::var("RUNSFOLDER").expect("RUNSFOLDER has to defined int .env file");
    println!("Run Path: {}", run_path);
    let mut runs = Runs{runs: Vec::new()};
    println!("Runs Found: ");
    let mut i = 1;
     match fs::read_dir(run_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            let dir_entry = path.unwrap();
            println!("> {:?} - {:?}",i, dir_entry.file_name());
            runs.runs.push(Run{
                name: dir_entry.path().file_stem().expect("lol").to_os_string().into_string().expect("second?"),
                file: dir_entry.file_name().into_string().expect("WTF?"),
                path: dir_entry
            });
            i += 1
        },
    }

    let run_id = selection_loop();
    let run_active = &runs.runs.get(run_id).expect("Idiot you should have picked a valid number");
    println!("Selected Run: {:?} from File: {:?}", run_active.name, run_active.file);
    action_menu_loop(run_active);
    Ok(())
}

fn selection_loop() -> usize {
    loop {
        println!("Please Select Run with the number:");
        let input = read_input();
        match input.trim().parse::<usize>() {
            Ok(x) => return x-1,
            Err(_) => {
                println!("Wrong Input {:?}", input);
                continue
            }
        };
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    input
}

fn action_menu_loop(run: &Run){
    println!("{:?}", run.path.path());
    let mut file = OpenOptions::new()
        .read(true)
        .open(run.path.path())
        .unwrap();
    loop {
        println!("Action-Menu Actions: show, karma, exit");
        let input = read_input();
        match input.trim() {
            "show" => {
                let mut content  = String::new();
                file.read_to_string(&mut content).expect("again");
                println!("{:?}",content);
                termimad::print_inline(&content.as_str());
                continue
            }
            _ => {}
        }
        break;
    }
}

