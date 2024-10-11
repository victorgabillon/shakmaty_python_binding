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
pub use fen::Epd;
use pyo3::prelude::*;

use std::collections::HashSet;
use std::collections::HashMap;
use std::borrow::BorrowMut;
 use std::borrow::Borrow;

use crate::{
    setup::{EnPassant},
};

use core::{
    num::NonZeroU32,
};

macro_rules! add_classes {
    ($module:ident, $($class:ty),+) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}






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
        let a = u32::from(self.chess.fullmoves()) -1;
        let b = self.chess.turn() as u32;
        let c = 2*a+1-b;
        Ok(c )
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

    fn legal_moves(&mut self) -> PyResult<HashSet<String>> {
        let move_list :MoveList = self.chess.legal_moves();
        let mut uci_legal_moves = HashSet::new();
        for a_move in move_list {
            uci_legal_moves.insert(a_move.to_uci(CastlingMode::Standard).to_string());
            }
        Ok(uci_legal_moves)
    }


    fn number_of_pieces_on_the_board(&mut self)-> PyResult<u32> {
        Ok(self.chess.board().occupied().0.count_ones())
        }

    fn fen(&mut self)-> PyResult<String> {
        let _a = self.chess.halfmoves().to_string();
        let _b = self.chess.board().to_string();
        let _d = self.chess.castles().castling_rights().0.to_string();
	let _e = Fen::from_position(self.chess.clone(), EnPassantMode::Legal).to_string();
        let _c = format!("{}-{}-{}", _b, _a,_d);
        
        Ok(_e)
        }


    fn piece_at(&mut self, square_int:u32)-> PyResult<Option<(bool,u32)>> {
        let square = Square::new(square_int);
        let a: Option<Piece> = self.chess.board().piece_at(square);
        if a.is_none() {
        Ok(None)
            } else{
                        Ok(Some((a.unwrap().color as u32 != 0,a.unwrap().role as u32)))
                }
        }



    fn has_queenside_castling_rights(&mut self, color: bool)-> PyResult<bool> {
        Ok(self.chess.castles().has(Color::from_white(color),CastlingSide::QueenSide))
        }

    fn has_kingside_castling_rights(&mut self, color: bool)-> PyResult<bool> {
        Ok(self.chess.castles().has(Color::from_white(color),CastlingSide::KingSide))
        }

    fn piece_map(&mut self)-> PyResult<HashMap<u64, (u32,bool)>> {
        let mut iter = <Board as Clone>::clone(&self.chess.board()).into_iter();
        //let mut iter = board_fen.iter().copied().peekable();
        let mut dict = HashMap::new();

        while let Some(ch) = iter.next() {
            dict.insert(ch.0.into(),(ch.1.role as u32,ch.1.color as u8 != 0));
        }
        Ok(dict)
        }



    fn is_attacked(
            &mut self,
            color:bool
    ) -> PyResult<bool> {
        let mut res :bool = false;
        //let mut all_square = ()
        //while rest == False   {
        let real_color: Color = Color::from_white(color);
	for (square,piece) in self.chess.board().clone().into_iter() {
	  //println!("{} {:?}  days", square, piece);
	  if piece.color == real_color {
		  let opposed_color = real_color.other();
		  let mut bitb = self.chess.board().clone().attacks_to( square, opposed_color, self.chess.board().clone().occupied());
		  let a = u64::from(bitb) !=0;
		  //println!("89 {:?} {}  bitb", bitb,a);
		  res = res || a;
	  }
	}
	Ok(res)
        //}
	

    }
     //   all_squares_of_color = chess.SquareSet()
     //  for piece_type in [1, 2, 3, 4, 5, 6]:
     //       new_squares = self.board.pieces(piece_type=piece_type, color=a_color)
     //       all_squares_of_color = all_squares_of_color.union(new_squares)
     //   all_attackers = chess.SquareSet()
     //   for square in all_squares_of_color:
     //       new_attackers = self.board.attackers(not a_color, square)
     //       all_attackers = all_attackers.union(new_attackers)
     //   return bool(all_attackers)
        
        
    fn play_modifications(&mut self, to: String) -> PyResult<()> {
        //let uci = to.parse::<UciMove>().expect("valid uci");
        //let m = uci.to_move(&self.chess).expect("legal uci");
        //let a = self.chess.ep_square(EnPassantMode::Legal);
        //let mut epsq :Option<EnPassant>;
        //if a.is_none(){
         //                           epsq = None;

           // }else{
             //           epsq = Some(EnPassant(a.expect("REASON")));

               // }

        let uci = to.parse::<UciMove>().expect("valid uci");
        let m = uci.to_move(&self.chess).expect("legal uci");
        self.chess.play_unchecked_modifications(&m);
        Ok(())

    }

}





#[pymodule]
fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(m, MyChess);
    Ok(())
}
