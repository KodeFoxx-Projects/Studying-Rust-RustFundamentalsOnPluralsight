struct Point {
    x:i32,
    y:i32
}

struct Line {
    start: Point,
    end: Point
}

fn main() {
    let point1 = Point { x: 10, y: 45 };
    println!("point1 (x: {} y: {})", point1.x, point1.y);

    let point2 = Point { x: 6, y: 45 };
    println!("point2 (x: {} y: {})", point2.x, point2.y);

    let line = Line { start: point1, end: point2 };
    println!("line starts at (x: {}, y: {}) and ends at (x: {}, y: {})", line.start.x, line.start.y, line.end.x, line.end.y)
}
