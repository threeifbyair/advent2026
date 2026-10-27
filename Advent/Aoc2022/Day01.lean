import Advent.Lib

namespace Advent.Aoc2022.Day01

/-- Sums each blank-line-separated group of calorie counts. -/
partial def groupSums (lines : List String) : List Nat :=
  let rec go : List String → Nat → List Nat → List Nat
    | [], current, acc => if current > 0 then acc ++ [current] else acc
    | "" :: rest, current, acc => go rest 0 (acc ++ [current])
    | line :: rest, current, acc => go rest (current + line.toNat?.getD 0) acc
  go lines 0 []

def runPuzzle  (data : List String) (args : Args): Except String String :=
  let totals := (groupSums data).mergeSort (· ≥ ·)
  if args.partTwo then
    match totals with
    | a :: b :: c :: _ => .ok s!"Top 3: {a + b + c}"
    | _ => .error "expected at least 3 elf groups"
  else
    match totals with
    | a :: _ => .ok s!"Max: {a}"
    | [] => .error "no elf groups found"

end Advent.Aoc2022.Day01
