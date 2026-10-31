import Advent.Lib
import Std.Data.HashMap
import Std.Data.HashSet

namespace Advent.Ec2026.Day02

/-- Finds a stream of the right length -/
partial def findStream (curStream : List Nat) (bigger: Std.HashMap Nat (Std.HashSet Nat)) (lenNeeded: Nat) : Option List Nat :=
  let rec go : List Nat → Std.HashMap Nat (Std.HashSet Nat) → Nat → Option List Nat
    | curStream, _, 0 => some curStream
    | first :: oldStream, bigger, len =>
      match bigger[first]? with
      | some options =>
        let matching := options.map (go (· :: curStream) bigger (lenNeeded - 1))
        |> filter ( match · with
          | some _ => true
          | none => false)


      | none => none
    | [], _, _ => none
  go curStream bigger lenNeeded

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


end Advent.Ec2026.Day02
