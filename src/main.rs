mod position;
mod piece;
mod player;
mod game;

fn main() {
    let chess = game::Game::new("Daan".to_owned(), "cthulhe".to_owned());
    println!("{}\n{}", chess.white, chess.black)
}
