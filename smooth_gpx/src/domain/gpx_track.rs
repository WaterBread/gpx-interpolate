use chrono::{DateTime, Duration, Utc};
#[derive(Clone)]
pub struct GpxPoint {
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: f32,
    pub time: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub(crate) struct GpxTrack {
    pub segments: Vec<GpxPoint>,
}

impl GpxTrack {
    pub fn new(segments: Vec<GpxPoint>) -> GpxTrack {
        GpxTrack { segments }
    }

    pub fn smooth_gpx_track(&mut self, number_to_insert: i32) {
        let mut new_segments = Vec::new();

        if self.segments.is_empty() || number_to_insert < 1 {
            return;
        }

        for window in self.segments.windows(2) {
            let (start, end) = (&window[0], &window[1]);

            if let (Some(start_time), Some(end_time)) = (start.time, end.time) {
                let time_interval =
                    (end_time - start_time).num_seconds() as f64 / (number_to_insert + 1) as f64;

                new_segments.push(start.clone());

                for i in 1..=number_to_insert {
                    let fraction = i as f32 / (number_to_insert + 1) as f32;

                    let new_lat = start.latitude + fraction * (end.latitude - start.latitude);
                    let new_lon = start.longitude + fraction * (end.longitude - start.longitude);

                    let new_ele = start.elevation + fraction * (end.elevation - start.elevation);
                    let new_time =
                        start_time + Duration::seconds((time_interval * i as f64) as i64);

                    new_segments.push(GpxPoint {
                        latitude: new_lat,
                        longitude: new_lon,
                        elevation: new_ele,
                        time: Some(new_time),
                    });
                }
            }
        }

        new_segments.push(self.segments.last().unwrap().clone());

        self.segments = new_segments;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::TimeZone;

    #[test]
    fn test_smooth_gpx_track() {
        let mut new_segments = Vec::new();

        new_segments.push(GpxPoint {
            latitude: 0.0,
            longitude: 0.0,
            elevation: 0.0,
            time: Some(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
        });

        new_segments.push(GpxPoint {
            latitude: 2.0,
            longitude: 2.0,
            elevation: 0.0,
            time: Some(Utc.with_ymd_and_hms(2023, 1, 1, 6, 0, 0).unwrap()),
        });

        let mut track = GpxTrack::new(new_segments);

        track.smooth_gpx_track(1);
        assert_eq!(track.segments.len(), 3);
        assert_eq!(track.segments[1].latitude, 1.0);
        assert_eq!(track.segments[1].longitude, 1.0);
        assert_eq!(track.segments[1].elevation, 0.0);
    }
}
