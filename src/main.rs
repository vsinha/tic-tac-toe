use std::{
    ascii::AsciiExt,
    fmt::Display,
    io::{stdin, Error},
    num::ParseIntError,
    ops::Index,
};

const LABELS: [char; 9] = ['0', '1', '2', '3', '4', '5', '6', '7', '8'];

#[derive(Clone, Debug)]
enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => " X ",
                Player::O => " O ",
            }
        )
    }
}

#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Option<Player>>>,
}

impl Default for Board {
    fn default() -> Self {
        let rows = 3;
        let cols = 3;
        Self {
            rows,
            cols,
            cells: vec![vec![None; cols]; rows],
        }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for i in 0..self.cols {
            write!(f, " {}  ", i)?;
        }
        write!(f, "\n")?;

        Ok(for r in 0..self.rows {
            let row = &self.cells[r];
            let s = row
                .iter()
                .map(|x| match x {
                    Some(player) => player.to_string(),
                    None => "   ".to_owned(),
                })
                .collect::<Vec<String>>()
                .join("┃");
            // let char = r.to_string().chars().nth(0).unwrap();
            write!(f, "{} {}\n", LABELS[r], s)?;

            if r < self.rows - 1 {
                write!(f, "  ")?;
                for _ in 0..self.cols - 1 {
                    write!(f, "━━━╋")?;
                }
                write!(f, "━━━\n")?;
            }
        })
    }
}

enum TurnResult<T> {
    Victory(T),
    Continue,
}

impl Board {
    fn evaluate(&self) -> TurnResult<Player> {
        TurnResult::Continue
    }
}

fn main() -> Result<(), ParseIntError> {
    let mut board = Board::default();
    let mut line = String::new();

    let mut player = Player::X;

    loop {
        println!("{}", board);

        print!("{}'s turn. Choose a location: ", player);
        let _input = stdin().read_line(&mut line).unwrap();

        match line.trim().split_once(",") {
            Some((x, y)) => {
                let r = x.parse::<usize>()?;
                let c = y.parse::<usize>()?;

                board.cells[r][c] = Some(player.clone());
            }
            None => todo!(),
        }

        match board.evaluate() {
            TurnResult::Continue => {
                line.clear();

                player = match player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                }
            }
            TurnResult::Victory(player) => {
                println!("Player {} wins!", player);
                return Ok(());
            }
        }
    }
}
