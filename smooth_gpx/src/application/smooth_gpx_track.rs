use std::path::Path;

use crate::ports::gpx_filepath_port::GPXFilepathPort;

pub(crate) struct SmoothGpxTrack<F>
where
    F: GPXFilepathPort,
{
    filepath_adapter: F,
}

impl<F> SmoothGpxTrack<F>
where
    F: GPXFilepathPort,
{
    pub fn new(filepath_adapter: F) -> SmoothGpxTrack<F> {
        SmoothGpxTrack { filepath_adapter }
    }

    pub fn execute(&self, data: &Path, output: &Path, number_to_insert: i32) {
        let gpx_track = self.filepath_adapter.load(data);

        let mut loaded_track = match gpx_track {
            Ok(gpx_track) => gpx_track,
            Err(_e) => panic!("Error loading gpx file"),
        };

        loaded_track.smooth_gpx_track(number_to_insert);

        self.filepath_adapter.save(output, &loaded_track);
    }
}
