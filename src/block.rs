use super::*;
use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::SystemTime;

const TARGET_HEXS: usize = 4;

/// 区块结构体
#[derive(Debug)]
pub struct Block {
    timestamp: u128,
    data: String,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
}

impl Block {
    /// 获取区块hash值
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    /// 新建区块
    pub fn new_block(data: String,prev_block_hash: String) -> Result<Block> {
        // 计算时间戳
        let timestamp = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)?
                            .as_millis();
        let mut block = Block{
            timestamp,
            data,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
        };
        let _ = block.run_proof_of_work()?;
        Ok(block)
    }

    /// 工作量证明
    fn run_proof_of_work(&mut self) -> Result<()> {
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data);
        self.hash = hasher.result_str();
        Ok(())
    }

    /// 校验hash值是否合法
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data);
        let hash = hasher.result_str();
        let mut vec1: Vec<u8> = Vec::new();
        vec1.resize(TARGET_HEXS, '0' as u8);
        Ok(hash[0..TARGET_HEXS] == String::from_utf8(vec1)?)
    }


    /// 准备数据
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (self.timestamp,self.data.clone(),
                                                self.prev_block_hash.clone(),
                                                self.nonce);
        let bytes = serialize(&content)?;
        Ok(bytes)
    }

    /// 生成创世区块
    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("创世区块"), String::new()).unwrap()
    }

}