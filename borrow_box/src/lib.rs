#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        Self {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        println!("{:?}", self);
        if self.p1.1 >= self.nb_games / 2 + 1 {
            return Some(&self.p1);
        } else if self.nb_games / 2 + 1 <= self.p2.1 {
            return Some(&self.p2);
        }
        None
    }

    pub fn update_score(&mut self, user_name: &str) {
        if (self.nb_games == 5 && (self.p1.1 == 3 || self.p2.1 == 3))
            || (self.nb_games == 11 && (self.p1.1 == 6 || self.p2.1 == 6))
        {
            return;
        }
        if self.p1.0 == user_name.to_string() {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name.to_string() {
            self.p2.1 += 1;
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {:?}", self.id)
    }
}
