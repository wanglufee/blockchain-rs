mod block;
mod blockchain;

pub type Result<T> = std::result::Result<T, failure::Error>;

use blockchain::*;
fn main() {
    let mut bc = Blockchain::new();
    let _ = bc.add_block(String::from("手动添加一个区块"));
    let _ = bc.add_block(String::from("工作996真的太烦了"));
    print!("{:#?}",bc);
}
