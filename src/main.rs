use clap::Parser;
use smooth_gpx;

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

pub fn main() {
    let args = Args::parse();

    let path = std::path::Path::new(&args.gpxfile);
    let output = std::path::Path::new(&args.output);

    smooth_gpx::smooth_gpx_track(path, output, args.number_to_insert);
}
