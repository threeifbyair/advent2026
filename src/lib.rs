// Enumerates all combinations of events and days, passing them to a callback macro.
// This is adapted from the Git repo maneatingape/advent-of-code-rust, which is licensed under the MIT license.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#[macro_export]
macro_rules! puzzles {
    ($callback:ident) => {
        $callback! {
            aoc2022 "Assist the Elves on their annual jungle expedition."
                day01 /*day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25*/,

            //aoc2023 "Restore global snow production with an island hopping odyssey."
            //    day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
            //    day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            //aoc2024 "Locate the Chief Historian in time for the big Christmas sleigh launch."
            //    day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
            //    day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            ec2025 "A Tale of Ducks and Dragons"
                day01 day02 /*day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20*/,

            aoc2025 "Finish the North Pole decorations in time for Christmas."
                day01 /*day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12*/

            //ec2026 "A Tale of Ducks and Dragons"
            //    day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
            //    day13 day14 day15 day16 day17 day18 day19 day20,

            //aoc2026 "Finish the North Pole decorations in time for Christmas."
            //    day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
        }
    };
}

/// Declares a public module for each year documented with its description and containing a
/// sub-module for each day.
///
/// Two interesting things:
/// * Module declaration is possible via macro.
/// * Rust allows the day sub-module declarations directly without needing an intermediate `mod.rs`
///   in each year folder.
#[macro_export]
macro_rules! library {
    ($($event:ident $description:literal $($day:ident)*),*) => {
        $(pub mod $event {
            #![doc = concat!("# ", $description)]
            $(pub mod $day;)*
        })*
    };
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    grid decrypt parse
);

puzzles!(library);

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Advent of Code and Everybody Codes")]
pub struct Args {
    /// Enable verbose output logs
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    pub part_two: bool,

    #[arg(short = 'q', long, default_value_t = false)]
    pub part_three: bool,

    #[arg(short, long, default_value_t = false)]
    pub encrypted: bool,

    /// An integer argument.
    #[arg(short, long, default_value_t = 0)]
    pub argint: i32,

    /// Encryption seed.
    #[arg(short, long, default_value_t = std::env::var("AOC_SEED").unwrap_or_else(|_| "0".to_string()).parse().unwrap_or(0))]
    pub seed: i32,

    /// Event.
    #[arg(short = 'E', long, default_value_t = std::env::var("AOC_EVENT").unwrap_or_else(|_| "".to_string()))]
    pub event: String,

    /// Day.
    #[arg(short, long, default_value_t = std::env::var("AOC_DAY").unwrap_or_else(|_| "0".to_string()).parse().unwrap_or(0))]
    pub day: u32,

    /// Input file name (default: "-" for stdin)
    #[arg(short, long, default_value_t = "-".to_string())]
    pub input: String,
}
