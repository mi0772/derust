use std::{env, fs, io};
use std::io::ErrorKind;
use std::path::Path;
use crate::utils;

pub enum OperationType {
    DeleteTempFiles(DeleteTempFilesOp),
    DeleteAppCache(DeleteAppCacheOp),
    ClearBrowserCache(ClearBrowserCacheOp),

}

pub struct DirInfo {
    pub(crate) total_size: usize,
    pub(crate) file_count: usize,
}

impl OperationType {
    pub(crate) fn execute(&mut self) -> io::Result<DirInfo> {
        match self {
            OperationType::DeleteTempFiles(op) => op.execute(),
            OperationType::DeleteAppCache(op) => op.execute(),
            OperationType::ClearBrowserCache(op) => op.execute(),
        }
    }

    pub(crate) fn from_selection(selection: usize) -> Self {
        match selection {
            0 => OperationType::DeleteTempFiles(DeleteTempFilesOp),
            1 => OperationType::DeleteAppCache(DeleteAppCacheOp ),
            2 => OperationType::ClearBrowserCache(ClearBrowserCacheOp),
            _ => unreachable!(),
        }
    }
}

trait Operation {
    fn execute(&mut self) -> io::Result<DirInfo>;
}

pub(crate) struct DeleteTempFilesOp ;
pub(crate) struct DeleteAppCacheOp;
pub(crate) struct ClearBrowserCacheOp;

impl Operation for DeleteTempFilesOp {
    fn execute(&mut self) -> io::Result<DirInfo> {

        let mut dir_info = DirInfo {
            total_size: 0,
            file_count: 0,
        };

        let temp_dir = "/tmp";  // Directory da cui cancellare i file

        println!("Pulizia della directory: {}", temp_dir);

        if Path::new(temp_dir).exists() {
            for entry in fs::read_dir(temp_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    dir_info.file_count += 1;
                    dir_info.total_size += fs::metadata(&path)?.len() as usize;
                    if let Err(e) = fs::remove_file(&path) {
                        if e.kind() != ErrorKind::PermissionDenied {
                            return Err(e);
                        }
                    }
                }
            }
        }
        Ok(dir_info)

    }
}

impl Operation for DeleteAppCacheOp {
    fn execute(&mut self) -> io::Result<DirInfo> {
        let mut dir_info = DirInfo {
            total_size: 0,
            file_count: 0,
        };

        let cache_dir = if cfg!(target_os = "linux") {
            format!("{}/.cache", env::var("HOME").unwrap())
        } else if cfg!(target_os = "macos") {
            format!("{}/Library/Caches", env::var("HOME").unwrap())
        } else if cfg!(target_os = "windows") {
            format!("C:\\Users\\{}\\AppData\\Local", env::var("USERNAME").unwrap())
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Sistema operativo non supportato"));
        };

        println!("Pulizia della cache nella directory: {}", cache_dir);

        if Path::new(&cache_dir).exists() {
            for entry in fs::read_dir(cache_dir)? {
                let entry = entry?;
                let path = entry.path();

                // Se è una directory, proviamo a rimuoverla
                if path.is_dir() {
                    let di = utils::fs::get_dir_info(&path)?;
                    dir_info.total_size += di.total_size as usize;
                    dir_info.file_count += di.file_count as usize;

                    if let Err(e) = fs::remove_dir_all(&path) {
                        if e.kind() != ErrorKind::PermissionDenied {
                            return Err(e);
                        }
                    } else {
                        dir_info.file_count += 1;
                        dir_info.total_size += fs::metadata(&path)?.len() as usize;
                    }
                } else if path.is_file() {
                    if let Err(e) = fs::remove_file(&path) {
                        if e.kind() != ErrorKind::PermissionDenied {
                            return Err(e);
                        }
                    }
                }
            }
            println!("Pulizia della cache completata.");
        } else {
            println!("La directory della cache non esiste.");
        }
        Ok(dir_info)
    }
}

impl Operation for ClearBrowserCacheOp {
    fn execute(&mut self) -> io::Result<DirInfo> {
        let mut dir_info = DirInfo {
            total_size: 0,
            file_count: 0,
        };

        let paths = vec![
            if cfg!(target_os = "linux") {
                format!("{}/.cache/google-chrome", env::var("HOME").unwrap())
            } else if cfg!(target_os = "macos") {
                format!("{}/Library/Caches/Google/Chrome", env::var("HOME").unwrap())
            } else if cfg!(target_os = "windows") {
                format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", env::var("USERNAME").unwrap())
            } else {
                return Err(io::Error::new(io::ErrorKind::NotFound, "Sistema operativo non supportato"));
            },
            if cfg!(target_os = "linux") {
                format!("{}/.cache/mozilla/firefox", env::var("HOME").unwrap())
            } else if cfg!(target_os = "macos") {
                format!("{}/Library/Caches/Firefox/Profiles", env::var("HOME").unwrap())
            } else if cfg!(target_os = "windows") {
                format!("C:\\Users\\{}\\AppData\\Local\\Mozilla\\Firefox\\Profiles", env::var("USERNAME").unwrap())
            } else {
                String::new()
            },
            if cfg!(target_os = "macos") {
                format!("{}/Library/Caches/com.apple.Safari", env::var("HOME").unwrap())
            } else {
                String::new()
            }
        ];

        for path in paths {
            if !path.is_empty() && Path::new(&path).exists() {
                println!("Pulizia della cache per: {}", path);
                for entry in fs::read_dir(&path)? {
                    let entry = entry?;
                    let entry_path = entry.path();

                    if entry_path.is_dir() {
                        println!("Cancellazione directory: {:?}", entry_path);
                        let di = utils::fs::get_dir_info(&entry_path)?;
                        dir_info.total_size += dir_info.total_size as usize;
                        dir_info.file_count += dir_info.file_count as usize;

                        if let Err(e) = fs::remove_dir_all(&entry_path) {
                            if e.kind() == ErrorKind::PermissionDenied {
                                println!("Permission denied per: {:?}, continuando...", entry_path);
                            } else {
                                return Err(e);
                            }
                        }
                    } else if entry_path.is_file() {
                        println!("Cancellazione file: {:?}", entry_path);
                        if let Err(e) = fs::remove_file(&entry_path) {
                            if e.kind() == ErrorKind::PermissionDenied {
                                println!("Permission denied per: {:?}, continuando...", entry_path);
                            } else {
                                return Err(e);
                            }
                            dir_info.total_size += fs::metadata(&entry_path)?.len() as usize;
                            dir_info.file_count += 1;
                        }
                    }
                }
                println!("Pulizia della cache completata per: {}", path);
            } else {
                println!("La directory della cache non esiste per: {}", path);
            }
        }
        Ok(dir_info)
    }

}