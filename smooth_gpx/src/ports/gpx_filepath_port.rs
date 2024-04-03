use std::path::Path;

use crate::domain::gpx_track::GpxTrack;

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Parse(String),
}

pub trait GPXFilepathPort {
    fn load(&self, file_path: &Path) -> Result<GpxTrack, ReadError>;
    fn save(&self, file_path: &Path, gpx_track: &GpxTrack);
}
