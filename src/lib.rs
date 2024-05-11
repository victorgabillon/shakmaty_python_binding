#[macro_use]
mod util;
mod bootstrap;
mod castling_side;
mod color;
mod magics;
mod movelist;
mod perft;
mod position;
mod role;
mod setup;
mod square;
mod types;

pub mod attacks;
pub mod bitboard;
pub mod board;
pub mod fen;
pub mod san;
pub mod uci;
pub mod zobrist;


#[cfg(feature = "variant")]
pub mod variant;

pub use bitboard::Bitboard;
pub use board::Board;
pub use castling_side::{ByCastlingSide, CastlingSide};
pub use color::{ByColor, Color, ParseColorError};
pub use movelist::MoveList;
pub use perft::perft;
pub use position::{
    Chess, FromSetup, Outcome, ParseOutcomeError, PlayError, Position, PositionError,
    PositionErrorKinds
};
pub use role::{ByRole, Role};
pub use setup::{Castles, Setup};
pub use square::{File, ParseSquareError, Rank, Square};
pub use types::{CastlingMode, EnPassantMode, Move, Piece, RemainingChecks};
pub use uci::UciMove;
pub use fen::Fen;
use pyo3::prelude::*;


macro_rules! add_classes {
    ($module:ident, $($class:ty),+) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}





/// A Python module implemented in Rust.
///#[pymodule]
///#[pyo3(name = "shakmaty_python_binding")]
///fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
///    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
///    Ok(())
///}

fn setup_fen<T: Position + FromSetup>(fen: &str) -> T {
        fen.parse::<Fen>()
            .expect("valid fen")
            .into_position::<T>(CastlingMode::Chess960)
            .expect("legal position")
}





#[pyclass]
pub struct MyChess {
    chess:Chess
}


#[pymethods]
impl MyChess {
    #[new]
    pub fn new(_fen_start:&str) -> MyChess {
        MyChess {
            chess: setup_fen(_fen_start)
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("board fen: {}", self.chess.board()))
    }

    fn play(&mut self, to: String) -> PyResult<()> {
        let uci = to.parse::<UciMove>().expect("valid uci");
        let m = uci.to_move(&self.chess).expect("legal uci");
        self.chess.play_unchecked(&m);
        Ok(())
    }

    fn ply(&mut self) -> PyResult<u32> {
        Ok(self.chess.halfmoves())
    }

    fn turn(&mut self) -> PyResult<u8> {
        Ok(self.chess.turn() as u8)
    }

    fn is_game_over(&mut self) -> PyResult<bool> {
        Ok(self.chess.is_game_over())
    }

    fn copy(&mut self) -> PyResult<MyChess> {
        Ok(MyChess {
            chess: self.chess.clone()
        })
    }
}





#[pymodule]
fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(m, MyChess);
    Ok(())
}