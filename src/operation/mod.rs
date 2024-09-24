use std::{env, fs, io};
use std::io::ErrorKind;
use std::path::Path;

pub enum OperationType {
    DeleteTempFiles(DeleteTempFilesOp),
    DeleteAppCache(DeleteAppCacheOp),
    ClearBrowserCache(ClearBrowserCacheOp),

}

impl OperationType {
    pub(crate) fn execute(&mut self) -> io::Result<()> {
        match self {
            OperationType::DeleteTempFiles(op) => op.execute(),
            OperationType::DeleteAppCache(op) => op.execute(),
            OperationType::ClearBrowserCache(op) => op.execute(),
        }
    }

    pub(crate) fn from_selection(selection: usize) -> Self {
        match selection {
            0 => OperationType::DeleteTempFiles(DeleteTempFilesOp {
                files_count: 0,
                bytes_count: 0,
            }),
            1 => OperationType::DeleteAppCache(DeleteAppCacheOp {
                files_count: 0,
                bytes_count: 0,
            }),
            2 => OperationType::ClearBrowserCache(ClearBrowserCacheOp {
                files_count: 0,
                bytes_count: 0,
            }),
            _ => unreachable!(),
        }
    }
}

trait Operation {
    fn execute(&mut self) -> io::Result<()>;
    fn display_files_count(&self);
    fn display_bytes_count(&self);
}

pub(crate) struct DeleteTempFilesOp {
    files_count: usize,
    bytes_count: usize,
}
pub(crate) struct DeleteAppCacheOp {
    files_count: usize,
    bytes_count: usize,
}
pub(crate) struct ClearBrowserCacheOp {
    files_count: usize,
    bytes_count: usize,
}
impl Operation for DeleteTempFilesOp {
    fn execute(&mut self) -> io::Result<()> {

        let temp_dir = "/tmp";  // Directory da cui cancellare i file

        println!("Pulizia della directory: {}", temp_dir);

        if Path::new(temp_dir).exists() {
            for entry in fs::read_dir(temp_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    self.files_count += 1;
                    self.bytes_count += fs::metadata(&path)?.len() as usize;
                    if let Err(e) = fs::remove_file(&path) {
                        if e.kind() != ErrorKind::PermissionDenied {
                            return Err(e);
                        }
                    }
                }
            }
            println!("Done, {} files cancellati, {} Mb liberati.", self.files_count, self.bytes_count/1024/1024);
        } else {
            println!("La directory temporanea non esiste.");
        }
        Ok(())

    }
    fn display_files_count(&self) {
        println!("File cancellati: {}", self.files_count);
    }
    fn display_bytes_count(&self) {
        println!("MBytes liberati: {}", self.bytes_count/1024/1024);
    }
}

impl Operation for DeleteAppCacheOp {
    fn execute(&mut self) -> io::Result<()> {
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
                    if let Err(e) = fs::remove_dir_all(&path) {
                        if e.kind() != ErrorKind::PermissionDenied {
                            return Err(e);
                        }
                    } else {
                        self.files_count += 1;
                        self.bytes_count += fs::metadata(&path)?.len() as usize;
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
        Ok(())
    }
    fn display_files_count(&self) {
        println!("File cancellati: {}", self.files_count);
    }
    fn display_bytes_count(&self) {
        println!("MBytes liberati: {}", self.bytes_count/1024/1024);
    }
}

impl Operation for ClearBrowserCacheOp {
    fn execute(&mut self) -> io::Result<()> {
        clear_browser_cache()
    }
    fn display_files_count(&self) {
        println!("File cancellati: {}", self.files_count);
    }
    fn display_bytes_count(&self) {
        println!("MBytes liberati: {}", self.bytes_count/1024/1024);
    }
}


// Funzione per cancellare la cache dei browser
fn clear_browser_cache() -> io::Result<()> {
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
                    }
                }
            }
            println!("Pulizia della cache completata per: {}", path);
        } else {
            println!("La directory della cache non esiste per: {}", path);
        }
    }
    Ok(())
}