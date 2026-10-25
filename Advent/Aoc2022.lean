import Advent.Aoc2022.Day01
-- import Advent.Aoc2022.Day02
-- import Advent.Aoc2022.Day03
-- import Advent.Aoc2022.Day04
-- import Advent.Aoc2022.Day05
-- import Advent.Aoc2022.Day06
-- import Advent.Aoc2022.Day07
-- import Advent.Aoc2022.Day08
-- import Advent.Aoc2022.Day09
-- import Advent.Aoc2022.Day10
-- import Advent.Aoc2022.Day11
-- import Advent.Aoc2022.Day12
-- import Advent.Aoc2022.Day13
-- import Advent.Aoc2022.Day14
-- import Advent.Aoc2022.Day15
-- import Advent.Aoc2022.Day16
-- import Advent.Aoc2022.Day17
-- import Advent.Aoc2022.Day18
-- import Advent.Aoc2022.Day19
-- import Advent.Aoc2022.Day20
-- import Advent.Aoc2022.Day21
-- import Advent.Aoc2022.Day22
-- import Advent.Aoc2022.Day23
-- import Advent.Aoc2022.Day24
-- import Advent.Aoc2022.Day25

def runDay (args : Args) : Except String String :=
  match args.day with
  | 1 => Advent.Aoc2022.Day01.runPuzzle args
  -- | 2 => Advent.Aoc2022.Day02.runPuzzle args
  -- | 3 => Advent.Aoc2022.Day03.runPuzzle args
  -- | 4 => Advent.Aoc2022.Day04.runPuzzle args
  -- | 5 => Advent.Aoc2022.Day05.runPuzzle args
  -- | 6 => Advent.Aoc2022.Day06.runPuzzle args
  -- | 7 => Advent.Aoc2022.Day07.runPuzzle args
  -- | 8 => Advent.Aoc2022.Day08.runPuzzle args
  -- | 9 => Advent.Aoc2022.Day09.runPuzzle args
  -- | 10 => Advent.Aoc2022.Day10.runPuzzle args
  -- | 11 => Advent.Aoc2022.Day11.runPuzzle args
  -- | 12 => Advent.Aoc2022.Day12.runPuzzle args
  -- | 13 => Advent.Aoc2022.Day13.runPuzzle args
  -- | 14 => Advent.Aoc2022.Day14.runPuzzle args
  -- | 15 => Advent.Aoc2022.Day15.runPuzzle args
  -- | 16 => Advent.Aoc2022.Day16.runPuzzle args
  -- | 17 => Advent.Aoc2022.Day17.runPuzzle args
  -- | 18 => Advent.Aoc2022.Day18.runPuzzle args
  -- | 19 => Advent.Aoc2022.Day19.runPuzzle args
  -- | 20 => Advent.Aoc2022.Day20.runPuzzle args
  -- | 21 => Advent.Aoc2022.Day21.runPuzzle args
  -- | 22 => Advent.Aoc2022.Day22.runPuzzle args
  -- | 23 => Advent.Aoc2022.Day23.runPuzzle args
  -- | 24 => Advent.Aoc2022.Day24.runPuzzle args
  -- | 25 => Advent.Aoc2022.Day25.runPuzzle args
  | _ => .error s!"Day {args.day} is not implemented yet."
