use tokio::io::{stdout, AsyncWriteExt};

#[derive(Debug, Default)]
pub struct TestSuite {
    pub successes: u32,
    pub failures: u32,
}
impl TestSuite {
    pub(super) async fn test(&self, name: &str) {
        print!("  {name}: ");
        let _ = stdout().flush().await;
    }
    pub(super) fn success(&mut self) {
        println!("✅");
        self.successes += 1;
    }
    pub(super) fn fail(&mut self, msg: &str) {
        println!("❌");
        if !msg.is_empty() {
            eprintln!("  {msg}");
        }
        self.failures += 1;
    }
    pub fn finish(&self) {
        if self.failures > 0 {
            eprintln!("\nSuccess: {}, Failures: {}", self.successes, self.failures);
            std::process::exit(-1);
        } else {
            println!("\nSuccess: {}, Failures: {}", self.successes, self.failures);
        }
    }
}