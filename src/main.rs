use chrono::prelude::*;
use std::collections::HashSet;
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

    println!("Choose an option: [1] Add Entry, [2] View Entries, [3] Search Entries by Date, [4] Search Entries by Tag");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => add_journal_entry(),
        "2" => view_entries().expect("Error viewing entries"),
        "3" => search_entries_by_date().expect("Error searching by date"),
        "4" => search_entries_by_tag().expect("Error searching by tag"),
        _ => println!("Invalid option"),
    }
}

fn add_journal_entry() {
    let date = Utc::now().format("%Y-%d-%m").to_string();
    println!("Enter your journal entry for today:");

    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read input");

    println!("Enter tags for this entry (separate by commas, e.g., work, personal):");

    let mut tags_input = String::new();
    io::stdin()
        .read_line(&mut tags_input)
        .expect("Failed to read input");
    let tags: HashSet<String> = tags_input
        .trim()
        .split(',')
        .map(|tag| tag.trim().to_string())
        .collect();

    loading_animation("Adding entry");

    if let Err(e) = add_entry(&date, &entry, &tags) {
        eprintln!("Error writing entry: {},", e);
    } else {
        println!("Entry added successfully!")
    }
}

fn add_entry(date: &str, entry: &str, tags: &HashSet<String>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Journal.txt")?;

    let tags_string = tags.iter().cloned().collect::<Vec<String>>().join(", ");
    writeln!(
        file,
        "{} | {} | Tags: [{}]",
        date,
        entry.trim(),
        tags_string
    )?;
    Ok(())
}

fn view_entries() -> io::Result<()> {
    let file = File::open("journal.txt")?;
    let reader = BufReader::new(file);

    loading_animation("Retrieving entries");

    println!("==== Journal Entries ====");

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn search_entries_by_date() -> io::Result<()> {
    println!("Enter the date to search (YYYY-DD-MM):");

    let mut search_date = String::new();
    io::stdin()
        .read_line(&mut search_date)
        .expect("Failed to read input");

    let file = File::open("journal.txt")?;
    let reader = BufReader::new(file);

    loading_animation("Searching entries");

    println!("==== Journal Entries for {} ====", search_date.trim());

    for line in reader.lines() {
        let line = line?;
        if line.contains(search_date.trim()) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn search_entries_by_tag() -> io::Result<()> {
    println!("Enter the tag to search for (e.g., work, personal):");

    let mut search_tag = String::new();
    io::stdin()
        .read_line(&mut search_tag)
        .expect("Failed to read input");

    let file = File::open("journal.txt")?;
    let reader = BufReader::new(file);

    loading_animation("Searching entries");

    println!("==== Journal Entries for tag: {} ====", search_tag.trim());
    for line in reader.lines() {
        let line = line?;
        if line.contains(&format!("Tags: [{}]", search_tag.trim()))
            || line.contains(&format!("Tags: [{}]", search_tag.trim()))
        {
            println!("{}", line);
        }
    }
    Ok(())
}
