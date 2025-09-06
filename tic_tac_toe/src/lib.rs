pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table){
        return "player O won".to_string();
    }
    if horizontal('X', table) || vertical('X', table) || diagonals('X', table){
        return "player X won".to_string();
    }

    "tie".to_string()
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][0] == player && table[2][0] == player {
        return true;
    }
    if table[0][1] == player && table[1][1] == player && table[2][1] == player {
        return true;
    }
    if table[2][0] == player && table[2][1] == player && table[2][2] == player {
        return true;
    }
    false
}
pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let mut counter: u32 = 0;
    for row in table {
        for ch in row {
            if ch == player {
                counter += 1;
            }
        }
        if counter == 3 {
            return true;
        } else {
            counter = 0;
        }
    }
    false
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }
    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    false
}
