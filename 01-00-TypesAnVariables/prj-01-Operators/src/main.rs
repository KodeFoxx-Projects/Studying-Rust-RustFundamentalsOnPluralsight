fn main() {
    showcase_rust_operators();
}

fn showcase_rust_operators(){
    // arithmethic
    let mut a = 2+3*4/2; // +-*/
    println!("{} | result of 2+3*4/2 should be '{}', acutal value is '{}'.", 8 == a, 8, a);
    
    a += 1;    
    println!("{} | result of a += 1 should be '{}', actual value is '{}'.", 9 == a, 9, a);


    // modulus    
    println!("{} | result of 13%2 should be '{}', actual value is '{}'.", 13%2 == 1, 1, 13%2);


    // power of x
    let n = 5;
    let power_of_result = i32::pow(n, 2);
    println!("{} | result of 5² should be '{}', actual value is '{}'.", 25 == power_of_result, 25, power_of_result);

    let b = 2.5;
    let b_cubed = f64::powi(b, 2);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b to the power of 2 => {}", b_cubed);
    println!("b to the power of 3.14 => {}", b_to_pi);


    // bitwise
    let c = 1 | 2; // | OR
                   // & AND
                   // ^ XOR
                   // ! NOR
    println!("{}", c);


    // shift operators
    let two_to_10 = 1 << 10;
    println!("2 to the power of ten (with shift to the left) == {}", two_to_10);


    // logical operators
    let pi_less_than_four = std::f64::consts::PI < 4.0;
    println!("{}", pi_less_than_four);
    // equality is done by '==', inequality is '!='
}
