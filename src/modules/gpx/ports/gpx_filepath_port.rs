use core::fmt;
use std::path::Path;

use crate::modules::gpx::domain::gpx_track::GpxTrack;

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::Io(e) => write!(f, "IO error: {}", e),
            ReadError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

pub trait GPXFilepathPort {
    fn load(&self, file_path: &Path) -> Result<GpxTrack, ReadError>;
    fn save(&self, file_path: &Path, gpx_track: &GpxTrack);
}
