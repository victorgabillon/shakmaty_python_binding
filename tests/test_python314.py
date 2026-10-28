"""Exercise the native move constructor and compare real play with python-chess."""

import random
import unittest

import chess
import shakmaty_python_binding as native


class NativeChessTests(unittest.TestCase):
    """Keep native argument extraction and chess behavior independently checked."""

    def assert_position_equal(self, expected, actual):
        self.assertEqual(actual.fen(), expected.fen())
        self.assertEqual(
            sorted(move.uci() for move in actual.legal_moves()),
            sorted(move.uci() for move in expected.legal_moves),
        )

    def test_special_move_construction(self):
        cases = [
            (chess.STARTING_FEN, "e2e4"),
            ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1g1"),
            ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1c1"),
            ("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1", "e5d6"),
        ] + [("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", f"a7a8{piece}")
             for piece in "qrbn"]
        for fen, uci in cases:
            with self.subTest(fen=fen, uci=uci):
                expected = chess.Board(fen)
                actual = native.MyChess(fen)
                move = native.MyMove(uci, actual)
                self.assertEqual(move.uci(), uci)
                expected.push_uci(uci)
                actual.play(move)
                self.assert_position_equal(expected, actual)
                self.assert_position_equal(expected, actual.copy())

    def test_500_seeded_plies(self):
        rng = random.Random(314)
        expected = chess.Board()
        actual = native.MyChess(expected.fen())
        for index in range(500):
            if expected.is_game_over():
                expected = chess.Board()
                actual = native.MyChess(expected.fen())
            self.assert_position_equal(expected, actual)
            uci = rng.choice(sorted(move.uci() for move in expected.legal_moves))
            move = native.MyMove(uci, actual)
            expected.push_uci(uci)
            actual.play(move)
            with self.subTest(ply=index):
                self.assert_position_equal(expected, actual)


if __name__ == "__main__":
    unittest.main()
