import Advent.Aoc2025.Day01
-- import Advent.Aoc2025.Day02
-- import Advent.Aoc2025.Day03
-- import Advent.Aoc2025.Day04
-- import Advent.Aoc2025.Day05
-- import Advent.Aoc2025.Day06
-- import Advent.Aoc2025.Day07
-- import Advent.Aoc2025.Day08
-- import Advent.Aoc2025.Day09
-- import Advent.Aoc2025.Day10
-- import Advent.Aoc2025.Day11
-- import Advent.Aoc2025.Day12
-- import Advent.Aoc2025.Day13
-- import Advent.Aoc2025.Day14
-- import Advent.Aoc2025.Day15
-- import Advent.Aoc2025.Day16
-- import Advent.Aoc2025.Day17
-- import Advent.Aoc2025.Day18
-- import Advent.Aoc2025.Day19
-- import Advent.Aoc2025.Day20
-- import Advent.Aoc2025.Day21
-- import Advent.Aoc2025.Day22
-- import Advent.Aoc2025.Day23
-- import Advent.Aoc2025.Day24
-- import Advent.Aoc2025.Day25

namespace Advent.Aoc2025

def runDay (data : List String) (args : Args) : Except String String :=
  match args.day with
  | 1 => Advent.Aoc2025.Day01.runPuzzle data args
  -- | 2 => Advent.Aoc2025.Day02.runPuzzle data args
  -- | 3 => Advent.Aoc2025.Day03.runPuzzle data args
  -- | 4 => Advent.Aoc2025.Day04.runPuzzle data args
  -- | 5 => Advent.Aoc2025.Day05.runPuzzle data args
  -- | 6 => Advent.Aoc2025.Day06.runPuzzle data args
  -- | 7 => Advent.Aoc2025.Day07.runPuzzle data args
  -- | 8 => Advent.Aoc2025.Day08.runPuzzle data args
  -- | 9 => Advent.Aoc2025.Day09.runPuzzle data args
  -- | 10 => Advent.Aoc2025.Day10.runPuzzle data args
  -- | 11 => Advent.Aoc2025.Day11.runPuzzle data args
  -- | 12 => Advent.Aoc2025.Day12.runPuzzle data args
  | _ => .error s!"Day {args.day} is not implemented yet."

end Advent.Aoc2025
