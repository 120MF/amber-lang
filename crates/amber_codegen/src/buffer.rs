/// Efficient line-based code buffer for C code generation
#[derive(Default)]
pub struct CodeBuffer {
    lines: Vec<String>,
}

impl CodeBuffer {
    pub fn push_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn push_indented_line(&mut self, indent: usize, line: &str) {
        let indentation = "    ".repeat(indent);
        self.lines.push(format!("{}{}", indentation, line));
    }

    pub fn finish(self) -> String {
        let mut content = self.lines.join("\n");
        // Add final newline if not already present
        if !content.is_empty() {
            content.push('\n');
        }
        format!(
            "#include <stdint.h>\n#include <stdbool.h>\n\n{}",
            content
        )
    }
}
