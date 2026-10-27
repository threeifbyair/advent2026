import Advent.Lib
import Advent.Aoc2022
import Advent.Aoc2025
import Advent.Ec2025
import Advent.Ec2026

import Cli

open Cli

def runEvent (data : List String) (args : Args) : Except String String :=
  match args.event with
  | "aoc2022" => Advent.Aoc2022.runDay data args
  | "aoc2025" => Advent.Aoc2025.runDay data args
  | "ec2025"  => Advent.Ec2025.runDay data args
  | "ec2026"  => Advent.Ec2026.runDay data args
  | _         => Except.error s!"Unknown event: {args.event}"


partial def readStreamToEnd (stream : IO.FS.Stream) (chunkSize : USize := 4096) : IO ByteArray := do
  let rec loop (acc : Array UInt8) : IO ByteArray := do
    let chunk ← stream.read chunkSize
    if chunk.isEmpty then
      pure ⟨acc⟩
    else
      loop (acc ++ chunk.data)
  loop #[]

-- 1. Define the execution logic handler
def runAdventCmd (p : Parsed) : IO UInt32 := do
  let args ← parseArgs p
  let inputDataRaw ← if args.input = "-" then
    let stdinStream ← IO.getStdin
    readStreamToEnd stdinStream
  else
    IO.FS.readBinFile args.input

  let inputDataSemiRaw := if args.encrypted then
    readEncryptedData inputDataRaw args.seed
  else
    inputDataRaw

  let inputData ← match String.fromUTF8? inputDataSemiRaw with
  | some str => pure str
  | none => throw (IO.userError "Invalid UTF-8 data after decryption")

  let data :=
  inputData.splitOn "\n"
    |>.map (·.toRawSubstring.trim.toString)

  let result := runEvent data args
  match result with
  | .ok msg => do
    IO.println msg
  | .error msg => do
    IO.eprintln s!"Error: {msg}"
    return 1
  return 0

-- 2. Define the command configuration using the DSL
def adventCmd : Cmd := `[Cli|
  advent VIA runAdventCmd;
  "Advent of Code in Lean 4"

  FLAGS:
    v, verbose;         "Enable verbose output logs"
    p, part_two;        "Enable part two logic"
    q, part_three;      "Enable part three logic"
    e, encrypted;       "Input is encrypted and needs decryption"
    a, argint;          "An integer argument"
    s, seed : Nat;      "The seed for the LCG"
    E, event : String;  "The event name"
    d, day : Nat;       "The day of the event"
    i, input : String;  "The input file path"
]

-- 3. Pass the raw List String to the validator in main
def main (args : List String) : IO UInt32 := do
  adventCmd.validate args
