use std::collections::HashMap;
use std::vec::Vec;

//use pyo3::{pymodule,pymethods,pyclass, PyResult,Python};
//use pyo3::types::PyModule;
use pyo3::prelude::*;

use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{
    Board, CastlingMode, CastlingSide, Chess, Color, EnPassantMode, FromSetup, Move, MoveList,
    Piece, Position, Role, Square,
};

mod push_with_modification;
pub use push_with_modification::do_move_record_modifications;

use shakmaty_syzygy::Tablebase;

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
    chess_move: Move,
    uci: String,
}

impl PartialEq for MyMove {
    fn eq(&self, other: &Self) -> bool {
        self.chess_move == other.chess_move
    }
}

#[pymethods]
impl MyMove {
    #[new]
    pub fn new(uci: &str, my_chess: &MyChess) -> MyMove {
        let chess_move = UciMove::from_ascii(uci.as_bytes())
            .expect("REASON")
            .to_move(&my_chess.chess)
            .expect("REASON");
        MyMove {
            chess_move: chess_move,
            uci: uci.to_string(),
        }
    }

    fn is_zeroing(&self) -> PyResult<bool> {
        Ok(self.chess_move.is_zeroing())
    }

    fn uci(&self) -> PyResult<String> {
        Ok(self.uci.clone())
    }
}

impl MyMove {
    pub fn new_rust(move_: Move, uci: String) -> MyMove {
        MyMove {
            chess_move: move_,
            uci: uci,
        }
    }
}

/////////////// Board state

pub struct MyBoardStateRust {
    pub __str__: String,
    pub ply: u32,
    pub turn: u8,
    pub is_game_over: bool,
    pub legal_moves: Vec<MyMove>,
    pub number_of_pieces_on_the_board: u32,
    pub has_queenside_castling_rights: bool,
    pub has_kingside_castling_rights: bool,
    pub piece_map: HashMap<u64, (u32, bool)>,
    pub is_attacked_white: bool,
    pub is_attacked_black: bool,
    pub occupied: u64,
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub promoted: u64,
    pub ep_square: i32,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub castling_rights: u64,
    pub white: u64,
    pub black: u64,
    pub result: String,
}

impl MyBoardStateRust {
    pub fn new(
        __str__: &str,
        ply: u32,
        turn: u8,
        is_game_over: bool,
        legal_moves: Vec<MyMove>,
        number_of_pieces_on_the_board: u32,
        has_queenside_castling_rights: bool,
        has_kingside_castling_rights: bool,
        piece_map: HashMap<u64, (u32, bool)>,
        is_attacked_white: bool,
        is_attacked_black: bool,
        occupied: u64,
        pawns: u64,
        knights: u64,
        bishops: u64,
        rooks: u64,
        queens: u64,
        kings: u64,
        promoted: u64,
        ep_square: i32,
        halfmove_clock: u32,
        fullmove_number: u32,
        castling_rights: u64,
        white: u64,
        black: u64,
        result: &str,
    ) -> MyBoardStateRust {
        MyBoardStateRust {
            __str__: (*__str__).to_string(),
            ply: ply,
            turn: turn,
            is_game_over: is_game_over,
            legal_moves: legal_moves,
            number_of_pieces_on_the_board: number_of_pieces_on_the_board,
            has_queenside_castling_rights: has_queenside_castling_rights,
            has_kingside_castling_rights: has_kingside_castling_rights,
            piece_map: piece_map,
            is_attacked_white: is_attacked_white,
            is_attacked_black: is_attacked_black,
            occupied: occupied,
            pawns: pawns,
            knights: knights,
            bishops: bishops,
            rooks: rooks,
            queens: queens,
            kings: kings,
            promoted: promoted,
            ep_square: ep_square,
            halfmove_clock: halfmove_clock,
            fullmove_number: fullmove_number,
            castling_rights: castling_rights,
            white: white,
            black: black,
            result: (*result).to_string(),
        }
    }
}

#[pyclass]

pub struct MyBoardState2 {
    pub __str__: PyResult<String>,
    pub ply: PyResult<u32>,
    pub turn: PyResult<u8>,
    pub is_game_over: PyResult<bool>,
    pub legal_moves: PyResult<Vec<MyMove>>,
    pub number_of_pieces_on_the_board: PyResult<u32>,
    pub has_queenside_castling_rights: PyResult<bool>,
    pub has_kingside_castling_rights: PyResult<bool>,
    pub piece_map: PyResult<HashMap<u64, (u32, bool)>>,
    pub is_attacked_white: PyResult<bool>,
    pub is_attacked_black: PyResult<bool>,
    pub occupied: PyResult<u64>,
    pub pawns: PyResult<u64>,
    pub knights: PyResult<u64>,
    pub bishops: PyResult<u64>,
    pub rooks: PyResult<u64>,
    pub queens: PyResult<u64>,
    pub kings: PyResult<u64>,
    pub promoted: PyResult<u64>,
    pub ep_square: PyResult<i32>,
    pub halfmove_clock: PyResult<u32>,
    pub fullmove_number: PyResult<u32>,
    pub castling_rights: PyResult<u64>,
    pub white: PyResult<u64>,
    pub black: PyResult<u64>,
    pub result: PyResult<String>,
}

