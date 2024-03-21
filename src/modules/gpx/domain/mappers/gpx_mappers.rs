use crate::modules::gpx::{
    adapters::gpx_filepath_adapter::{RawGpxTrack, TrackPoint, TrackPt, TrackSegment},
    domain::gpx_track::{GpxPoint, GpxTrack},
};

pub fn map_raw_gpx_to_gpx(
    raw_gpx: crate::modules::gpx::adapters::gpx_filepath_adapter::RawGpxTrack,
) -> GpxTrack {
    let mut segments = Vec::new();

    for segment in raw_gpx.track {
        for segment in segment.segments {
            for point in &segment.points {
                segments.push(GpxPoint {
                    latitude: point.lat,
                    longitude: point.lon,
                    elevation: point.elevation.unwrap_or(0.0),
                    time: point.time,
                });
            }
        }
    }

    GpxTrack::new(segments)
}

pub fn map_gpx_to_raw_gpx(gpx: GpxTrack) -> RawGpxTrack {
    let mut track_segment = TrackSegment {
        segments: Vec::new(),
    };

    let mut track_pt = TrackPt { points: Vec::new() };

    for point in gpx.segments {
        track_pt.points.push(TrackPoint {
            lat: point.latitude,
            lon: point.longitude,
            elevation: Some(point.elevation),
            time: point.time,
        });
    }

    track_segment.segments.push(track_pt);

    let mut raw_gpx = RawGpxTrack { track: Vec::new() };

    raw_gpx.track.push(track_segment);

    raw_gpx
}
