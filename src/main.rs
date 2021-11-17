use std::{
    ascii::AsciiExt,
    fmt::Display,
    io::{stdin, Error},
    num::ParseIntError,
    ops::Index,
};

const LABELS: [char; 9] = ['0', '1', '2', '3', '4', '5', '6', '7', '8'];

#[derive(Clone, Debug)]
enum Mark {
    X,
    O,
    None,
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mark::X => " X ",
                Mark::O => " O ",
                Mark::None => "   ",
            }
        )
    }
}

#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Mark>>,
}

impl Default for Board {
    fn default() -> Self {
        let rows = 3;
        let cols = 3;
        Self {
            rows,
            cols,
            cells: vec![vec![Mark::None; cols]; rows],
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
                .map(|x| x.to_string())
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

fn main() -> Result<(), ParseIntError> {
    let mut board = Board::default();
    let mut line = String::new();

    let mut player = Mark::X;

    loop {
        println!("{}", board);

        print!("{}'s turn. Choose a location: ", player);
        let _input = stdin().read_line(&mut line).unwrap();

        match line.trim().split_once(",") {
            Some((x, y)) => {
                let r = x.parse::<usize>()?;
                let c = y.parse::<usize>()?;

                board.cells[r][c] = player.clone();
            }
            None => todo!(),
        }

        line.clear();

        player = match player {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            _ => todo!(),
        }
    }
}
