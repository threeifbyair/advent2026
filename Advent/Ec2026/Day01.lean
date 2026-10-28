import Advent.Lib

namespace Advent.Ec2026.Day01

/-- Sums each blank-line-separated group of calorie counts. -/
partial def groupSums (lines : List String) : List Nat :=
  let rec go : List String → Nat → List Nat → List Nat
    | [], current, acc => if current > 0 then acc ++ [current] else acc
    | "" :: rest, current, acc => go rest 0 (acc ++ [current])
    | line :: rest, current, acc => go rest (current + line.toNat?.getD 0) acc
  go lines 0 []

def partOneSquare (value : Nat) : Nat :=
  value * value

def partTwoSquare (value : Nat) : Nat :=
  ((value + 3) * (value + 3)) - ((value + 1) * (value + 1))

def partThreeSquare (value : Nat) : Nat :=
  (value * 2 + 1) * (value * 2 + 1) - 1

def calcHighest (size: Nat) (args: Args): Nat :=
  let squareFunction := if args.partThree then
    partThreeSquare
    else if args.partTwo then
    partTwoSquare
    else
    partOneSquare

  let possibleValues: List Nat := List.range 10000
    |> List.map squareFunction
    |> List.filter (· >= size)

  match possibleValues.min? with
    | some x => x - size
    | _ => 0


def runPuzzle  (data : List String) (args : Args): Except String String :=

  let result: Nat := data[0]!.splitOn ","
                          |>.filterMap String.toNat?
                          |>.map (calcHighest · args)
                          |>.sum

  .ok s!"Needed: {result}"


end Advent.Ec2026.Day01
