fn if_statement(){
    let temp = 35;

    // basic if flow
    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is ok!");
    }

    // if as an expression
    let day = if temp > 20 {"sunny"} else {"cloudy"}; // the if is actually an expression in rust!
    println!("today is {}", day);
    println!(
        "it is {}", 
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"}
    );

    // if inside if's (nested)
    println!(
        "it is {}",         
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}            
        } else if temp < 10 {"cold"} else {"ok"}
    );
}

fn main() {
    if_statement();
}
