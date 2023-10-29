use color_print::cformat;
use compiler::{error::Result, io};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        std::cmp::Ordering::Less => io::repl(),
        std::cmp::Ordering::Equal => io::script(&args[1]),
        std::cmp::Ordering::Greater => {
            eprintln!(
                "{}",
                cformat!("<y>Usage</>: <u>{}</> <dim><<script>></>", args[0])
            );
            Ok(())
        }
    }
}
