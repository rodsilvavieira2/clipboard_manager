use std::process::Command;

use crate::service::cliboard_history::ClipboardContent;
use crate::service::cliboard_provider::IClipboardProvider;

const COMMAND: &str = "cliphist";

#[derive(Clone, Copy)]
pub struct CliphistProvider;

impl IClipboardProvider for CliphistProvider {
    fn name(&self) -> &'static str {
        COMMAND
    }

    fn list_entries(&self) -> Result<Vec<(Option<String>, ClipboardContent)>, String> {
        let output = Command::new(COMMAND)
            .arg("list")
            .output()
            .map_err(|err| format!("{COMMAND} list failed: {err}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("{COMMAND} list non-zero exit: {stderr}"));
        }

        let mut entries = Vec::new();

        for line in output.stdout.split(|byte| *byte == b'\n') {
            if line.is_empty() {
                continue;
            }

            let line = if let Some(stripped) = line.strip_suffix(b"\r") {
                stripped
            } else {
                line
            };

            let (id_bytes, content_bytes) = match line.iter().position(|byte| *byte == b'\t') {
                Some(index) => (Some(&line[..index]), &line[index + 1..]),
                None => (None, line),
            };

            let content = clean_bytes_to_string(content_bytes);
            if content.is_empty() {
                continue;
            }

            let raw_id = id_bytes
                .map(clean_bytes_to_string)
                .filter(|value| !value.is_empty());

            let content_type = if content.starts_with("[[ binary data") {
                ClipboardContent::Image(())
            } else {
                ClipboardContent::Text(content)
            };

            entries.push((raw_id, content_type));
        }

        Ok(entries)
    }
}

fn clean_bytes_to_string(bytes: &[u8]) -> String {
    let filtered: Vec<u8> = bytes.iter().copied().filter(|byte| *byte != 0).collect();
    String::from_utf8_lossy(&filtered).trim().to_string()
}
