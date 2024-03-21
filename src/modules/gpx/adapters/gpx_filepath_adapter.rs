use crate::modules::gpx::domain::mappers::gpx_mappers;
use crate::modules::gpx::ports::gpx_filepath_port::ReadError;
use crate::modules::gpx::{domain::gpx_track::GpxTrack, ports::gpx_filepath_port::GPXFilepathPort};
use chrono::{DateTime, Utc};
use quick_xml::de::from_str;
use serde::Deserialize;
use serde_derive::Serialize;
use std::{fmt::Debug, fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "gpx")]
pub struct RawGpxTrack {
    #[serde(rename = "trk")]
    pub track: Vec<TrackSegment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackSegment {
    #[serde(rename = "trkseg")]
    pub segments: Vec<TrackPt>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackPt {
    #[serde(rename = "trkpt")]
    pub points: Vec<TrackPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackPoint {
    #[serde(rename = "@lat")]
    pub lat: f32,
    #[serde(rename = "@lon")]
    pub lon: f32,
    #[serde(rename = "ele", default)]
    pub elevation: Option<f32>,
    pub time: Option<DateTime<Utc>>,
}

pub fn parse_gpx(gpx_data: &str) -> Result<RawGpxTrack, ReadError> {
    let parsed = from_str::<RawGpxTrack>(gpx_data);

    match parsed {
        Ok(gpx) => Ok(gpx),
        Err(e) => Err(ReadError::Parse(e.to_string())),
    }
}

pub(crate) struct GPXFilepathAdapter;
impl GPXFilepathPort for GPXFilepathAdapter {
    fn load(&self, path: &Path) -> Result<GpxTrack, ReadError> {
        let contents = fs::read_to_string(path);

        let contents = match contents {
            Ok(contents) => contents,
            Err(e) => return Err(ReadError::Io(e)),
        };

        let gpx = parse_gpx(&contents)?;

        let gpx_track: GpxTrack = gpx_mappers::map_raw_gpx_to_gpx(gpx);

        Ok(gpx_track)
    }

    fn save(&self, file_path: &Path, gpx_track: &GpxTrack) {
        let raw_gpx = gpx_mappers::map_gpx_to_raw_gpx(gpx_track.clone());

        let xml = quick_xml::se::to_string(&raw_gpx);

        // Write to a new file
        match xml {
            Ok(xml) => {
                // Add the xml header
                let formatted_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", xml);
                fs::write(file_path, formatted_xml).expect("Unable to write file");
            }
            Err(e) => panic!("Error serializing gpx file: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::gpx::domain::gpx_track::GpxPoint;

    use super::*;
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn test_parse_gpx() {
        let gpx_data = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <gpx version="1.1" creator="Xcode">
                <trk>
                    <trkseg>
                        <trkpt lat="51.0" lon="0.0">
                            <ele>0.0</ele>
                            <time>2020-01-01T00:00:00Z</time>
                        </trkpt>
                    </trkseg>
                </trk>
            </gpx>
        "#;

        let gpx_track = parse_gpx(gpx_data);

        match gpx_track {
            Ok(gpx_track) => {
                assert_eq!(gpx_track.track.len(), 1);
                assert_eq!(gpx_track.track[0].segments[0].points.len(), 1);

                assert_eq!(gpx_track.track[0].segments[0].points[0].lat, 51.0);
                assert_eq!(gpx_track.track[0].segments[0].points[0].lon, 0.0);
                assert_eq!(
                    gpx_track.track[0].segments[0].points[0].elevation,
                    Some(0.0)
                );
            }
            Err(e) => panic!("Error parsing gpx file: {}", e),
        }
    }

    #[test]
    fn test_load() {
        let gpx_filepath_adapter = GPXFilepathAdapter;

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/test.gpx");

        println!("GPX: {}", d.to_str().unwrap());

        let gpx_track = gpx_filepath_adapter.load(&d);

        let gpx_track = match gpx_track {
            Ok(gpx_track) => gpx_track,
            Err(e) => panic!("Error loading gpx file: {:?}", e),
        };

        assert_eq!(gpx_track.segments.len(), 2);
        assert_eq!(gpx_track.segments[0].latitude, 48.8566);
        assert_eq!(gpx_track.segments[0].longitude, 2.3522);
        assert_eq!(gpx_track.segments[0].elevation, 0.0);

        assert_eq!(gpx_track.segments[1].latitude, 51.5074);
        assert_eq!(gpx_track.segments[1].longitude, -0.1278);
        assert_eq!(gpx_track.segments[1].elevation, 0.0);
    }

    #[test]
    fn test_save() {
        let tmp_dir = TempDir::new("gpx_save_test").unwrap();
        let file_path = tmp_dir.path().join("test_gpx.gpx");

        let gpx = GpxTrack::new(vec![
            GpxPoint {
                latitude: 48.8566,
                longitude: 2.3522,
                elevation: 0.0,
                time: Some(Utc::now()),
            },
            GpxPoint {
                latitude: 51.5074,
                longitude: -0.1278,
                elevation: 0.0,
                time: Some(Utc::now()),
            },
        ]);

        let gpx_filepath_adapter = GPXFilepathAdapter;

        gpx_filepath_adapter.save(&file_path, &gpx);

        let gpx_track = gpx_filepath_adapter.load(&file_path);

        let gpx_track = match gpx_track {
            Ok(gpx_track) => gpx_track,
            Err(e) => panic!("Error loading gpx file: {:?}", e),
        };

        assert_eq!(gpx_track.segments.len(), 2);
        assert_eq!(gpx_track.segments[0].latitude, 48.8566);
        assert_eq!(gpx_track.segments[0].longitude, 2.3522);
        assert_eq!(gpx_track.segments[0].elevation, 0.0);

        assert_eq!(gpx_track.segments[1].latitude, 51.5074);
        assert_eq!(gpx_track.segments[1].longitude, -0.1278);
        assert_eq!(gpx_track.segments[1].elevation, 0.0);
    }
}
