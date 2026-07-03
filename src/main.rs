use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

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
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { url, code } => {
            set_in(url, code).unwrap();
        },
        Commands::Get { code } => {
            match get_in(code) {
                Ok(url) => println!("Fast link ->\n{url}"),
                Err(_) => println!("Code not found"),
            }
        }
    }
}


fn set_in(link: String, code: String) -> Result<()> {
    let connection = Connection::open("./shortens.db")?;
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

fn get_in(code: String) -> Result<String> {
    let connection = Connection::open("./shortens.db")?;
    let mut stmt = connection.prepare(
        "SELECT link FROM commands WHERE code = ?1"
    )?;

    let url: String = stmt.query_row([code], |row| row.get(0))?;
    Ok(url)

}