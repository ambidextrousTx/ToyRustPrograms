// Using structs to represent 2D points
// and calculating the Euclidean distance between them

struct Point {
    x: f64,
    y: f64,
}

mod pointutils {
    pub fn euclidean(point_a: Point, point_b: Point) {
        0.0
    
    }

}

fn main() {
    let point_a: Point = Point { x: 0.3, y: 20.0 };
    let point_b: Point = Point { x: 4.0, y: -0.03 };

    let euclidean_distance = pointutils::euclidean(point_a, point_b);
    println!("Found distance: ", euclidean_distance);

}
