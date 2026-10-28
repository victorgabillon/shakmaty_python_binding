class MyChess(object):



    def __init__(self, _fen_start:str) -> None:
       ...


    def piece_at(self, square:int) -> tuple[bool,int]|None:
       ...


    def piece_map(self) -> dict[int,tuple[int,bool]]:
       ...

    def is_attacked(
            self,
            a_color: chess.Color
    ) -> bool:
       ...

    def result(self) -> str:
        ...

    def promoted(self) -> int:
        ...

    def ep_square(self) -> int:
        ...


    def castling_rights(self) -> int:
        ...

    #returns a bitboard
    def occupied(self) -> int:
        ...

    #returns a bitboard
    def pawns(self) -> int:
       ...

    #returns a bitboard
    def kings(self) -> int:
       ...


    #returns a bitboard
    def queens(self) -> int:
       ...

    #returns a bitboard
    def bishops(self) -> int:
       ...

    #returns a bitboard
    def rooks(self) -> int:
       ...

    #returns a bitboard
    def knights(self) -> int:
       ...

    #returns a bitboard
    def black(self) -> int:
       ...

    #returns a bitboard
    def white(self) -> int:
       ...

    def ply(self) -> int:
       ...

    def turn(self) -> int:
       ...

    def is_game_over(self) -> bool:
       ...


    def fen(self) -> str:
       ...

    def has_kingside_castling_rights(self, color:bool)-> bool:
        ...

    def has_queenside_castling_rights(self, color:bool)-> bool:
        ...

    def fullmove_number(self) -> int:
       ...

    def halfmove_clock(self) -> int:
       ...

    def copy(self) -> MyChess:
       ...

    def play(self, move: MyMove)-> None:
        ...

    def play_and_return_o(self, my_move_: MyMove) -> tuple[int,int,int,int,int,int,int,int,int,int,int,int]:
        ...

    def play_and_return_modifications(self, my_move_: MyMove)-> tuple[tuple[int,int,int,int,int,int,int,int,int,int,int,int], set[tuple[int,int,int]],set[tuple[int,int,int]]]  :
        ...

    def number_of_pieces_on_the_board(self)-> int:
        ...

    def legal_moves(self) -> list[MyMove]:
       ...

class MyMove(object):


    def __init__(self, uci:str, my_chess:MyChess) -> None:
       ...

    def is_zeroing(self)->bool:
        ...

    def uci(self)->str:
        ...


class MyTableBase(object):


    def __init__(self, _path_to_table:str) -> None:
       ...

    def probe_wdl(self,my_chess:MyChess)->int:
        ...

    def probe_dtz(self,my_chess:MyChess)->int:
        ...





