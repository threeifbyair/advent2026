import Advent.Lib

import Cli

open Cli

-- 1. Define the command configuration using the DSL
def adventCmd : Cmd := `[Cli|
  advent VIA runAdventCmd;
  DESCRIPTION "Advent of Code in Lean 4"

  FLAGS:
    verbose, v;        "Enable verbose output logs"
    part_two, p;       "Enable part two logic"
    part_three, q;     "Enable part three logic"
    encrypted, e;      "Input is encrypted and needs decryption"
    argint, a;         "An integer argument"
    seed, s : UInt32;  "The seed for the LCG"
    event, E : String; "The event name"
    day, d : Nat;      "The day of the event"
    input, i : String; "The input file path"
]

-- 2. Define the execution logic handler
def runAdventCmd (p : Parsed) : IO UInt32 := do
  let args ← parseArgs p
  -- FIXME: open and optionally decrypt the input file here
  let result := runEvent args
  match result with
  | .ok msg => do
    IO.println msg
  | .error msg => do
    IO.eprintln s!"Error: {msg}"
    return 1
  return 0

-- 3. Pass the raw List String to the validator in main
def main (args : List String) : IO UInt32 := do
  adventCmd.validate args
