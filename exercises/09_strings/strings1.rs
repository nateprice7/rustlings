// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    let answer2 = current_favorite_colorstr();
    println!("My current favorite color is {}", answer);
    println!("My current favorite color is {}", answer2);
}

fn current_favorite_color() -> &'static str {
    "blue"
}
fn current_favorite_colorstr() -> String {
    "blue".to_string()
}
