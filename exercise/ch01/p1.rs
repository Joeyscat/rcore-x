// 显示当前目录下的文件名

use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.file_name());
    }

    Ok(())
}
