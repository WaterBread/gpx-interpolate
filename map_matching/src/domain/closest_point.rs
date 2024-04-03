use geo::algorithm::haversine_distance::HaversineDistance;
use geo::Point;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Element {
    geometry: Vec<Geometry>,
}

#[derive(Deserialize, Debug)]
struct Geometry {
    lat: f64,
    lon: f64,
}

fn find_closest_point(gpx_point: &Point<f64>, elements: &[Element]) -> (Point<f64>, f64) {
    let mut closest_point = Point::new(0.0, 0.0);
    let mut min_distance = f64::MAX;

    for element in elements {
        for point in &element.geometry {
            let current_point = Point::new(point.lon, point.lat);
            let current_distance = gpx_point.haversine_distance(&current_point);
            if current_distance < min_distance {
                min_distance = current_distance;
                closest_point = current_point;
            }
        }
    }

    (closest_point, min_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_point() {
        let elements = vec![Element {
            geometry: vec![
                Geometry {
                    lat: 50.7451378,
                    lon: 7.1736881,
                },
                Geometry {
                    lat: 50.7451378,
                    lon: 7.1736881,
                },
            ],
        }];

        let gpx_point = Point::new(7.1736881, 50.7451378);

        let (closest_point, distance) = find_closest_point(&gpx_point, &elements);

        assert_eq!(closest_point, Point::new(7.1736881, 50.7451378));
        assert_eq!(distance, 0.0);
    }
}
