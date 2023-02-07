#[derive(Copy, Drop)]
struct Point {
    x: felt,
    y: felt,
}

fn point( x: felt, y: felt) -> Point {
    Point{x: x, y: y}
}