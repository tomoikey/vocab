use std::process::{Child, Command};

pub struct Speaker {
    process: Option<Child>,
}

impl Speaker {
    pub fn new() -> Self {
        Self { process: None }
    }

    #[cfg(target_os = "macos")]
    pub fn speak(&mut self, text: &str) {
        if let Some(mut child) = self.process.take() {
            child.kill().expect("failed to kill say");
        }
        let child = Command::new("say")
            .args(["-v", "Samantha", text])
            .spawn()
            .expect("failed to execute say");
        self.process = Some(child);
    }
}
