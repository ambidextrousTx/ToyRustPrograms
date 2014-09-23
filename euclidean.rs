// Using structs to represent 2D points
// and calculating the Euclidean distance between them

mod pointutils {

    pub struct Point {
        pub x: f64,
        pub y: f64,
    }


    pub fn euclidean(point_a: Point, point_b: Point) -> f64 {
        0.0

    }
}

fn main() {
    let point_a: pointutils::Point = pointutils::Point { x: 0.3, y: 20.0 };
    let point_b: pointutils::Point = pointutils::Point { x: 4.0, y: -0.03 };

    let euclidean_distance = pointutils::euclidean(point_a, point_b);
    // let euclidean_distance = euclidean(point_a, point_b);
    println!("Found distance: {}", euclidean_distance);

}
