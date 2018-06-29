use std::mem;

fn main() {
    // fixed length, known up front
    let mut array:[i32;5] = [1,2,3,4,5];
    let mut array_another_way_of_declaring = [1,2,3,4,5];
    print_array_info(&array);    
    array[0] = 321;
    print_array_info(&array);

    // auto fill array
    let mut autofill = [1i32; 10];
    print_array_info(&autofill);
    // now let's fill it with a for loop from 1..10
    for i in 0..autofill.len() {
        autofill[i] = i as i32;
    }
    print_array_info(&autofill);

    // matrices
    let matrix = [
        [1i32, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1]
    ];
    print_matrix_info(&matrix);
}

fn print_array_info(array: &[i32]) {
    // outputting information
    println!("The given array has {}# elements, with the first element containing value '{}'.", 
        array.len(),
        array[0]
    );
    println!(" -> Values: {:?}", array);
    println!(" -> Takes up {} bytes", std::mem::size_of_val(array));

    // playing with if
    if array.len() == 5 {
        if array != [1,2,3,4,5] { 
            println!(" -> The array has been modified!");
        } else {
            println!(" -> The array was left untouched.");
        };
    }

    // line seperator
    println!("");
}

fn print_matrix_info(matrix: &[[i32;5]; 5]) {
    // outputting information
    println!("The given matrix has {} rows.", 
        matrix.len()
    );
    println!(" -> Matrix values: {:?}", matrix);
    println!(" -> Takes up {} bytes", std::mem::size_of_val(matrix));

    // plauing with for loop
    println!(" -> Values from top left to bottom right:");
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len() {
            if i == j {
                println!("      - [{}][{}] = {}", i, j, matrix[i][j])
            }
        }
    }

    // line seperator
    println!("");
}
