use std::mem;

fn main() {
    // allocate a chunk of memory by declaring a variable
    // - variable with name a
    //   of type u8
    //   - the letter 'u' stands for unsigned (zero or positive)
    //   - the number '8' stands for the amount of bits
    //   => thus we declarted 8bits for a
    let a:u8 = 123;

    // let's try an print using the println macro
    println!("a = {}", a);

    println!("");
    println!("------------------------");

    // if we try changing a variable, we get an error because
    // the variable is immutable
    // if you want it to be mutable, you have to explicitly declare it
    // a = 456;

    // let's make a mutable variable that we can change
    let mut b:u8 = 123;
    println!("b = {}", b);
    b += a;
    println!("b = {} == 246", b);

    println!("");
    println!("------------------------");

    // you can omit the type declaration and
    // try to let rust figure out what it is...
    let number = 123456789;
    println!("number = {} | size = {} bytes", number, mem::size_of_val(&number));
    let text = "This is a string";
    println!("text = {} | size = {} bytes", text, mem::size_of_val(&text));
    let character = 'c'; // 4 bytes
    println!("character = {} | size = {} bytes", text, mem::size_of_val(&character));
    let boolean = false;
    println!("boolean = {} | size = {} bytes", boolean, mem::size_of_val(&boolean));
    let floating_point = 2.546; // default is a double-precision, 8 bytes or 64bits; f64 -> it's a floating point (f32 also exists)
    println!("floating_point = {} | size = {} bytes", boolean, mem::size_of_val(&floating_point));

    println!("");
    println!("------------------------");

    // isize and usize can be useful when you need to have a variable the which has 
    // the size of the memory address of the system you are running on.
    let isize_variable:isize = 123;
    let size_of_isize_variable = mem::size_of_val(&isize_variable);
    println!("isizeVariable = {} | size = {} bytes | {}-bit OS", isize_variable, size_of_isize_variable, size_of_isize_variable * 8);
    let usize_variable:usize = 321;
    let size_of_usize_variable = mem::size_of_val(&usize_variable);
    println!("usizeVariable = {} | size = {} bytes | {}-bit OS", usize_variable, size_of_usize_variable, size_of_usize_variable * 8);
}
