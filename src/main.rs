use clap::{command, Arg, ArgAction};

use todo::run;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .help("List all of your todos. (Run by default)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("create")
                .long("create")
                .short('c')
                .help("Create a new or clear the current list")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("add")
                .long("add")
                .short('a')
                .help("Add new items")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("finish")
                .long("finish")
                .short('f')
                .help("Finish items")
                .action(ArgAction::SetTrue),
        )

        .get_matches();

    run(matches);
}
