// The Puzzle struct and run! macro are adapted from the Git repo maneatingape/advent-of-code-rust, which is licensed under the MIT license.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::fs;

use advent::Args;
use advent::puzzles;
use advent::util::decrypt::*;

use clap::Parser;
use std::io::{Error, Read}; // for Args::parse

pub struct Puzzle {
    event: String,
    day: u32,
    wrapper: fn(&[&str], &Args) -> Result<String, Error>,
}

macro_rules! run {
    ($($event:ident $description:literal $($day:ident)*),*) => {
        [$($(
            Puzzle {
                event: stringify!($event).into(),
                day: stringify!($day).trim_start_matches("day").parse().unwrap(),
                wrapper: |data, args| {
                    use advent::$event::$day::*;

                    puzzle(data, args)
                }
            }
        ,)*)*]
    };
}

fn main() {
    let args = Args::parse();

    let inbytes = if args.input == "-" {
        let mut buffer = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    } else {
        fs::read(&args.input).expect("Failed to read input file")
    };

    let indata = if args.encrypted {
        read_encrypted_file(inbytes, args.seed as u32).expect("Failed to decrypt input")
    } else {
        String::from_utf8(inbytes).expect("Failed to decode input")
    };

    // Build list of all solutions, then filter to run only selected solutions.
    let found_puzzles: Vec<Puzzle> = puzzles!(run)
        .into_iter()
        .filter(|s| args.event == s.event)
        .filter(|s| args.day == s.day)
        .collect();

    if found_puzzles.is_empty() {
        eprintln!(
            "No matching puzzles found for event '{}' and day '{}'",
            args.event, args.day
        );
        std::process::exit(1);
    }

    for puzzle in found_puzzles {
        let result = (puzzle.wrapper)(&indata.lines().collect::<Vec<_>>(), &args);
        println!("{} {}: {}", puzzle.event, puzzle.day, result.unwrap());
    }
}
