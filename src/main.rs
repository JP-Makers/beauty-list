use chrono::{DateTime, Local};
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;
use std::path::{Path, PathBuf};
use strum::Display;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Color, Style};
use tabled::{Table, Tabled};
#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size")]
    len_bytes: String,
    #[tabled(rename = "Modified")]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best colored ls command tool")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let get_files = get_files(&path);
            let mut table = Table::new(&get_files);
            table.with(Style::rounded());

            table.modify(Rows::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::new(1..=1), Color::FG_MAGENTA);
            table.modify(Columns::new(2..=2), Color::FG_YELLOW);
            table.modify(Columns::new(3..=3), Color::FG_CYAN);

            for (i, entry) in get_files.iter().enumerate() {
                match entry.e_type {
                    EntryType::Dir => {
                        table.modify((i + 1, 0), Color::FG_BRIGHT_BLUE);
                    }
                    EntryType::File => {
                        table.modify((i + 1, 0), Color::FG_GREEN);
                    }
                }
            }
            println!("{}", table);
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "error reading directory".red());
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }

    data.sort_by(|a, b| {
        let a_is_dir = matches!(a.e_type, EntryType::Dir);
        let b_is_dir = matches!(b.e_type, EntryType::Dir);
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        let size_str = if meta.is_dir() {
            "".to_string()
        } else {
            format_bytes(meta.len())
        };

        let mod_time = match meta.modified() {
            Ok(time) => {
                let datetime: DateTime<Local> = time.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            Err(_) => "".to_string(),
        };

        let file_name = file
            .file_name()
            .into_string()
            .unwrap_or("unknown name".into());

        data.push(FileEntry {
            name: file_name,
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: size_str,
            modified: mod_time,
        });
    }
}

fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut v = bytes as f64;
    let mut unit_idx = 0;
    while v >= 1024.0 && unit_idx < units.len() - 1 {
        v /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.1} {}", v, units[unit_idx])
}
