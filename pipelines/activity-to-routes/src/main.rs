use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use kd_tree::{ItemAndDistance, KdPoint, KdTree};
use polyline::decode_polyline;
use std::{fs, thread, vec};
use priority_queue::PriorityQueue;


#[derive(Debug, PartialOrd, PartialEq, Serialize)]
pub struct CmpF64(pub f64);

impl Eq for CmpF64 {}

impl Ord for CmpF64 {
    fn cmp(&self, other: &Self) -> Ordering {
	if let Some(ordering) = self.partial_cmp(other) {
	    ordering
	} else {
	    // Choose what to do with NaNs, for example:
	    Ordering::Less
	}
    }
}

impl CmpF64 {
    pub fn to_f64(&self) -> f64 {
        match &self {
            CmpF64(n) => -(*n)
        }
    }
}

#[derive(Serialize, Hash, PartialEq, Eq)]
pub struct GraphPath {
    pub path: Vec<String>,
    pub add_cost: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Node {
    lat: f64,
    lon: f64,
    neighbors: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Activity {
    id: i64,
    map: ActivityMap,
}

#[derive(Debug, Clone, Deserialize)]
struct ActivityMap {
    summary_polyline: String,
}

#[derive(Debug, Clone)]
struct Item {
    point: [f64; 2],
    id: usize,
}

impl KdPoint for Item {
    type Scalar = f64;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f64 { self.point[k] }
}


fn load_data() -> (HashMap<String, Node>, Vec<Activity>) {
    let graph_data = fs::read_to_string("../graph.json").expect("Unable to read graph.json");
    let activities_data = fs::read_to_string("../inputs/strava_activities.json").expect("Unable to read strava_activities.json");
    let graph: HashMap<String, Node> = from_str(&graph_data).expect("Failed to parse graph");
    let activities: Vec<Activity> = from_str(&activities_data).expect("Failed to parse activities");
    (graph, activities)
}

fn build_kd_tree(graph: &HashMap<String, Node>) -> KdTree<Item> {
    let point_vecs = graph.iter()
        .map(|(id, node)| {
            Item {
                point: [node.lat, node.lon],
                id: from_str::<usize>(id).expect("Failed to parse id"),
            }
        })
        .collect::<Vec<_>>();
    // print first 10 points
    for point in point_vecs.iter().take(10) {
        // println!("Point: ({}, {})", point.point[0], point.point[1]);
    }

    KdTree::build_by_ordered_float(point_vecs)
}

fn get_nearest_node(kd_tree: &KdTree<Item>, lat: f64, lon: f64) -> Option<(String, f64)> {
    let query_point = [lat, lon];
    let nearest = kd_tree.nearest(&query_point).unwrap();
    let id = nearest.item.id;
    let distance = nearest.squared_distance.sqrt();
    Some((id.to_string(), distance))
}

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    node: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the BinaryHeap is a max-heap, so we reverse the order for min-heap behavior
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn l2_distance(a: &Node, b: &Node) -> f64 {
    let dx = a.lat - b.lat;
    let dy = a.lon - b.lon;
    (dx * dx + dy * dy).sqrt()
}

// fn shortest_path(graph: &HashMap<String, Node>, start: &str, end: &str) -> Option<Vec<String>> {

pub fn shortest_path(start_node: &String, end_node: &String, graph: &HashMap<String, Node>)-> Option<Vec<String>> {
    /// Init work list and empty explored set
    let mut pq = PriorityQueue::new();
    let mut explored_set = HashSet::new();


    // Init pq with current start node
    pq.push(
        GraphPath{path: vec![start_node.clone()], add_cost: "0.0".to_string()},
        CmpF64(0.0)
    );

    // Loop until pq is empty or first path is found, always explore lower-cost paths first
    while let Some((graph_path, length)) = pq.pop() {
        // Extract the node path and the outside penalty (add_cost)
        let cur_path = graph_path.path;
        // println!("Current path: {:?}, start: {:?}, end: {:?}", cur_path, start_node, end_node);
        let cur_add_cost = graph_path.add_cost;
        // number of nodes in path 
        let path_len = cur_path.len();
        assert!(path_len > 0);
        
        // Extracting neighbors from the most recently added node to the path
        let last_node_id = cur_path[path_len-1].clone();
        let last_node = match graph.get(&last_node_id) {
            None => continue,
            Some(node) => node,
        };

        if !explored_set.contains(&last_node_id) {
            explored_set.insert(last_node_id);
        }
        else {
            continue
        }

        // Check if the popped off node is in the goal set
        let last_node_id = cur_path[path_len-1].clone();
        if end_node.clone() == last_node_id {
            return Some(cur_path.to_vec());
        }

        // Add neighbors
        let neighbors = &last_node.neighbors;
        for node_id in neighbors.iter() {
            let mut path_copy = cur_path.to_vec();
            path_copy.push(node_id.to_string());
            let dist = l2_distance(last_node, graph.get(node_id).unwrap());
            let CmpF64(float_len) = length;
            
            let new_length = -float_len + dist; 
            let cur_add_f64 = cur_add_cost.parse::<f64>().unwrap();
            pq.push(GraphPath{
                path: path_copy,
                add_cost: (cur_add_f64).to_string()
            }, CmpF64(-new_length));
        }
    }
    None
}

fn main() {
    let (graph, activities) = load_data();
    let kd_tree = build_kd_tree(&graph);
    let mut successful_activities = 0;
    for activity in activities {
        let polyline = decode_polyline(&activity.map.summary_polyline, 5).unwrap();
        let start = polyline.clone().into_iter().next();
        
        if let Some(start) = start {
            // println!("Start point: ({}, {})", start.y, start.x);
            if start.y < 40.38 || start.y > 40.513333333333335 || start.x < -80.05 || start.x > -79.85 {
                // println!("Skipping activity ID: {}", activity.id);
                continue;
            }
        
        } else {
            // println!("No start point found for activity ID: {}", activity.id);
            continue;
        }

        let mut path = vec![];

        for point in polyline {
            if let Some((nearest_node_id, dist)) = get_nearest_node(&kd_tree, point.y, point.x) {
                path.push(nearest_node_id);
            }
        }

        if path.len() < 2 {
            continue;
        }

        // println!("Activity ID: {}", activity.id);
        // println!("Path: {:?}", path);


        let connected_paths = path.windows(2).map(
            |window| {
                let start = &window[0];
                let end = &window[1];
                shortest_path(start, end, &graph).unwrap_or(vec![start.to_string(), end.to_string()])
            }
        );

        let route = connected_paths.flatten().collect::<Vec<_>>();
        //  write route to file named "routes/{activity.id}.txt"
        let mut file = File::create_new(format!("routes/{}.txt", activity.id)).unwrap();
        file.write_all(format!("{:?}", route).as_bytes()).unwrap();
        
        // println!("Route: {:?}", route);
        // if let Some(route) = shortest_path(&graph, start, end) {
        //     println!("Activity ID: {}", activity.id);
        //     println!("Route: {:?}", route);
        // } else {
        //     println!("No route found for activity ID: {}", activity.id);
        // }

        successful_activities += 1;
        println!("Activity ID: {} processed successfully ({} / {})", activity.id, successful_activities, 715);
    }
    println!("Total successful activities: {}", successful_activities);
}