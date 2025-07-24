mod task;
use task::TaskList;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mytodo", version = "1.0", about = "Aplikasi to-do list CLI sederhana")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Menambah tugas baru dengan deskripsi tertentu
    Add { description: String },
    /// Menampilkan semua tugas
    List,
    /// Menandai tugas dengan nomor tertentu sebagai selesai
    Done { id: usize },
    /// Menghapus tugas dengan nomor tertentu
    Remove { id: usize },
}

pub fn run(cli: Cli) -> Result<()> {
    let mut tasks = TaskList::load().unwrap_or_default();
    let mut updated = false;

    match cli.command {
        Commands::Add { description } => {
            println!("{}", tasks.add(description)?);
            updated = true;
        }
        Commands::List => {
            println!("{}", tasks.print()?);
        }
        Commands::Done { id } => {
            println!("{}", tasks.done(id)?);
            updated = true;
        }
        Commands::Remove { id } => {
            println!("{}", tasks.remove(id)?);
            updated = true;
        }
    }

    if updated {
        tasks.save().context("Gagal menyimpan data tugas")?;
    }

    Ok(())
}
