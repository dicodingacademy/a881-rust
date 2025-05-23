use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    const FILE_PATH: &'static str = "tasks.json";

    pub fn load() -> Result<Self> {
        if !Path::new(Self::FILE_PATH).exists() {
            return Ok(Self::default());

        }

        let data = fs::read_to_string(Self::FILE_PATH)?;
        let list = serde_json::from_str(&data)?;
        Ok(list)
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }

    pub fn add(&mut self, desc: String) {
        self.tasks.push(Task { description: desc, done: false });
        println!("Tugas baru ditambahkan.");
    }

    pub fn print(&self) {
        if self.tasks.is_empty() {
            println!("(Belum ada tugas)");
            return;
        }

        println!("Daftar Tugas:");
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[x]" } else { "[ ]" };
            println!("{}. {} {}", i + 1, status, task.description);
        }
    }

    pub fn done(&mut self, id: usize) -> Result<()> {
        if id == 0 || id > self.tasks.len() {
            bail!("Nomor tugas {} tidak ditemukan", id);
        }
        self.tasks[id - 1].done = true;
        println!("Tugas {} ditandai selesai.", id);
        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> Result<()> {
        if id == 0 || id > self.tasks.len() {
            bail!("Nomor tugas {} tidak ditemukan", id);
        }
        self.tasks.remove(id - 1);
        println!("Tugas {} dihapus.", id);
        Ok(())
    }
}
