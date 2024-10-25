class MyChess(object):
    # no doc
    def copy(self, *args, **kwargs): # real signature unknown
        pass







    def legal_moves(self, *args, **kwargs): # real signature unknown
        pass

    def number_of_pieces_on_the_board(self, *args, **kwargs): # real signature unknown
        pass

    def piece_at(self, *args, **kwargs): # real signature unknown
        pass

    def piece_map(self, *args, **kwargs): # real signature unknown
        pass



    def play_modifications(self, *args, **kwargs): # real signature unknown
        pass



    def __init__(self, *args, **kwargs): # real signature unknown
        pass

    @staticmethod # known case of __new__
    def __new__(*args, **kwargs): # real signature unknown
        """ Create and return a new object.  See help(type) for accurate signature. """
        pass

    def __str__(self, *args, **kwargs): # real signature unknown
        """ Return str(self). """
        pass
        
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
                  
                               
    def play(self, move:MyMove)-> None: 
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
        
        


