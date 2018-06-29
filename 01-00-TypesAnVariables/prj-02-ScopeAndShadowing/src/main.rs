fn main() {
    let a = 5;
    println!("a = {}", a); // a is in scope of function main, local scope.

    some_function();

    {
        let b = 456;
        println!("b = {}", b);

        println!("a = {}", a); // but a does exists in this inner scope

        let a = 123123; // but we can declare another a in the inner scope
        println!("a = {}", a); // so we can "redeclare" the variable
    }

    // here b is not available...
    // println!("b = {}", b); // this won't compile!

    // the outer a is still 5
    println!("a = {}", a);
}

fn some_function(){
    // a doesn't exist here
    // println!("a = {}", a); // this won't compile!
}
