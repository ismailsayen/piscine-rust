use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("player1"), String::from("player2"), 3);
    println!("{:?}", game.read_winner());

    game.update_score("player2");
    // game.update_score("player2");
    // game.update_score("Susana");
    // game.update_score("Susana");
    println!("{:?}", game.read_winner());

    // game.update_score("Mark");
    // // This one will not count because it already 5 games played, the `nb_games`
    // game.update_score("Susana");

    println!("{:?}", game.read_winner());

    println!("{:?}", game.delete());

    // game.read_winner();
    // This will give an error as the game was dropped with `delete` and no longer exists
}