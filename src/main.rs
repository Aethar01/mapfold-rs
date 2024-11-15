use clap::Parser;
use log::{info, warn, error};
use env_logger;
mod fold;
 
#[derive(Parser)]
struct Args {
    /// Optional argument in the format `res/mod`. Defaults to 0/0 if not provided.
    #[arg(short, long, num_args = 1, value_delimiter = '/')]
    res_mod: Vec<i32>,

    /// Dimensions for foldings in the format `nxm`
    #[arg(short, long, required = true, num_args = 1, value_delimiter = 'x')]
    dimensions: Vec<i32>,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}


fn main() {
    let  args = Args::parse();
    if args.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    info!("Verbose mode enabled");

    let (res, mod_value) = if args.res_mod.len() == 2 {
        info!("Using res/mod: {}/{}", args.res_mod[0], args.res_mod[1]);
        (args.res_mod[0], args.res_mod[1])
    } else {
        warn!("No or invalid res/mod provided; defaulting to 0/0");
        (0, 0)
    };

    if args.dimensions.iter().any(|&x| x <= 0) {
        error!("Invalid dimensions: {:?}", args.dimensions);
    } else {
        info!("Using dimensions: {:?}", args.dimensions);
    }

    info!("Folding...");
    let mut folder = fold::Folding::new();
    folder.get_fold_count(&args.dimensions, true, res, mod_value);
    println!("Fold count: {}", folder.fold_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_count_values() {
        #[derive(Debug)]
        struct DimensionAndExpected {
            dimensions: Vec<i32>,
            expected: Vec<u64>,
        }

        impl DimensionAndExpected {
            fn new() -> Self {
                Self {
                    dimensions: Vec::new(),
                    expected: Vec::new(),
                }
            }

            fn push(&mut self, new_dimension: i32, new_expected: u64) {
                self.dimensions.push(new_dimension);
                self.expected.push(new_expected);
            }
        } 

        // let known_values: Vec<i64> = vec![2, 8, 60, 320, 1980, 10512, 60788, 320896, 1787904, 9381840, 51081844, 266680992, 1429703548, 7432424160, 39409195740, 204150606976, 1073644675448, 5545305620064];
        let known_values: Vec<i64> = vec![2, 8, 60, 320, 1980, 10512, 60788];
        let mut test_cases = DimensionAndExpected::new();
        for (i, j) in known_values.iter().enumerate() {
            test_cases.push(i as i32 + 1, *j as u64);
        }
        for (expected, dimensions) in test_cases.expected.iter().zip(test_cases.dimensions.iter()) {
            let twod_dimensions = vec![2, *dimensions];
            let mut folder = fold::Folding::new();
            assert_eq!(folder.get_fold_count(&twod_dimensions, true, 0, 0), *expected);
        }
    }
}
