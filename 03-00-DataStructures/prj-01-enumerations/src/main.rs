enum Color {
    None,
    Red, 
    Green,
    Blue,    
    RgbColor(u8, u8, u8), // some kind of tuple
    CmykColor{cyan:u8, magenta:u8, yellow: u8, black: u8} // struct in stead of tuple, so it's named
}

fn main() {
    match_color(Color::Red);
    match_color(Color::RgbColor(0,0,0));
    match_color(Color::RgbColor(0,0,20));
    match_color(Color::CmykColor{cyan: 0, magenta: 0, yellow: 0, black: 255});
}

fn match_color(color:Color) {
    match color {
        Color::Red 
            => println!("{}", "R"),
        Color::Green 
            => println!("{}", "G"),
        Color::Blue 
            => println!("{}", "B"),
          Color::RgbColor(0,0,0) 
        | Color::CmykColor{cyan:_, magenta:_, yellow:_, black: 255} 
            => println!("Black as night!"),        
        _ => println!("{}", "?")
    };
}