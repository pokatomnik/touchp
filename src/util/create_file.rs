use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub enum FileCreateErrors {
    FailedToCreateDirectory(String),
    FailedToCreateFile(String),
    FailedToWrite(String),
}

impl FileCreateErrors {
    pub fn get_description(&self) -> String {
        match self {
            FileCreateErrors::FailedToCreateDirectory(directory) => format!("Failed to create directory: \"{}\"", directory),
            FileCreateErrors::FailedToCreateFile(file) => format!("Failed to create file: \"{}\"", file),
            FileCreateErrors::FailedToWrite(file) => format!("Failed to write file: \"{}\"", file),
        }
    }
}

pub fn create_file(absolute_target_path: PathBuf, contents: Option<String>) -> Result<(), FileCreateErrors> {
    let absolute_target_directory = {
        let mut absolute_target_directory = absolute_target_path.clone();
        absolute_target_directory.pop();
        absolute_target_directory
    };

    let is_directory = fs::metadata(&absolute_target_directory)
        .map(|m| m.is_dir())
        .unwrap_or(false);

    if is_directory {
        let mut file = fs::File::create(&absolute_target_path)
            .map_err(|_| {
                FileCreateErrors::FailedToCreateFile(absolute_target_path.to_string_lossy().to_string())
            })?;
        if contents.is_some() {
            file
                .write(contents.unwrap().as_bytes())
                .map_err(|_| {
                    FileCreateErrors::FailedToWrite(absolute_target_path.to_string_lossy().to_string())
                })?;
            return file
                .flush()
                .map(|_| ())
                .map_err(|_| {
                    FileCreateErrors::FailedToWrite(absolute_target_path.to_string_lossy().to_string())
                });
        }
        return Ok(());
    }

    fs::create_dir_all(&absolute_target_directory).map_err(|_| {
        FileCreateErrors::FailedToCreateDirectory(absolute_target_directory.to_string_lossy().to_string())
    })?;

    let mut file = fs::File::create(&absolute_target_path)
        .map_err(|_| {
            FileCreateErrors::FailedToCreateFile(absolute_target_path.to_string_lossy().to_string())
        })?;

    if contents.is_some() {
        file
            .write(contents.unwrap().as_bytes())
            .map_err(|_| {
                FileCreateErrors::FailedToWrite(absolute_target_path.to_string_lossy().to_string())
            })?;
        return file
            .flush()
            .map(|_| ())
            .map_err(|_| {
                FileCreateErrors::FailedToWrite(absolute_target_path.to_string_lossy().to_string())
            });
    }

    Ok(())
}