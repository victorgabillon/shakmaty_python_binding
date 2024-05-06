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
    PositionErrorKinds,
};
pub use role::{ByRole, Role};
pub use setup::{Castles, Setup};
pub use square::{File, ParseSquareError, Rank, Square};
pub use types::{CastlingMode, EnPassantMode, Move, Piece, RemainingChecks};
pub use uci::UciMove;

use pyo3::prelude::*;


macro_rules! add_classes {
    ($module:ident, $($class:ty),+) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}

macro_rules! add_functions {
    ($module:ident, $($function:ident),+) => {
        $(
            $module.add_wrapped(wrap_pyfunction!($function))?;
        )+
    };
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
///#[pymodule]
///#[pyo3(name = "shakmaty_python_binding")]
///fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
///    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
///    Ok(())
///}


#[pyclass(module = "shakmaty_python_binding")]
pub struct MyChess {
    chess:Chess
}

#[pymethods]
impl MyChess {
    #[new]
    pub const fn new() -> MyChess {
        MyChess {
            chess: Chess::new()
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Email(subject={}", self.chess.board()))
    }

    fn play(&mut self, to: String) -> PyResult<()> {
        //let move_str = String::from("e2e3");
        let uci = to.parse::<UciMove>().expect("valid uci");
        let m = uci.to_move(&self.chess).expect("legal uci");
        let m2=&Move::Normal {
            role: Role::Pawn,
            from: Square::E2,
            to: Square::E4,
            capture: None,
            promotion: None,
        };
        self.chess.play_unchecked(&m);
        Ok(())
    }

}





#[pymodule]
fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(m, MyChess);
    Ok(())
}