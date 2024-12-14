#[warn(dead_code)]
use std::time::{UNIX_EPOCH, SystemTime};

use sha2::{Digest, Sha256};
#[derive(Debug, Clone)]
struct Block {
   index: u64,
   data: String,
   previous_hash: String,
   timestamp: u64,
   hash: String,
}

impl Block {

    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp =  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let data_to_hash = format!("{}{}{}{}", index, data, previous_hash, timestamp);
        let mut hasher = Sha256::new();

        hasher.update(data_to_hash.as_bytes());

        let hash = format!("{:x}",hasher.finalize());

        Block {
            index,
            data,
            previous_hash,
            timestamp,
            hash,
        }
    }

    fn new_genesis_block() -> Block {
        let index = 0;
        let previous_hash = String::from("");
        let timestamp = 0;
        let data = String::from("Genesis Block");
        let data_to_hash = format!("{}{}{}{}", index, data, previous_hash, timestamp);
        let timestamp =  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash.as_bytes());
        let hash = format!("{:x}",hasher.finalize());
        Block {
            index,
            data,
            previous_hash,
            timestamp,
            hash,
        }
    }

    fn get_index(&self) -> u64 {
        self.index
    }
    fn get_data(&self) -> String {
        self.data.clone()
    }
    fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }
    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    fn get_hash(&self) -> String {
        self.hash.clone()
    }

}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>
}
impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new()
        }
    }

    fn add_block(&mut self, data: String)  {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u64;
        let block = Block::new(index, data, previous_hash.to_string());
        self.chain.push(block);
        println!("{:?}", &self.chain.last().unwrap().previous_hash)
    }

    fn add_genesis_block(&mut self) {
        let genesis_block = Block::new_genesis_block();
        self.chain.push(genesis_block);
    }

    fn print_chain(&self){
        for(_i,block) in self.chain.iter().enumerate(){
            println!("BlockIndex: {}",block.get_index());
            println!("BlockData: {}",block.get_data());
            println!("BlockPreviousHash: {}",block.get_previous_hash());
            println!("BlockTimestamp: {}",block.get_timestamp());
            println!("BlockHash: {}",block.get_hash());
            println!("--------------------------------------------------------------------------------");
        }
    }

    fn is_valid(&self) -> bool {
        
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.get_previous_hash() != previous.get_hash() {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_genesis_block();
    blockchain.add_block(String::from("Block 1"));
    blockchain.add_block(String::from("Block 2"));
    blockchain.add_block(String::from("Block 3"));
    blockchain.print_chain();
    println!("Is Valid: {}",blockchain.is_valid());
}
