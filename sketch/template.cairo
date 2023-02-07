use draw::point;
use draw::Point;

// Set return tuple with all shapes you are returning
fn main() -> (Point, Point) {
    let p1_X = $1;
    let p1_Y = $2;
    let p2_X = $3;
    let p2_Y = $4;
  (
    point(p1_X, p1_Y),
    point(p2_X, p2_Y),
  )
}