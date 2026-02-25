use chrono::{DateTime, Local};
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
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
    #[tabled(rename = "No")]
    no: usize,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size")]
    len_bytes: String,
    #[tabled(rename = "Mode")]
    mode: String,
    #[tabled(rename = "Octal")]
    octal: String,
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

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let get_files = get_files(&path);
            let mut table = Table::new(&get_files);
            table.with(Style::rounded());

            table.modify(Rows::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::new(2..=2), Color::FG_MAGENTA);
            table.modify(Columns::new(3..=3), Color::FG_YELLOW);
            table.modify(Columns::new(4..=4), Color::FG_WHITE);
            table.modify(Columns::new(5..=5), Color::FG_CYAN);
            table.modify(Columns::new(6..=6), Color::FG_CYAN);

            for (i, entry) in get_files.iter().enumerate() {
                match entry.e_type {
                    EntryType::Dir => {
                        table.modify((i + 1, 1), Color::FG_BRIGHT_BLUE);
                    }
                    EntryType::File => {
                        table.modify((i + 1, 1), Color::FG_GREEN);
                    }
                }
            }
            println!("{}", table);
        } else {
            eprintln!("{}: {}", "Error".red().bold(), "Path does not exist".red());
            std::process::exit(1);
        }
    } else {
        eprintln!("{}: {}", "Error".red().bold(), "searching directory".red());
        std::process::exit(1);
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if path.is_file() {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown name")
            .to_string();
        map_data(path, file_name, &mut data);
    } else if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                let file_name = file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into());
                map_data(&file.path(), file_name, &mut data);
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

    for (i, entry) in data.iter_mut().enumerate() {
        entry.no = i + 1;
    }

    data
}

fn map_data(path: &Path, file_name: String, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(path) {
        let size_str = format_bytes(meta.len());

        let mod_time = match meta.modified() {
            Ok(time) => {
                let datetime: DateTime<Local> = time.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            Err(_) => "".to_string(),
        };

        #[cfg(unix)]
        let (mode_str, octal_str) = {
            let mode = meta.permissions().mode();
            let is_dir = meta.is_dir();
            let is_symlink = meta.file_type().is_symlink();
            let octal = mode & 0o777;
            (
                format_mode_str(mode, is_dir, is_symlink),
                format!("{:03o}", octal),
            )
        };
        #[cfg(not(unix))]
        let (mode_str, octal_str) = ("unknown".to_string(), "000".to_string());

        data.push(FileEntry {
            no: 0,
            name: file_name,
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: size_str,
            mode: mode_str,
            octal: octal_str,
            modified: mod_time,
        });
    }
}

fn format_mode_str(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let mut s = String::with_capacity(10);
    s.push(if is_dir {
        'd'
    } else if is_symlink {
        'l'
    } else {
        '-'
    });
    s.push(if mode & 0o0400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o0200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o0100 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o0040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o0020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o0010 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o0004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o0002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o0001 != 0 { 'x' } else { '-' });
    s
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
