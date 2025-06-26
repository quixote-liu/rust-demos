use anyhow::Result;
use brotli::Decompressor;
use std::{
    fs::File,
    io::{copy, BufReader, BufWriter},
    path::Path,
};

fn decompress_br_file(input_path: &Path, output_path: &Path) -> Result<()> {
    // 打开压缩文件
    let compressed_file = File::open(input_path)?;
    let reader = BufReader::new(compressed_file);

    
    // 创建输出文件
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // 创建 Brotli 解压器
    let mut decompressor = Decompressor::new(reader, 4096); // 4KB 缓冲区

    // 执行解压缩
    copy(&mut decompressor, &mut writer)?;

    println!(
        "成功解压缩: {} → {}",
        input_path.display(),
        output_path.display()
    );
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let compressed_file = Path::new("C:\\code\\rust-demos\\src\\file_demo\\br_compress\\data-variables.json.br");
        let output_file = Path::new("C:\\code\\rust-demos\\src\\file_demo\\br_compress\\data-variables.json");
        let res = decompress_br_file(compressed_file, output_file);
        println!("result =  {:?}", res);
    }
}