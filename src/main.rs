use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::{fs, path::{PathBuf}};
use home::home_dir;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Add {
        url: String,
        code: String
    },
    Get { code: String }
}

fn main() {
    let db_path = match shortens_path() {
        Ok(path) => path,
        Err(e) => {
            eprint!("Error: {e}");
            std::process::exit(1);
        }

    };
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { url, code } => {
            set_in(url, code, &db_path).unwrap();
        },
        Commands::Get { code } => {
            match get_in(code, &db_path) {
                Ok(url) => println!("Fast link ->\n{url}"),
                Err(_) => println!("Code not found"),
            }
        }
    }
}

fn shortens_path() -> Result<PathBuf, String> {
    let mut db_path = home_dir().ok_or("Home directory not found...")?;
    db_path.push("MyShorten");
    fs::create_dir_all(&db_path).map_err(|e| format!("Failed to create the directory: {e}"))?;
    db_path.push("shortens.db");

    Ok(db_path)
}


fn set_in(link: String, code: String, db_path: &PathBuf) -> Result<()> {

    let connection = Connection::open(db_path)?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            code TEXT,
            link TEXT
        );",
        (),
    )?;

    connection.execute("INSERT INTO commands (code, link) VALUES (?1, ?2)", (&code, &link))?;
    Ok(())
}

fn get_in(code: String, db_path: &PathBuf) -> Result<String> {
    let connection = Connection::open(db_path)?;
    let mut stmt = connection.prepare(
        "SELECT link FROM commands WHERE code = ?1"
    )?;

    let url: String = stmt.query_row([code], |row| row.get(0))?;
    Ok(url)

}