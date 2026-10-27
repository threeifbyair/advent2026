import Cli

structure Args where
  verbose     : Bool
  partTwo     : Bool
  partThree   : Bool
  encrypted   : Bool
  argInt      : Int
  seed        : UInt32
  event       : String
  day         : Nat
  input       : String

def seedFromEnv : IO UInt32 := do
  match ← IO.getEnv "AOC_SEED" with
  | some s => return (s.toNat?.getD 0).toUInt32
  | none => return 0

def eventFromEnv : IO String := do
  match ← IO.getEnv "AOC_EVENT" with
  | some s => return s
  | none => return ""

def dayFromEnv : IO Nat := do
  match ← IO.getEnv "AOC_DAY" with
  | some s => return (s.toNat?.getD 0)
  | none => return 0


def parseArgs (p : Cli.Parsed) : IO Args := do
  let seed ← match p.flag? "seed" with
    | some f => pure (f.as! Nat).toUInt32
    | none => seedFromEnv
  let event ← match p.flag? "event" with
    | some f => pure (f.as! String)
    | none => eventFromEnv
  let day ← match p.flag? "day" with
    | some f => pure (f.as! Nat)
    | none => dayFromEnv

  return {
    verbose     := p.hasFlag "verbose"
    partTwo     := p.hasFlag "part_two"
    partThree   := p.hasFlag "part_three"
    encrypted   := p.hasFlag "encrypted"
    argInt      := p.flag? "argint" |>.map (·.as! Int) |>.getD 0
    seed        := seed
    event       := event
    day         := day
    input       := p.flag? "input" |>.map (·.as! String) |>.getD "-" }

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

def readEncryptedData (encryptedBytes : ByteArray) (seed : UInt32) : ByteArray :=
  decryptLoop encryptedBytes 0 seed ByteArray.empty
