mod block;

mod blockchain;
use blockchain::Blockchain;

fn main() {
    let difficulty = 3;
    let mut blockchain = Blockchain::new(difficulty);

    let n_of_blocks = 3;
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
