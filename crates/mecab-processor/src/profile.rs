use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct SizeReport {
    pub char_def: usize,
    pub unk_dict: usize,
    pub cost_matrix: usize,
    pub entry_dict: usize,
    pub term_fst: usize,
}

impl SizeReport {
    pub fn total(&self) -> usize {
        self.char_def + self.unk_dict + self.cost_matrix + self.entry_dict + self.term_fst
    }
}

pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub trait AsFileSize {
    fn as_file_size(&self) -> String;
}

impl AsFileSize for usize {
    fn as_file_size(&self) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;

        if *self < KB as usize {
            format!("{} B", *self)
        } else if *self < MB as usize {
            format!("{:.2} KB", *self as f64 / KB)
        } else {
            format!("{:.2} MB", *self as f64 / MB)
        }
    }
}
