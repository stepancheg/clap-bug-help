use clap::Parser;

/// Help of command.
///
/// When this line is added, each option is printed multiline.
#[derive(clap::Parser)]
struct MyOpts {
    /// Help of foo.
    #[clap(long)]
    foo: String,
    /// Help of bar.
    #[clap(long)]
    bar: String,
}

fn main() {
    MyOpts::parse();
    println!("Hello, world!");
}
