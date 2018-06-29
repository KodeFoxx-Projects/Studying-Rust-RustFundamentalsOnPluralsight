fn match_statement() {
    let country_code = 32; // 1 to 9999

    let country = match country_code {
        32 => "Belgium",
        44 => "United Kingdom",
        7 => "Russia",
        1...31 => "Somewhere", // 1..31 it would not include 31, but 1....31 will include 31
        _ => "unknown"
    };

    println!("Code '{}' corresponds to '{}'.", country_code, country);
}

fn main() {
    match_statement();
}
