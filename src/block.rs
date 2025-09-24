extern crate bincode;
use bincode::{serialize, deserialize};
use crate::{ProoFOFWork, Transaction};
use crate::proof_of_work::ProoFOFWork;
use crate::transaction::Transaction;
use serde::{deserialize, serialize};
use sled::IV;


pub struct Block{
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<String>,
    nonce: u64,
    height: usize,
}

impl Block{

    pub fn deserialize(bytes: &[u8])-> Block{
        bincode::deserialize(bytes).unwrap()
    }

    pub fn serialize(&self)-> Vec<u8>{
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn new_block(pre_block_hash: String, transactions: &[Transactions], height: usize)-> 
    Block{
    let mut block = Block {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        hash: String::new(),
        transactions: transactions.to_vec(),
        nonce: 0,
        height,
    };

    let pow =  ProoFOFWork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();
    block.nonce = nonce;
    block.hash = hash;
    return block
}

pub fn get_transactions(&self) -> &[Transaction] {
    self.transactions.as_slice()
}
pub fn get_transactions(&self)-> &[Transaction]{
    self.transactions.as_slice()
}

pub fn get_pre_block_hash(&self)->String{
    self.pre_block_hash.clone()
}

pub fn get_hash(&self)-> &str{
    self.hash.as_str()
}

pub fn get_hash_bytes(&self)-> Vec<u8>{
    self.hash.as_bytes().to_vec()
}

pub fn get_timestamp(&self)->i64{
    self.timestamp
}

pub fn get_height(&self)-> usize{
    self.height
}

pub fn hash_transactions(&self)-> Vec<u8>{
    let mut txhashs = vec![];
    for transaction in &self.transactions{
        txhashs.extend(transaction.get_id());
    }
    crate::sha256_digest(txhashs.as_slice())
}

pub fn generate_genesis_block(transaction: &Transaction)-> Block{
    let transactions = vec![transaction.clone()];
    return Block::new_block(String::from("None"), &transactions, 0);
}

}

impl From<Block> for IVec{
    fn from(b:Block) -> Self{
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}