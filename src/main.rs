use clap::Parser;
use std::env::current_exe;
use std::error::Error;
use std::fs;
use std::fs::{read, write, File};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    name: String,
    todo: Vec<String>,
    #[clap(short, long)]
    remove: bool,
}

fn main() {
    let args = Args::parse();

    if args.todo.is_empty() && args.remove {
        delete_todo(args.name).unwrap();

        return;
    } else if args.todo.is_empty() {
        println!("{}", read_todo(args.name).unwrap());

        return;
    } else {
        write_todo(args.name, args.todo.join(" ")).unwrap();

        println!("Created {} as a todo", args.todo.join(" "));

        return;
    }
}

fn get_dir() -> String {
    let mut dir = current_exe().unwrap();
    dir.pop();
    dir.push("todos");

    return dir.to_str().unwrap().to_string();
}

fn write_todo(name: String, data: String) -> Result<(), Box<dyn Error>> {
    let dir = get_dir();
    let dir_formatted = format!("{}//{}.txt", dir, name);
    let path = Path::new(&dir_formatted);

    println!("{:?}", dir);

    std::fs::create_dir_all(dir.clone())?;

    if !path.exists() {
        File::create(path)?;
    }

    File::open(path)?;

    write(Path::new(&format!("{}//{}.txt", dir, name)), data)?;

    Ok(())
}

fn read_todo(name: String) -> Result<String, Box<dyn Error>> {
    let dir = get_dir();
    let dir_formatted = format!("{}//{}.txt", dir, name);
    let path = Path::new(&dir_formatted);

    if !path.exists() {
        panic!("File does not exist.")
    }

    let contents = String::from_utf8(read(path)?)?;

    Ok(contents)
}

fn delete_todo(name: String) -> Result<(), Box<dyn Error>> {
    let dir = get_dir();
    let dir_formatted = format!("{}//{}.txt", dir, name);
    let path = Path::new(&dir_formatted);

    if !path.exists() {
        panic!("File does not exist.")
    }

    fs::remove_file(path)?;

    Ok(())
}
