use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use crate::cli_structs::*;
use crate::reg_gen::register::*;
use crate::reg_gen::json_handling::*;
use crate::reg_gen::header_handling::*;
use crate::tui_handler::*;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    Terminal,
};

pub fn add_register_handler(args: AddRegisterArgs) -> Result<(), std::io::Error> {
    let mut register_family = pull_existing_json(args.path.clone());

    if !register_family.register_family_widths.contains(&args.size) {
        register_family.register_family_widths.push(args.size);
    }

    let mut register = Register {
        name: args.name,
        size: args.size,
        fields: vec![],
    };

    loop {
        print!("Would you like to add a(nother) field to this register (y/n): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();

        io::stdin().read_line(&mut response).expect("Failed to read line!");
        response = response.trim().to_string();

        match response.as_str() {
            "y" => {
                register.add_register_field();
            },
            "n" => {
                break;
            },
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }

    register_family.registers.push(register);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .truncate(true)
        .open(args.path.clone())
        .expect("Could not open JSON file!");

    match file.write_all(serde_json::to_string_pretty(&register_family).unwrap().as_bytes()) {
        Ok(_) => {},
        Err(why) => panic!("Couldn't write to {}: {}", args.path, why)
    }

    Ok(())
}

pub fn generate_handler(args: GenerateArgs) -> Result<(), std::io::Error> {
    let register_family = pull_existing_json(args.path);
    // Generate the files
    generate_files(&register_family);
    Ok(())
}

pub fn bootstrap_handler(args: BootstrapArgs) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.path.clone())
        .expect("Could not open or create JSON file!");

    let reg_family = RegisterFamily {
        register_family: args.name.clone(),
        register_family_widths: Vec::new(),
        registers: Vec::new(),
    };

    match file.write_all(serde_json::to_string_pretty(&reg_family).unwrap().as_bytes()) {
        Ok(_) => {},
        Err(why) => panic!("Couldn't write to {}: {}", args.path, why)
    }

    Ok(())
}

pub fn tui_handler(args: TuiArgs) -> Result<(), std::io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(args.path);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
