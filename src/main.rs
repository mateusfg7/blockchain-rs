mod block;

mod blockchain;
use blockchain::Blockchain;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        short = 'd',
        long = "difficulty",
        help = "The number of zeros (difficulty) at the start of hash"
    )]
    difficulty: usize,

    #[arg(
        short = 'b',
        long = "blocks",
        help = "Number of blocks to be generated"
    )]
    blocks: usize,
}

fn main() {
    let cli = Cli::parse();

    let difficulty = cli.difficulty;
    let mut blockchain = Blockchain::new(difficulty);

    let n_of_blocks = cli.blocks;
    let mut curr_n = 0;

    println!("");
    loop {
        if curr_n < n_of_blocks {
            blockchain.add_block();
            curr_n += 1;
        } else {
            break;
        }
    }
}