#[pyclass]
pub struct MyBoardState {
    board_state_rust: MyBoardStateRust,
}

/////////////// TABLE BASE

#[pyclass]
pub struct MyTableBase {
    tables: Tablebase<shakmaty::Chess>,
}

#[pymethods]
impl MyTableBase {
    #[new]
    pub fn new(_path_to_table: &str) -> MyTableBase {
        let mut tables_ = Tablebase::new();
        let _ = tables_.add_directory(_path_to_table);
        MyTableBase { tables: tables_ }
    }

    fn probe_wdl(&self, my_chess: &MyChess) -> PyResult<i8> {
        match self.tables.probe_wdl_after_zeroing(&my_chess.chess) {
            Ok(wdl) => Ok(i8::from(wdl)),
            Err(err) => panic!("probe wdl: {}", err),
        }
    }

    fn probe_dtz(&self, my_chess: &MyChess) -> PyResult<i32> {
        match self.tables.probe_dtz(&my_chess.chess) {
            Ok(dtz) => Ok(i32::from(dtz.ignore_rounding())),
            Err(err) => panic!("probe dtz: {}", err),
        }
    }
}

/////////////// CHESSBOARD

#[pyclass]
pub struct MyChess {
    chess: Chess,
}

#[pymethods]
impl MyChess {
    #[new]
    pub fn new(_fen_start: &str) -> MyChess {
        MyChess {
            chess: setup_fen(_fen_start),
        }
    }

    fn play(&mut self, my_move_: &MyMove) -> PyResult<()> {
        self.chess.play_unchecked(&my_move_.chess_move);
        Ok(())
    }

    fn play_and_return(&mut self, my_move_: &MyMove) -> PyResult<(String, u32, u8, bool)> {
        self.chess.play_unchecked(&my_move_.chess_move);

        let a = u32::from(self.chess.fullmoves()) - 1;
        let b = self.chess.turn() as u32;
        let c = 2 * a + 1 - b;

        Ok((
            format!("board fen: {}", self.chess.board()),
            c,
            self.chess.turn() as u8,
            self.chess.is_game_over(),
        ))
    }


    fn play_and_return_modifications(&mut self, my_move_: &MyMove) -> PyResult<(String, u32, u8, bool)> {
        self.chess.play_unchecked_return_modifications(&my_move_.chess_move);

        let a = u32::from(self.chess.fullmoves()) - 1;
        let b = self.chess.turn() as u32;
        let c = 2 * a + 1 - b;

        Ok((
            format!("board fen: {}", self.chess.board()),
            c,
            self.chess.turn() as u8,
            self.chess.is_game_over(),
        ))
    }

