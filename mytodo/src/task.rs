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
            bail!("Nomor tugas {} tidak ditemukan.", id);
        }
        self.tasks[id - 1].done = true;
        Ok(format!("Tugas {} ditandai selesai.", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<String> {
        if id == 0 || id > self.tasks.len() {
            bail!("Nomor tugas {} tidak ditemukan.", id);
        }
        self.tasks.remove(id - 1);
        Ok(format!("Tugas {} dihapus.", id))
    }

	pub fn print(&self) -> Result<String> {
        if self.tasks.is_empty() {
            return Ok("(Belum ada tugas)".to_string());
        }

        let mut output = String::from("Daftar Tugas:");
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[x]" } else { "[ ]" };
            output.push_str(&format!("\n{}. {} {}", i + 1, status, task.description));
        }
		Ok(output)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut list = TaskList::default();

        let _ = list.add("Belajar Rust".into());

        assert_eq!(list.tasks.iter().len(), 1);
        assert_eq!(list.tasks[0].description, "Belajar Rust");
        assert_eq!(list.tasks[0].done, false);
    }

    #[test]
    fn test_done_task_success() {
        let mut list = TaskList::default();
        let _ = list.add("Belajar Rust".into());

        let msg = list.done(1).unwrap();

        assert_eq!(msg, "Tugas 1 ditandai selesai.");
        assert_eq!(list.tasks[0].done, true);
    }

    #[test]
    fn test_done_task_failed() {
        let mut list = TaskList::default();
        let _ = list.add("Belajar Rust".into());

        let err_msg = list.done(10).unwrap_err();

        assert_eq!(err_msg.to_string(), "Nomor tugas 10 tidak ditemukan.");
        assert_eq!(list.tasks[0].done, false);
    }

    #[test]
    fn test_remove_task_success() {
        let mut list = TaskList::default();
        let _ = list.add("Belajar Rust".into());

        let msg = list.remove(1).unwrap();

        assert_eq!(msg, "Tugas 1 dihapus.");
        assert_eq!(list.tasks.is_empty(), true);
    }

    #[test]
    fn test_remove_task_failed() {
        let mut list = TaskList::default();
        let _ = list.add("Belajar Rust".into());

        let err_msg = list.remove(10).unwrap_err();

        assert_eq!(err_msg.to_string(), "Nomor tugas 10 tidak ditemukan.");
        assert_eq!(list.tasks.is_empty(), false);
    }

    #[test]
    fn test_print_empty() {
        let list = TaskList::default();

        let output = list.print().unwrap();
        
        assert_eq!(output, "(Belum ada tugas)");
    }

    #[test]
    fn test_print_tasks() {
        let mut list = TaskList::default();
        let _ = list.add("Belajar Rust".into());
        let _ = list.add("Belajar Unit Test".into());

        let output = list.print().unwrap();

        assert_eq!(output, "Daftar Tugas:\n1. [ ] Belajar Rust\n2. [ ] Belajar Unit Test");
    }
}