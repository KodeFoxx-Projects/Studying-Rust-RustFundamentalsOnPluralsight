#![allow(dead_code)]
use std::mem;

    struct Point{
        x: f64,
        y: f64
    }

    fn origin() -> Point {
        return Point{x: 64.581, y: 15468.220};
    }

    pub fn stack_and_heap() {
        let point1 = origin(); // stack allocated    
        println!("point1 takes up {} bytes", mem::size_of_val(&point1));
        println!("point1 x:{} y:{}", point1.x, point1.y);

        let point2 = Box::new(origin()); // heap allocated, this uses less space because it uses only the address on the stack
        println!("point2 takes up {} bytes", mem::size_of_val(&point2));
        println!("point2 x:{} y:{}", point2.x, point2.y);

        let point3 = *point2; // here we are unboxing, an reallocating it to the stack
        println!("point3 takes up {} bytes", mem::size_of_val(&point3));
        println!("point3 x:{} y:{}", point3.x, point3.y);
    }    