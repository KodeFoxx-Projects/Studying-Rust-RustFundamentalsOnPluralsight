const PI_SHORTHAND:f64 = 3.14; // this constant no fixed address

static MEANING_OF_LIFE:i8 = 42; // static is another way of a global value
static mut MUTABLE_GLOBAL_VARIABLE:i8 = 24;

fn main() {
    println!("Pi in short form is '{}'", PI_SHORTHAND);
    println!("Meaning of life is '{}'", MEANING_OF_LIFE);

    unsafe {
        println!("MUTABLE GLOBAL VARIABLE '{}'", MUTABLE_GLOBAL_VARIABLE);
        MUTABLE_GLOBAL_VARIABLE = 0; // marking it as unsafe, we can of course then change a mutable globally declared variable
        println!("MUTABLE GLOBAL VARIABLE '{}'", MUTABLE_GLOBAL_VARIABLE);
    }
}
