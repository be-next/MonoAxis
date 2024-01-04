use clap::Parser;
use std::fs;
use mono_axis::core::cellular_automaton_builder::CellularAutomatonBuilder;

/// Launch a mono-axis cellular automaton.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct MonoAxis {
    /// Path to the configuration file.
    #[clap(short, long)]
    config: String,

    /// Number of steps to perform.
    /// If not specified, the automaton will run indefinitely.
    /// If specified, the automaton will run for the specified number of steps.
    #[clap(short, long)]
    steps: Option<usize>,
}

fn main() {
    let args = MonoAxis::parse();

    if let Err(e) = fs::metadata(&args.config) {
        eprintln!("Error : configuration file '{}' doesn't exist. Error : {}", args.config, e);
        return;
    }

    let step =  args.steps.unwrap_or(10);

    //"examples/example_01/configuration.json"
    let cab = CellularAutomatonBuilder::new();
    let mut ca = cab.build(&args.config).unwrap();

    ca.print();

    for _ in 0..step {
        ca.step();
        ca.print();
    }
}