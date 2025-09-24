pub fn create_blockchain(genesis_address: &str) -> Blockchain{
    let db =sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
    let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    let tip_hash;

    if data.is_none(){
        let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
        let block = Block::generate_genesis_block(&coinbase_tx);
        self::update_blocks_tree(&blocks_tree,&block);
        tip_hash = String::from(block.get_hash());
    } else{
        tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
    }

     Blockchain{
        tip_hash: Arc::new(RwLock::new(tip_hash)),
        db,
    }
}

pub fn new_blockchain() -> Blockchain{
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
    let tip_bytes = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap()
    .expect("No existe Blockchain found. Create one firts.");

    Blockchain{
        tip_hash: Arc::new(RwLock::new(tip_hash)),
        db,
    }
}

pub fn get_db(&self) -> &Db {
    &self.db

}

pub fn get_tip_hash(&self) -> String {
    self.tip_hash.read().unwrap().clone()  
}

pub fn set_tip_hash(&self, new_tip_hash: &string){
    let mut tip_hash = self.tip_hash.write().unwrap();
    *tip_hash = String::from(new_tip_hash)
}

pub fn iterator(&self) -> BlockchainIterator{
    BlockchainIterator::new(self.get_tip_hash(),self.db.clone())
}