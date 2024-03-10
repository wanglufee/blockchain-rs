use super::*;
use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::SystemTime;

/// 区块结构体
#[derive(Debug)]
pub struct Block {
    timestamp: u128,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {
    /// 设置hash值
    fn set_hash(&mut self) -> Result<()> {
        // 计算时间戳
        self.timestamp = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)?
                            .as_millis();
        // 计算hash值
        let content = (self.timestamp,self.data.clone(),self.prev_block_hash.clone());
        let bytes = serialize(&content)?;
        let mut hasher = Sha256::new();
        hasher.input(&bytes);
        self.hash = hasher.result_str();
        Ok(())
    }

    /// 获取区块hash值
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    /// 新建区块
    pub fn new_block(data: String,prev_block_hash: String) -> Result<Block> {
        let mut block = Block{
            timestamp: 0,
            data,
            prev_block_hash,
            hash: String::new(),
        };
        let _ = block.set_hash();
        Ok(block)
    }

    /// 生成创世区块
    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("创世区块"), String::new()).unwrap()
    }

}