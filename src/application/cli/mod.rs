use clap::Parser;

use crate::modules::gpx::adapters::gpx_filepath_adapter::GPXFilepathAdapter;
use crate::modules::gpx::application::smooth_gpx_track;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the gpx file
    #[arg(short, long)]
    gpxfile: String,

    /// Number of points to insert between each point
    #[arg(short, long, default_value = "2")]
    number_to_insert: i32,

    /// Output file
    #[arg(short, long)]
    output: String,
}

pub fn run() {
    let args = Args::parse();

    let gpx_filepath_adapter = GPXFilepathAdapter {};

    let smooth_gpx_track = smooth_gpx_track::SmoothGpxTrack::new(gpx_filepath_adapter);

    let path = std::path::Path::new(&args.gpxfile);
    let output = std::path::Path::new(&args.output);

    smooth_gpx_track::SmoothGpxTrack::execute(
        &smooth_gpx_track,
        &path,
        &output,
        args.number_to_insert,
    );
}
