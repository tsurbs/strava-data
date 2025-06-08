use std::fs::{File, read_to_string};
use std::io::Write;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use polyline::decode_polyline;


const REGION_BOUNDARY: [[f64; 2]; 2] = [
    // top-right corner
    [-80.05663907003299, 40.36757390135135], 
    // bottom-left corner
    [-79.83633552682205, 40.53113770150144],
];

#[derive(Debug, Clone, Deserialize)]
struct Activity {
    id: i64,
    map: ActivityMap,
}

#[derive(Debug, Clone, Deserialize)]
struct ActivityMap {
    summary_polyline: String,
}

#[derive(Debug, Clone, Serialize)]
struct NormActivityData {
    points: Vec<[f64; 2]>,
    id: usize,
}

#[derive(Debug, Clone, Serialize)]
struct NormData {
    activities: Vec<NormActivityData>,
    region: [[f64; 2]; 2],
}
impl NormData {
    fn len(&self) -> usize {
        self.activities.len()
    }
    fn push(&mut self, activity: NormActivityData) {
        self.activities.push(activity);
    }
}


type Point = [f64; 2];

fn load_data() -> Vec<Activity> {
    let activities_data = read_to_string("../../inputs/strava_activities.json").expect("Unable to read strava_activities.json");
    let activities: Vec<Activity> = from_str(&activities_data).expect("Failed to parse activities");
    activities
}

fn normalized_point(point: geo_types::Coord, boundary: [[f64; 2]; 2]) -> [f64; 2] {
    // lat and lon ratio varying by location n the earth's surface shouldn't really 
    // matter here since the region ends up being unitless
    let x = (point.x - boundary[0][0]) / (boundary[1][0] - boundary[0][0]);
    let y = (point.y - boundary[1][1]) / (boundary[0][1] - boundary[1][1]);
    [x, y]
}

fn main() {
    let activities = load_data();
    let mut all_activities = NormData {
        activities: Vec::new(),
        region: REGION_BOUNDARY,
    };
    for activity in activities {
        let polyline = decode_polyline(&activity.map.summary_polyline, 5).unwrap();
        let start = polyline.clone().into_iter().next();
        
        if let Some(start) = start {
            // println!("Start point: ({}, {})", start.y, start.x);
            if start.y < REGION_BOUNDARY[0][1] || start.y > REGION_BOUNDARY[1][1] ||
               start.x < REGION_BOUNDARY[0][0] || start.x > REGION_BOUNDARY[1][0] {
                // println!("Skipping activity ID: {}", activity.id);
                continue;
            }
        
        } else {
            // println!("No start point found for activity ID: {}", activity.id);
            continue;
        }

        let mut normed_activity: NormActivityData = NormActivityData {
            points: Vec::new(),
            id: activity.id as usize,
        };

        for (i, point) in polyline.into_iter().enumerate() {
            let norm_point = normalized_point(point, REGION_BOUNDARY);
            normed_activity.points.push([norm_point[0], norm_point[1]]);
        }        
        all_activities.push(normed_activity);
        println!("Activity ID: {} processed successfully ({} / {})", activity.id, all_activities.len(), 775);
    }
    let mut file = File::create("../../normalized_activities.json").expect("Unable to create file");
    let json_data = serde_json::to_string(&all_activities).expect("Failed to serialize activities");
    file.write_all(json_data.as_bytes()).expect("Failed to write to file");
    
    println!("Total successful activities: {}", all_activities.len());
}