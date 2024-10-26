class MyChess(object):

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

    def fullmove_clock(self) -> int: 
       ...
       
    def halfmove_clock(self) -> int: 
       ...
     
    def copy(self) -> MyChess: 
       ...             
                               
    def play(self, move:MyMove)-> None: 
        ...
        
    def number_of_pieces_on_the_board(self)-> int: 
        ...
           
    def legal_moves(self) -> set[MyMove]:
       ...           
     
class MyMove(object):


    @staticmethod # known case of __new__
    def __new__(*args, **kwargs): # real signature unknown
        """ Create and return a new object.  See help(type) for accurate signature. """
        pass
        
    def is_zeroing(self)->bool:
        ...

    def uci(self)->str:
        ...
        
        


