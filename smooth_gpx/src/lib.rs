use std::path::Path;

use adapters::gpx_filepath_adapter::GPXFilepathAdapter;

mod adapters;
mod application;
mod domain;
mod ports;

pub fn smooth_gpx_track(filepath: &Path, output: &Path, number_to_insert: i32) {
    let filepath_adapter = GPXFilepathAdapter {};

    let app = application::smooth_gpx_track::SmoothGpxTrack::new(filepath_adapter);

    app.execute(filepath, output, number_to_insert);
}
