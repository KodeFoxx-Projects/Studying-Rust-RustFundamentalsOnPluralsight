use std::fmt;

fn main() {
    let vector_of_i32 = create_vector(3);
    print_vector_info(&vector_of_i32);

    let mut mutable_vector = create_vector(10);
    mutable_vector.push(6845);
    print_vector_info(&mutable_vector);
    let last_element = mutable_vector.pop();
    print_vector_info(&mutable_vector);
    println!("Variable last_element is an Option<T>: '{:?}'.", last_element);
    
    println!("");
    println!("Printing 'mutable_vector' values by using pop:");
    while let Some(value) = mutable_vector.pop() {
        println!(" - {}", value)
    }
}

fn print_vector_info<T>(vector: &Vec<T>)
    where T: 
        std::fmt::Debug 
      + std::fmt::Display
{
    // print out information
    println!("Vector contains #{} elements, the first value is '{}'.", vector.len(), vector[0]);
    println!(" -> Vector's content: {:?}", vector);

    // get an element at 999th position (doesn't exist)
    println!(" -> Trying to get value at certain position...");
    print_value_from_vector_at_position(&vector, 1);
    print_value_from_vector_at_position(&vector, 2);
    print_value_from_vector_at_position(&vector, 9999);

    // empty line
    println!("");
}

fn print_value_from_vector_at_position<T>(vector: &Vec<T>, position: usize) 
    where T: 
        std::fmt::Debug 
      + std::fmt::Display
{
    println!("    - Trying to get value at position '{}:", position);
    match vector.get(position) {
        Some(value) => println!("      - Value at index '{}' is '{}.'", position, value),
        None => println!("      - No element at index '{}'.", position)
    };
}

fn create_vector(length: i32) -> Vec<i32> {
    let mut vector = Vec::new();

    for i in 1..(length+1) {
        vector.push(i as i32);
    }

    return vector;
}
