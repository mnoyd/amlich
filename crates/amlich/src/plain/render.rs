pub const BOLD: &str = "\x1b[1m";
pub const CYAN: &str = "\x1b[36m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const DIM: &str = "\x1b[2m";
pub const RESET: &str = "\x1b[0m";

#[derive(Default)]
pub struct Frame {
    lines: Vec<String>,
}
impl Frame {
    pub fn line(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }
    pub fn blank(&mut self) {
        self.lines.push(String::new());
    }
    pub fn section(&mut self, title: &str) {
        self.line(format!("{CYAN}{BOLD}── {title} {RESET}"));
    }
    pub fn finish(self, scroll: usize) -> String {
        self.lines
            .into_iter()
            .skip(scroll)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frame_skips_scroll() {
        let mut f = Frame::default();
        f.line("a");
        f.line("b");
        assert_eq!(f.finish(1), "b\n");
    }
}
