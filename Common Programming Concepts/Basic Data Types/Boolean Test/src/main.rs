fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening: bool = is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
