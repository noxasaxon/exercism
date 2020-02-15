pub fn raindrops(n: u32) -> String {
    let mut result = String::from("");

    if n % 3 == 0 {
        result = result + "Pling";
    }
    if n % 5 == 0 {
        result = result + "Plang";
    }
    if n % 7 == 0 {
        result = result + "Plong";
    }
    if result.len() < 1 {
        return n.to_string();
    }
    
    result
}
