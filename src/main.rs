use std::path::{Path, PathBuf};
use std::fs;
use clap::Parser;
use owo_colors::OwoColorize;
use owo_colors::colors::*;
use strum::Display;
use tabled::settings::{Color, Style};
use tabled::settings::object::{Columns, Rows};
use tabled::{Table, Tabled};

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    name: String,
    e_type: EntryType,
    len_bytes: u64,
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best colored ls command tool")]
struct Cli {
    path: Option<PathBuf>
}

fn main() {
    let cli = Cli::parse();


    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let get_files  = get_files(&path);
            let mut table = Table::new(&get_files);
            table.with(Style::rounded());
            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);

            for (i, entry) in get_files.iter().enumerate() {
                match entry.e_type {
                    EntryType::Dir => {
                        table.modify(Rows::new(i + 1..=i + 1), Color::FG_BRIGHT_BLUE);
                    }
                    EntryType::File => {
                        table.modify(Rows::new(i + 1..=i + 1), Color::FG_GREEN);
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
    let mut data  = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }   
    }
    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry { 
            name: file.file_name().into_string().unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            }, 
            len_bytes: meta.len(), 
            modified: "".to_string(),
        });
    } 
}