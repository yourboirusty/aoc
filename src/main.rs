use clap::Parser;
use pack::all_packs::get_packs;

use crate::day::read_lines_for;

mod day;
mod pack;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pack_name: String,
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let packs = get_packs();
    if let Some(pack) = packs.get(&args.pack_name) {
        if let Some(day) = pack.days.get(&args.day) {
            let lines = read_lines_for(day);
            print!("{}", day.solve(&lines))
        }
    }
}
