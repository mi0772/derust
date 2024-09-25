use std::{fs, io};
use std::path::Path;
use crate::operation::DirInfo;

pub(crate) fn get_dir_info(path: &Path) -> io::Result<DirInfo> {
    let mut dir_info = DirInfo {
        total_size: 0,
        file_count: 0,
    };

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            let subdir_info = get_dir_info(&entry.path())?;
            dir_info.total_size += subdir_info.total_size;
            dir_info.file_count += subdir_info.file_count;
        } else {
            dir_info.total_size += metadata.len() as usize;
            dir_info.file_count += 1;
        }
    }
    Ok(dir_info)
}