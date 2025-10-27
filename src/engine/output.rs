use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Text,
    Json,
    Xml,
    Csv,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Text
    }
}

pub struct OutputCapture {
    enabled: bool,
    stdout_buffer: Arc<Mutex<String>>,
    stderr_buffer: Arc<Mutex<String>>,
    original_stdout: Option<Box<dyn Write + Send>>,
    original_stderr: Option<Box<dyn Write + Send>>,
}

impl OutputCapture {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            stdout_buffer: Arc::new(Mutex::new(String::new())),
            stderr_buffer: Arc::new(Mutex::new(String::new())),
            original_stdout: None,
            original_stderr: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn capture_stdout(&self, output: &str) {
        if self.enabled {
            if let Ok(mut buffer) = self.stdout_buffer.lock() {
                buffer.push_str(output);
            }
        }
    }

    pub fn capture_stderr(&self, output: &str) {
        if self.enabled {
            if let Ok(mut buffer) = self.stderr_buffer.lock() {
                buffer.push_str(output);
            }
        }
    }

    pub fn get_stdout(&self) -> String {
        self.stdout_buffer
            .lock()
            .map(|buffer| buffer.clone())
            .unwrap_or_default()
    }

    pub fn get_stderr(&self) -> String {
        self.stderr_buffer
            .lock()
            .map(|buffer| buffer.clone())
            .unwrap_or_default()
    }

    pub fn get_combined_output(&self) -> String {
        let stdout = self.get_stdout();
        let stderr = self.get_stderr();

        if stdout.is_empty() {
            stderr
        } else if stderr.is_empty() {
            stdout
        } else {
            format!("{}{}", stdout, stderr)
        }
    }

    pub fn clear(&self) {
        if let Ok(mut buffer) = self.stdout_buffer.lock() {
            buffer.clear();
        }
        if let Ok(mut buffer) = self.stderr_buffer.lock() {
            buffer.clear();
        }
    }

    pub fn get_stdout_lines(&self) -> Vec<String> {
        self.get_stdout()
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    pub fn get_stderr_lines(&self) -> Vec<String> {
        self.get_stderr()
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    pub fn format_output(&self, format: OutputFormat) -> String {
        let stdout = self.get_stdout();
        let stderr = self.get_stderr();

        match format {
            OutputFormat::Text => {
                if stdout.is_empty() {
                    stderr
                } else if stderr.is_empty() {
                    stdout
                } else {
                    format!("{}\n{}", stdout, stderr)
                }
            }
            OutputFormat::Json => {
                format!(
                    r#"{{
  "stdout": {},
  "stderr": {},
  "combined": {}
}}"#,
                    serde_json::to_string(&stdout).unwrap_or("\"\"".to_string()),
                    serde_json::to_string(&stderr).unwrap_or("\"\"".to_string()),
                    serde_json::to_string(&self.get_combined_output())
                        .unwrap_or("\"\"".to_string())
                )
            }
            OutputFormat::Xml => {
                format!(
                    r#"<output>
  <stdout>{}</stdout>
  <stderr>{}</stderr>
  <combined>{}</combined>
</output>"#,
                    escape_xml(&stdout),
                    escape_xml(&stderr),
                    escape_xml(&self.get_combined_output())
                )
            }
            OutputFormat::Csv => {
                format!(
                    "type,content\nstdout,{}\nstderr,{}",
                    escape_csv(&stdout),
                    escape_csv(&stderr)
                )
            }
        }
    }

    pub fn get_stats(&self) -> OutputStats {
        let stdout = self.get_stdout();
        let stderr = self.get_stderr();

        OutputStats {
            stdout_bytes: stdout.len(),
            stderr_bytes: stderr.len(),
            stdout_lines: stdout.lines().count(),
            stderr_lines: stderr.lines().count(),
            total_bytes: stdout.len() + stderr.len(),
            total_lines: stdout.lines().count() + stderr.lines().count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputStats {
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub stdout_lines: usize,
    pub stderr_lines: usize,
    pub total_bytes: usize,
    pub total_lines: usize,
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn escape_csv(input: &str) -> String {
    if input.contains(',') || input.contains('"') || input.contains('\n') {
        format!("\"{}\"", input.replace('"', "\"\""))
    } else {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_capture_basic() {
        let capture = OutputCapture::new(true);

        capture.capture_stdout("Hello, World!\n");
        capture.capture_stderr("Error message\n");

        assert_eq!(capture.get_stdout(), "Hello, World!\n");
        assert_eq!(capture.get_stderr(), "Error message\n");
        assert_eq!(
            capture.get_combined_output(),
            "Hello, World!\nError message\n"
        );
    }

    #[test]
    fn test_output_capture_disabled() {
        let capture = OutputCapture::new(false);

        capture.capture_stdout("This should not be captured\n");
        capture.capture_stderr("Neither should this\n");

        assert_eq!(capture.get_stdout(), "");
        assert_eq!(capture.get_stderr(), "");
    }

    #[test]
    fn test_output_format_text() {
        let capture = OutputCapture::new(true);
        capture.capture_stdout("Line 1\nLine 2\n");
        capture.capture_stderr("Error line\n");

        let formatted = capture.format_output(OutputFormat::Text);
        assert!(formatted.contains("Line 1"));
        assert!(formatted.contains("Error line"));
    }

    #[test]
    fn test_output_stats() {
        let capture = OutputCapture::new(true);
        capture.capture_stdout("Line 1\nLine 2\nLine 3\n");
        capture.capture_stderr("Error 1\nError 2\n");

        let stats = capture.get_stats();
        assert_eq!(stats.stdout_lines, 3);
        assert_eq!(stats.stderr_lines, 2);
        assert_eq!(stats.total_lines, 5);
        assert!(stats.stdout_bytes > 0);
        assert!(stats.stderr_bytes > 0);
    }

    #[test]
    fn test_clear_output() {
        let capture = OutputCapture::new(true);
        capture.capture_stdout("Some output\n");
        capture.capture_stderr("Some error\n");

        assert!(!capture.get_stdout().is_empty());
        assert!(!capture.get_stderr().is_empty());

        capture.clear();

        assert!(capture.get_stdout().is_empty());
        assert!(capture.get_stderr().is_empty());
    }

    #[test]
    fn test_get_lines() {
        let capture = OutputCapture::new(true);
        capture.capture_stdout("Line 1\nLine 2\nLine 3\n");

        let lines = capture.get_stdout_lines();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
    }
}
