fn while_and_loop() {
    let mut x = 1;
    
    println!("while");
    // while loop
    while x < 1000 {
        x *= 2;
        println!("x = {}", x);
    }

    println!("");
    println!("while (with continue;)");
    // while loop with continue instruction
    x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 { continue; } // this skips the value 64, skips everything below
        println!("x = {}", x);
    }

    println!("");
    println!("loop (with break;)");
    // loop
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; } // breaks out of the loop, can also be used in the while loop
    }

    println!("");
    println!("for loop");
    for x in 1..11 { //11 is the upper bound
        println!("x = {}", x);
    }

    println!("");
    println!("for loop (position, and element)");
    for (pos,x) in (30..41).enumerate() {
        println!("{}: is = {}", pos, x);
    }
}

fn main() {
    while_and_loop();
}
