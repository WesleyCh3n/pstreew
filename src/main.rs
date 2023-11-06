use clap::Parser;
use pstreew::PrintConfig;

mod pstreew;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// show pid
    #[arg(short = 'p', long, action)]
    show_pid: bool,
    /// specify pid
    #[arg(short = 'P', long)]
    pid: Option<u32>,
    /// show parnet process
    #[arg(short = 's', long, action)]
    parent: bool,
    /// sort by pid
    #[arg(short = 'n', long, action)]
    sort: bool,
    /// hide arguments.
    #[arg(short = 'd', long, action, default_value_t = false)]
    hide_arg: bool,
    /// show process full path
    #[arg(short = 'l', long, action, default_value_t = false)]
    show_full_path: bool,
}

fn main() {
    let _ = colored::control::set_virtual_terminal(true);
    let args = Args::parse();
    let cfg = PrintConfig::new()
        .show_pid(args.show_pid)
        .sort(args.sort)
        .show_arg(!args.hide_arg)
        .show_path(args.show_full_path)
        .build();
    if let Some(pid) = args.pid {
        if let Some(node) = pstreew::find_by_pid(pid) {
            if args.parent {
                pstreew::print(pstreew::add_parent(node), &cfg);
            } else {
                pstreew::print(node, &cfg);
            }
        }
        return;
    }

    let trees = pstreew::list_all();
    for tree in trees {
        pstreew::print(tree, &cfg);
    }
}
