use super::*;
use crate::block::*;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    /// 初始化链
    pub fn new() -> Blockchain {
        Blockchain{
            blocks: vec![Block::new_genesis_block()]
        }
    }

    /// 添加一个区块
    pub fn add_block(&mut self,data:String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash())?;
        self.blocks.push(new_block);
        Ok(())
    }
}