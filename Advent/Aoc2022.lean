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

namespace Advent.Aoc2022

def runDay (data : List String) (args : Args) : Except String String :=
  match args.day with
  | 1 => Advent.Aoc2022.Day01.runPuzzle data args
  -- | 2 => Advent.Aoc2022.Day02.runPuzzle data args
  -- | 3 => Advent.Aoc2022.Day03.runPuzzle data args
  -- | 4 => Advent.Aoc2022.Day04.runPuzzle data args
  -- | 5 => Advent.Aoc2022.Day05.runPuzzle data args
  -- | 6 => Advent.Aoc2022.Day06.runPuzzle data args
  -- | 7 => Advent.Aoc2022.Day07.runPuzzle data args
  -- | 8 => Advent.Aoc2022.Day08.runPuzzle data args
  -- | 9 => Advent.Aoc2022.Day09.runPuzzle data args
  -- | 10 => Advent.Aoc2022.Day10.runPuzzle data args
  -- | 11 => Advent.Aoc2022.Day11.runPuzzle data args
  -- | 12 => Advent.Aoc2022.Day12.runPuzzle data args
  -- | 13 => Advent.Aoc2022.Day13.runPuzzle data args
  -- | 14 => Advent.Aoc2022.Day14.runPuzzle data args
  -- | 15 => Advent.Aoc2022.Day15.runPuzzle data args
  -- | 16 => Advent.Aoc2022.Day16.runPuzzle data args
  -- | 17 => Advent.Aoc2022.Day17.runPuzzle data args
  -- | 18 => Advent.Aoc2022.Day18.runPuzzle data args
  -- | 19 => Advent.Aoc2022.Day19.runPuzzle data args
  -- | 20 => Advent.Aoc2022.Day20.runPuzzle data args
  -- | 21 => Advent.Aoc2022.Day21.runPuzzle data args
  -- | 22 => Advent.Aoc2022.Day22.runPuzzle data args
  -- | 23 => Advent.Aoc2022.Day23.runPuzzle data args
  -- | 24 => Advent.Aoc2022.Day24.runPuzzle data args
  -- | 25 => Advent.Aoc2022.Day25.runPuzzle data args
  | _ => .error s!"Day {args.day} is not implemented yet."

end Advent.Aoc2022
