// use stack_and_heap;

mod stack_and_heap;

fn main() {
    let x = 5; // i32
    // this is placed on the stack, 
    //   - short term storage
    //   - faster, but limited space

    let y = Box::new(10);
    // this is placed on the heap,
    //   - long term storage
    //   - y is on the stack, but it contains the address of the location in memory where the value of y is located
    //     aka y holds a reference    
    println!("y's value = {}", *y);


    // stack and heap demonstration in other module
    stack_and_heap::stack_and_heap();
}
