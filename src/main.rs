use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::Count)
        )
        .get_matches();

    let verbose = matches.get_count("verbose");
    println!("Verbosity level at {}", verbose);
}
