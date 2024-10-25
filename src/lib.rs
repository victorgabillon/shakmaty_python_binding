use std::collections::HashSet;
use std::collections::HashMap;

//use pyo3::{pymodule,pymethods,pyclass, PyResult,Python};
//use pyo3::types::PyModule;
use pyo3::prelude::*;

use shakmaty::{Position, FromSetup,CastlingMode,Chess,MoveList,EnPassantMode,Square,Piece,Color,CastlingSide,Board,Role,Move};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;

use shakmaty_syzygy::{Tablebase};





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
            .into_position::<T>(CastlingMode::Standard)
            .expect("legal position")
}



///////////// MOVE

#[derive(Eq, Hash)]
#[pyclass]
pub struct MyMove {
    chess_move:Move
}

impl PartialEq for MyMove {
    fn eq(&self, other: &Self) -> bool {
        self.chess_move == other.chess_move
    }
}

#[pymethods]
impl MyMove {

    #[new]
    pub fn new(uci:&str, my_chess:&MyChess) -> MyMove {
        MyMove{
        	chess_move:UciMove::from_ascii(uci.as_bytes()).expect("REASON").to_move(&my_chess.chess).expect("REASON")
        }
    }
    
    fn is_zeroing(&self) -> PyResult<bool> {
        Ok(self.chess_move.is_zeroing())
    }


    fn uci(&self) -> PyResult<String> {
        Ok(self.chess_move.to_uci(CastlingMode::Standard).to_string())
    }
}


impl MyMove {

    pub fn new_rust(move_:Move) -> MyMove {
    
        MyMove {
            chess_move: move_
        }
    }

}
/////////////// TABLE BASE


#[pyclass]
pub struct MyTableBase {
    tables:Tablebase<shakmaty::Chess>
}


#[pymethods]
impl MyTableBase {
    #[new]
    pub fn new(_path_to_table:&str) -> MyTableBase {
    
    	let mut tables_ = Tablebase::new();
        let _ = tables_.add_directory(_path_to_table);
        MyTableBase {
            tables: tables_
        }
    }
   
    fn probe_wdl(&self, my_chess : &MyChess) -> PyResult<i8> {
	
	match self.tables.probe_wdl_after_zeroing(&my_chess.chess) {
            Ok(wdl) => Ok(i8::from(wdl)),
            Err(err) => panic!("probe wdl: {}", err),
        }

    }
    
    fn probe_dtz(&self, my_chess : &MyChess) -> PyResult<i32> {
	
	match self.tables.probe_dtz(&my_chess.chess) {
            Ok(dtz) => Ok(i32::from(dtz.ignore_rounding())),
            Err(err) => panic!("probe dtz: {}", err),
        }

    }
}



/////////////// CHESSBOARD

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

    fn play(&mut self, my_move_: &MyMove) -> PyResult<()> {
        self.chess.play_unchecked(&my_move_.chess_move);
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

    fn legal_moves(&mut self) -> PyResult<HashSet<MyMove>> {
        let move_list :MoveList = self.chess.legal_moves();
        let mut legal_moves_my = HashSet::new();
        for a_move in move_list {
            legal_moves_my.insert(MyMove::new_rust(a_move));
            }
        Ok(legal_moves_my)
    }


    //fn legal_moves_2(&mut self) -> PyResult<MoveList> {
    //    let move_list :MoveList = self.chess.legal_moves();
    //    Ok(move_list)
    //}

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
		  let bitb = self.chess.board().clone().attacks_to( square, opposed_color, self.chess.board().clone().occupied());
		  let a = u64::from(bitb) !=0;
		  //println!("89 {:?} {}  bitb", bitb,a);
		  res = res || a;
	  }
	}
	Ok(res)
        //}
	

    }
    
    

    #[inline]
    pub fn occupied(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().occupied().clone()))
    }

    #[inline]
    pub  fn pawns(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Pawn)))
    }

    #[inline]
    pub  fn knights(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Knight)))
    }

    #[inline]
    pub  fn bishops(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Bishop)))
    }

    #[inline]
    pub  fn rooks(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Rook)))
    }

    #[inline]
    pub  fn queens(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Queen)))
    }

    #[inline]
    pub  fn kings(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::King)))
    }
    
    pub  fn promoted(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.promoted()))
    }
    
        #[inline]
    pub  fn ep_square(&self) -> PyResult<i32> {
       	let a= self.chess.maybe_ep_square();

        match a {
    		Some(x) => {return Ok(i32::from(x)) }
    		None => {return Ok(i32::from(-1))}
}
        
    }
    
     pub  fn halfmove_clock(&self) -> PyResult<u32> {
        Ok(u32::from(self.chess.halfmoves()))
        
    }
    
    pub  fn fullmove_clock(&self) -> PyResult<u32> {
        Ok(u32::from(self.chess.fullmoves()))
        
    }
    
        #[inline]
    pub  fn castling_rights(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.castles().castling_rights()))
    }
    

    #[inline]
    pub  fn white(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_color(Color::White)))
    }

    #[inline]
    pub  fn black(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_color(Color::Black)))
    }
    
   fn result(&self) -> PyResult<String> {

   	let a= self.chess.outcome();

        match a {
    		Some(x) => {return Ok(x.as_str().to_owned())}
    		None => {return Ok("*".to_owned())}
}
    } 
    
        
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
        let _m = uci.to_move(&self.chess).expect("legal uci");
        //self.chess.play_unchecked_modifications(&m);
        Ok(())

    }

}





#[pymodule]
fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(m, MyMove);
    add_classes!(m, MyChess);
    add_classes!(m, MyTableBase);

    Ok(())
}
