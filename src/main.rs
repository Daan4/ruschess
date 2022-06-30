mod position;

fn main() {
    let pos = position::Position::new(1, 2);
    println!("{}", pos);
    let pos = position::Position::new(0, 2);
    println!("{}", pos);
}
