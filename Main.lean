import LeanAdventOfCode.Lib

import Cli

open Cli

-- 1. Define the command configuration using the DSL
def myCmd : Cmd := `[Cli|
  my_program VIA runMyCmd;
  DESCRIPTION "A Lean 4 program that parses named options"

  FLAGS:
    verbose, v;      "Enable verbose output logs"
    name, n : String; "The name of the user to greet"

  ARGS:
    inputs : String;  "Trailing positional arguments"
]

-- 2. Define the execution logic handler
def runMyCmd (p : Parsed) : IO UInt32 := do
  -- Check for a boolean flag
  let verbose := p.hasFlag "verbose"
  if verbose then
    IO.println "Verbose mode is ON"

  -- Safely extract and read a typed named option
  if let some nameArg := p.flag? "name" then
    let name := nameArg.as! String
    IO.println s!"Hello, {name}!"
  else
    IO.println "Hello, anonymous user!"

  -- Process trailing positional args
  let input := p.positionalArg! "inputs"
  IO.println s!"Processing input file: {input.as! String}"

  return 0

-- 3. Pass the raw List String to the validator in main
def main (args : List String) : IO UInt32 := do
  myCmd.validate args

import Lean

-- Identical LCG step using UInt32 wrapping arithmetic
def nextLcg (state : UInt32) : UInt32 :=
  state * 1664525 + 1013904223

-- Tail-recursive loop to decrypt bytes efficiently
partial def decryptLoop (bytes : ByteArray) (idx : Nat) (seed : UInt32) (acc : ByteArray) : ByteArray :=
  if idx ≥ bytes.size then
    acc
  else
    let byte := bytes.get! idx
    let nextSeed := nextLcg seed
    let key := (nextSeed &&& 0xFF).toUInt8
    let decryptedByte := byte ^^^ key
    decryptLoop bytes (idx + 1) nextSeed (acc.push decryptedByte)

def readEncryptedFile (path : System.FilePath) (seed : UInt32) : IO String := do
  let encryptedBytes ← IO.FS.readBinFile path
  let decryptedBytes := decryptLoop encryptedBytes 0 seed ByteArray.empty
  
  -- Convert the raw bytes back into a UTF-8 String
  match String.fromUTF8? decryptedBytes with
  | some str => return str
  | none => throw (IO.userError "Invalid UTF-8 data after decryption")

def main : IO Unit := do
  try
    let decrypted ← readEncryptedFile "encrypted.bin" 42
    IO.println s!"Decrypted content:\n{decrypted}"
  catch e =>
    IO.eprintln s!"Error: {e}"
