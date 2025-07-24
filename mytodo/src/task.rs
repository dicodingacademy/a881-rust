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
	pub fn add(&mut self, desc: String) -> Result<String> {
        self.tasks.push(Task { description: desc, done: false });
        Ok("Tugas baru ditambahkan.".to_string())
    }

    pub fn done(&mut self, id: usize) -> Result<String> {
        if id == 0 || id > self.tasks.len() {
            bail!("Nomor tugas {} tidak ditemukan", id);
        }
        self.tasks[id - 1].done = true;
        Ok(format!("Tugas {} ditandai selesai.", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<String> {
        if id == 0 || id > self.tasks.len() {
            bail!("Nomor tugas {} tidak ditemukan", id);
        }
        self.tasks.remove(id - 1);
        Ok(format!("Tugas {} dihapus.", id))
    }

	pub fn print(&self) -> String {
        if self.tasks.is_empty() {
            return "(Belum ada tugas)".to_string();
        }

        let mut output = String::from("Daftar Tugas:");
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[x]" } else { "[ ]" };
            output.push_str(&format!("\n{}. {} {}", i + 1, status, task.description));
        }
		output
    }

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
}
