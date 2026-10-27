import Advent.Ec2026.Day01
-- import Advent.Ec2026.Day02
-- import Advent.Ec2026.Day03
-- import Advent.Ec2026.Day04
-- import Advent.Ec2026.Day05
-- import Advent.Ec2026.Day06
-- import Advent.Ec2026.Day07
-- import Advent.Ec2026.Day08
-- import Advent.Ec2026.Day09
-- import Advent.Ec2026.Day10
-- import Advent.Ec2026.Day11
-- import Advent.Ec2026.Day12
-- import Advent.Ec2026.Day13
-- import Advent.Ec2026.Day14
-- import Advent.Ec2026.Day15
-- import Advent.Ec2026.Day16
-- import Advent.Ec2026.Day17
-- import Advent.Ec2026.Day18
-- import Advent.Ec2026.Day19
-- import Advent.Ec2026.Day20

namespace Advent.Ec2026

def runDay (data : List String) (args : Args) : Except String String :=
  match args.day with
  | 1 => Advent.Ec2026.Day01.runPuzzle data args
  -- | 2 => Advent.Ec2026.Day02.runPuzzle data args
  -- | 3 => Advent.Ec2026.Day03.runPuzzle data args
  -- | 4 => Advent.Ec2026.Day04.runPuzzle data args
  -- | 5 => Advent.Ec2026.Day05.runPuzzle data args
  -- | 6 => Advent.Ec2026.Day06.runPuzzle data args
  -- | 7 => Advent.Ec2026.Day07.runPuzzle data args
  -- | 8 => Advent.Ec2026.Day08.runPuzzle data args
  -- | 9 => Advent.Ec2026.Day09.runPuzzle data args
  -- | 10 => Advent.Ec2026.Day10.runPuzzle data args
  -- | 11 => Advent.Ec2026.Day11.runPuzzle data args
  -- | 12 => Advent.Ec2026.Day12.runPuzzle data args
  -- | 13 => Advent.Ec2026.Day13.runPuzzle data args
  -- | 14 => Advent.Ec2026.Day14.runPuzzle data args
  -- | 15 => Advent.Ec2026.Day15.runPuzzle data args
  -- | 16 => Advent.Ec2026.Day16.runPuzzle data args
  -- | 17 => Advent.Ec2026.Day17.runPuzzle data args
  -- | 18 => Advent.Ec2026.Day18.runPuzzle data args
  -- | 19 => Advent.Ec2026.Day19.runPuzzle data args
  -- | 20 => Advent.Ec2026.Day20.runPuzzle data args
  | _ => .error s!"Day {args.day} is not implemented yet."

end Advent.Ec2026