    fn play_and_return_o(
        &mut self,
        my_move_: &MyMove,
    ) -> PyResult<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u8, i32, u64)> {
        self.chess.play_unchecked(&my_move_.chess_move);

        let cas = u64::from(self.chess.castles().castling_rights());
        // let  is_game_over = self.chess.is_game_over();

        let pawns = u64::from(self.chess.board().by_role(Role::Pawn));
        let knights = u64::from(self.chess.board().by_role(Role::Knight));
        let bishops = u64::from(self.chess.board().by_role(Role::Bishop));
        let rooks = u64::from(self.chess.board().by_role(Role::Rook));
        let queens = u64::from(self.chess.board().by_role(Role::Queen));
        let kings = u64::from(self.chess.board().by_role(Role::King));
        let white = u64::from(self.chess.board().by_color(Color::White));
        let black = u64::from(self.chess.board().by_color(Color::Black));
        let turn = self.chess.turn() as u8;

        let a = self.chess.maybe_ep_square();
        let ep_square;

        match a {
            Some(x) => ep_square = i32::from(x),
            None => ep_square = i32::from(-1),
        }
        let promoted = u64::from(self.chess.promoted());

        Ok((
            cas, pawns, knights, bishops, rooks, queens, kings, white, black, turn, ep_square,
            promoted,
        ))
    }

    fn copy(&mut self) -> PyResult<MyChess> {
        Ok(MyChess {
            chess: self.chess.clone(),
        })
    }

    fn piece_at(&mut self, square_int: u32) -> PyResult<Option<(bool, u32)>> {
        let square = Square::new(square_int);
        let a: Option<Piece> = self.chess.board().piece_at(square);
        if a.is_none() {
            Ok(None)
        } else {
            Ok(Some((a.unwrap().color as u32 != 0, a.unwrap().role as u32)))
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("board fen: {}", self.chess.board()))
    }

    fn ply(&mut self) -> PyResult<u32> {
        let a = u32::from(self.chess.fullmoves()) - 1;
        let b = self.chess.turn() as u32;
        let c = 2 * a + 1 - b;
        Ok(c)
    }

    fn turn(&mut self) -> PyResult<u8> {
        Ok(self.chess.turn() as u8)
    }

    fn is_game_over(&mut self) -> PyResult<bool> {
        Ok(self.chess.is_game_over())
    }


    fn legal_moves(&mut self) -> PyResult<Vec<MyMove>> {
        let move_list: MoveList = self.chess.legal_moves();
        let mut legal_moves_my = Vec::new();
        for a_move in move_list {
            let uci = a_move.to_uci(CastlingMode::Standard).to_string();
            legal_moves_my.push(MyMove::new_rust(a_move, uci));
        }
        Ok(legal_moves_my)
    }

    fn number_of_pieces_on_the_board(&mut self) -> PyResult<u32> {
        Ok(self.chess.board().occupied().0.count_ones())
    }

    fn fen(&mut self) -> PyResult<String> {
        let _a = self.chess.halfmoves().to_string();
        let _b = self.chess.board().to_string();
        let _d = self.chess.castles().castling_rights().0.to_string();
        let _e = Fen::from_position(self.chess.clone(), EnPassantMode::Legal).to_string();
        let _c = format!("{}-{}-{}", _b, _a, _d);

        Ok(_e)
    }

    fn has_queenside_castling_rights(&mut self, color: bool) -> PyResult<bool> {
        Ok(self
            .chess
            .castles()
            .has(Color::from_white(color), CastlingSide::QueenSide))
    }

    fn has_kingside_castling_rights(&mut self, color: bool) -> PyResult<bool> {
        Ok(self
            .chess
            .castles()
            .has(Color::from_white(color), CastlingSide::KingSide))
    }

    fn piece_map(&mut self) -> PyResult<HashMap<u64, (u32, bool)>> {
        let mut iter = <Board as Clone>::clone(&self.chess.board()).into_iter();
        //let mut iter = board_fen.iter().copied().peekable();
        let mut dict = HashMap::new();

        while let Some(ch) = iter.next() {
            dict.insert(ch.0.into(), (ch.1.role as u32, ch.1.color as u8 != 0));
        }
        Ok(dict)
    }

    fn is_attacked(&mut self, color: bool) -> PyResult<bool> {
        let mut res: bool = false;
        //let mut all_square = ()
        //while rest == False   {
        let real_color: Color = Color::from_white(color);
        for (square, piece) in self.chess.board().clone().into_iter() {
            //println!("{} {:?}  days", square, piece);
            if piece.color == real_color {
                let opposed_color = real_color.other();
                let bitb = self.chess.board().clone().attacks_to(
                    square,
                    opposed_color,
                    self.chess.board().clone().occupied(),
                );
                let a = u64::from(bitb) != 0;
                //println!("89 {:?} {}  bitb", bitb,a);
                res = res || a;
            }
        }
        Ok(res)
        //}
    }

    #[inline]
    pub fn occupied(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().occupied()))
    }

    #[inline]
    pub fn pawns(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Pawn)))
    }

    #[inline]
    pub fn knights(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Knight)))
    }

    #[inline]
    pub fn bishops(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Bishop)))
    }

    #[inline]
    pub fn rooks(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Rook)))
    }

    #[inline]
    pub fn queens(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::Queen)))
    }

    #[inline]
    pub fn kings(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_role(Role::King)))
    }

    pub fn promoted(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.promoted()))
    }

    #[inline]
    pub fn ep_square(&self) -> PyResult<i32> {
        let a = self.chess.maybe_ep_square();

        match a {
            Some(x) => return Ok(i32::from(x)),
            None => return Ok(i32::from(-1)),
        }
    }

    pub fn halfmove_clock(&self) -> PyResult<u32> {
        Ok(u32::from(self.chess.halfmoves()))
    }

    pub fn fullmove_number(&self) -> PyResult<u32> {
        Ok(u32::from(self.chess.fullmoves()))
    }

    #[inline]
    pub fn castling_rights(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.castles().castling_rights()))
    }

    #[inline]
    pub fn white(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_color(Color::White)))
    }

    #[inline]
    pub fn black(&self) -> PyResult<u64> {
        Ok(u64::from(self.chess.board().by_color(Color::Black)))
    }

    fn result(&self) -> PyResult<String> {
        let a = self.chess.outcome();

        match a {
            Some(x) => return Ok(x.as_str().to_owned()),
            None => return Ok("*".to_owned()),
        }
    }
}

#[pymodule]
fn shakmaty_python_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    add_classes!(m, MyMove);
    add_classes!(m, MyChess);
    add_classes!(m, MyTableBase);
    //add_classes!(m, MyBoardState);
    Ok(())
}
