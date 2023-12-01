use clap::Parser;
use pack::all_packs::get_packs;

mod day;
mod pack;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("aoc23"))]
    pack_name: String,
    #[arg(short, long)]
    day: String,
}

fn main() {
    let args = Args::parse();
    let packs = get_packs();
    if let Some(pack) = packs.get(&args.pack_name) {
        if let Some(day) = pack.days.get(&args.day) {
            day.solve(&|day_name| pack.read_lines(day_name));
        } else {
            println!("Day {} not found in pack {}", args.day, args.pack_name);
        }
    } else {
        println!("Pack {} not found", args.pack_name);
    }
}
