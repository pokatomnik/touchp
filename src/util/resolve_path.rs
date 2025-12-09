use std::env;
use std::path::PathBuf;
use path_absolutize::Absolutize;

pub enum ResolvePathError {
    FailedToAbsolutize(String),
    FailedToResolveCurrentDirectory,
}

impl ResolvePathError {
    pub fn get_description(&self) -> String {
        match self {
            ResolvePathError::FailedToAbsolutize(description) => format!("Failed to get absolute path for \"{}\"", description),
            ResolvePathError::FailedToResolveCurrentDirectory => "Failed to resolve current directory".to_string(),
        }
    }
}

pub fn resolve_path(target_arg: &str) -> Result<PathBuf, ResolvePathError> {
    let result = PathBuf::from(target_arg);
    if result.is_absolute() {
        return match result.absolutize() {
            Ok(path) => Ok(PathBuf::from(path)),
            Err(_) => Err(ResolvePathError::FailedToAbsolutize(target_arg.to_string())),
        }
    }
    let cwd = match env::current_dir() {
        Ok(cwd) => Ok(cwd),
        Err(_) => Err(ResolvePathError::FailedToResolveCurrentDirectory)
    }?;

    let absolute_raw = cwd.join(result);
    match absolute_raw.absolutize() {
        Ok(path) => Ok(path.to_path_buf()),
        Err(_) => Err(ResolvePathError::FailedToAbsolutize(absolute_raw.to_string_lossy().to_string())),
    }
}