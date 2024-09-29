use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::{thread, time};

fn show_ascii_art() {
    println!(
        "
 ## ##   ####       ####               ####   ## ##   ##  ###  ### ##   ###  ##    ##     ####     
##   ##   ##         ##                 ##   ##   ##  ##   ##   ##  ##    ## ##     ##     ##      
##        ##         ##                 ##   ##   ##  ##   ##   ##  ##   # ## #   ## ##    ##      
##        ##         ##                 ##   ##   ##  ##   ##   ## ##    ## ##    ##  ##   ##      
##        ##         ##             ##  ##   ##   ##  ##   ##   ## ##    ##  ##   ## ###   ##      
##   ##   ##  ##     ##             ##  ##   ##   ##  ##   ##   ##  ##   ##  ##   ##  ##   ##  ##  
 ## ##   ### ###    ####             ## #     ## ##    ## ##   #### ##  ###  ##  ###  ##  ### ###  
"
    );
}

fn loading_animation(message: &str) {
    let spinner = ['|', '/', '-', '\\'];
    print!("{} ", message);
    io::stdout().flush().unwrap();

    for i in 0..15 {
        print!("\r{} {}", message, spinner[i % spinner.len()]);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
    println!("\r{}... Done!", message);
}

fn main() {
    show_ascii_art();

    println!("Choose an option: [1] Add Entry, [2] View Entries, [3] Search Entries");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => {
            let date = Utc::now().format("%Y-%d-%m").to_string();
            println!("Enter your journal entry for today:");

            let mut entry = String::new();
            io::stdin()
                .read_line(&mut entry)
                .expect("Failed to read input");

            loading_animation("Adding entry");

            if let Err(e) = add_entry(&date, &entry) {
                eprintln!("Error writing entry: {},", e);
            } else {
                println!("Entry added successfully!")
            }
        }
        "2" => {
            loading_animation("Retrieving entries");

            if let Err(e) = view_entries() {
                eprintln!("Error viewing entries: {}", e);
            }
        }
        "3" => {
            println!("Enter the date for search (YYYY-DD-MM):");
            let mut search_date = String::new();
            io::stdin()
                .read_line(&mut search_date)
                .expect("Failed to read input");

            loading_animation("Searching entries");

            if let Err(e) = search_by_date(&search_date.trim()) {
                eprintln!("Error searching entries: {}", e);
            }
        }
        _ => println!("Invalid option"),
    }
}

fn add_entry(date: &str, entry: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("journal.txt")?;

    writeln!(file, "{}: {}", date, entry)?;
    Ok(())
}

fn view_entries() -> io::Result<()> {
    let file = File::open("journal.txt")?;
    let reader = BufReader::new(file);

    println!("==== Journal Entries ====");

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn search_by_date(search_date: &str) -> io::Result<()> {
    let file = File::open("journal.txt")?;
    let reader = BufReader::new(file);

    println!("==== Journal Entries for {} ====", search_date);

    for line in reader.lines() {
        let line = line?;
        if line.contains(search_date) {
            println!("{}", line);
        }
    }

    Ok(())
}
