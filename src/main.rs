use clap::Parser;
mod foldrename;
 
#[derive(Parser)]
struct Args {
    /// Optional argument in the format `res/mod`. Defaults to 0/0 if not provided.
    #[arg(short, long, num_args = 1, value_delimiter = '/')]
    res_mod: Vec<i32>,

    /// Dimensions for foldings in the format `n m`
    #[arg(short, long, required = true, num_args = 2)]
    dimensions: Vec<i32>,
}


fn main() {
    let  args = Args::parse();

    let (res, mod_value) = if args.res_mod.len() == 2 {
        println!("Using res/mod: {}/{}", args.res_mod[0], args.res_mod[1]);
        (args.res_mod[0], args.res_mod[1])
    } else {
        println!("No or invalid res/mod provided; defaulting to 0/0");
        (0, 0)
    };

    if args.dimensions.iter().any(|&x| x <= 0) {
        println!("Invalid dimensions provided");
        return;
    } else {
        println!("Using dimensions: {:?}", args.dimensions);
    }

    println!("Folding...");
    let mut folder = foldrename::Folding::new();
    folder.foldings(args.dimensions, true, res, mod_value);
    println!("{}", folder.next());
}
