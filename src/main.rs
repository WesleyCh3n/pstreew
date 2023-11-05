use clap::Parser;

mod pstreew;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long, action)]
    show_pid: bool,
    #[arg(short = 'P', long)]
    pid: Option<u32>,
    #[arg(short = 's', long, action)]
    parent: bool,
    #[arg(short = 'n', long, action)]
    sort: bool,
}

fn main() {
    let args = Args::parse();
    if let Some(pid) = args.pid {
        if let Some(node) = pstreew::find_by_pid(pid) {
            if args.parent {
                pstreew::print(
                    pstreew::add_parent(node),
                    args.show_pid,
                    args.sort,
                );
            } else {
                pstreew::print(node, args.show_pid, args.sort);
            }
        }
        return;
    }

    let trees = pstreew::list_all();
    for tree in trees {
        pstreew::print(tree, args.show_pid, args.sort);
    }
}
