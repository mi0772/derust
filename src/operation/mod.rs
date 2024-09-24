use std::{env, fs, io};
use std::io::ErrorKind;
use std::path::Path;

pub enum OperationType {
    DeleteTempFiles(DeleteTempFilesOp),
    DeleteAppCache(DeleteAppCacheOp),
    ClearBrowserCache(ClearBrowserCacheOp),

}

impl OperationType {
    pub(crate) fn execute(&self) -> io::Result<()> {
        match self {
            OperationType::DeleteTempFiles(op) => op.execute(),
            OperationType::DeleteAppCache(op) => op.execute(),
            OperationType::ClearBrowserCache(op) => op.execute(),
        }
    }

    pub(crate) fn from_selection(selection: usize) -> Self {
        match selection {
            0 => OperationType::DeleteTempFiles(DeleteTempFilesOp),
            1 => OperationType::DeleteAppCache(DeleteAppCacheOp),
            2 => OperationType::ClearBrowserCache(ClearBrowserCacheOp),
            _ => unreachable!(),
        }
    }
}

trait Operation {
    fn execute(&self) -> io::Result<()>;
}

pub(crate) struct DeleteTempFilesOp;
pub(crate) struct DeleteAppCacheOp;
pub(crate) struct ClearBrowserCacheOp;
impl Operation for DeleteTempFilesOp {
    fn execute(&self) -> io::Result<()> {
        delete_temp_files()
    }
}

impl Operation for DeleteAppCacheOp {
    fn execute(&self) -> io::Result<()> {
        delete_app_cache()
    }
}

impl Operation for ClearBrowserCacheOp {
    fn execute(&self) -> io::Result<()> {
        clear_browser_cache()
    }
}

fn delete_temp_files() -> io::Result<()> {
    let temp_dir = "/tmp";  // Directory da cui cancellare i file

    println!("Pulizia della directory: {}", temp_dir);

    if Path::new(temp_dir).exists() {
        for entry in fs::read_dir(temp_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Cancelliamo solo i file, non le directory
                println!("Cancellazione file: {:?}", path);
                fs::remove_file(path)?;
            }
        }
        println!("Cancellazione completata.");
    } else {
        println!("La directory temporanea non esiste.");
    }
    Ok(())
}

fn delete_app_cache() -> io::Result<()> {
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
                println!("Cancellazione directory di cache: {:?}", path);
                if let Err(e) = fs::remove_dir_all(&path) {
                    if e.kind() == ErrorKind::PermissionDenied {
                        println!("Permission denied per: {:?}, continuando...", path);
                    } else {
                        return Err(e);
                    }
                }
            } else if path.is_file() {
                // Se è un file, proviamo a rimuoverlo
                println!("Cancellazione file di cache: {:?}", path);
                if let Err(e) = fs::remove_file(&path) {
                    if e.kind() == ErrorKind::PermissionDenied {
                        println!("Permission denied per: {:?}, continuando...", path);
                    } else {
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