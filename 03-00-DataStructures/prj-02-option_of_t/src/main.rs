fn main() {
    // Option<T>
    let some = demonstration_of_some_or_none(3.0, 2.0);
    let none = demonstration_of_some_or_none(3.0, 0.0);    
}

fn demonstration_of_some_or_none(x: f64, y:f64) -> Option<f64> {
    // Option<T> can either contain Some(value) or None
    let result:Option<f64> = 
        if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);

    match result {
        Some(value) => println!(" -> The result of the the division is '{}'.", value),
        None => println!(" -> Invalid operation (divide by zero)!!")
    }

    // destructing when some, if destructing fails then it doesn't get executed
    if let Some(value) = result { println!("    value = {}", value); }

    return result;
}