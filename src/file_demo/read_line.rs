use std::io::Write;
use std::fs;

fn read_line_demo() {
    let file_path = "/home/lcs/worksplace/rust-demos/src/file_demo/test.conf";
    if let Ok(content) = std::fs::read_to_string(file_path) {
        println!("====START===");
        let mut is_filter = false;
        let new_content: String = content.lines()
            .into_iter()
            .filter_map(|line| {
                if line.contains("Section \"Monitor\"") {
                    is_filter = true;
                }
                if is_filter && line.contains("EndSection") {
                    is_filter = false;
                    return None;
                }
                if !is_filter {
                    let mut line = line.to_string();
                    line.push('\n');
                    return Some(line);
                }
                None
            })
            .collect();
            // .filter(|line|{
            //     if line.contains("Section \"Monitor\"") {
            //         is_filter = false;
            //     }
            //     if !is_filter && line.contains("EndSection") {
            //         is_filter = true;
            //         return false;
            //     }
            //     is_filter
            // })
        println!("===new_content = {}", new_content.clone());
        if let Ok(mut file) = std::fs::OpenOptions::new().write(true).truncate(true).open(file_path) {
            file.write(new_content.as_bytes()).unwrap();
        }
    }
    println!("====END===");
}

fn get_rotate_angle() -> u16 {
    // 通过读取配置文件获取角度
    let conf_path = "/home/lcs/worksplace/rust-demos/src/file_demo/rotate.conf";
    if let Ok(content) = fs::read_to_string(conf_path) {
        if let Some(angle) = content.lines().into_iter().find_map(|line| -> Option<u16> {
            let line = line.trim();
            let mut res: Option<u16> = None;
            if line.contains("Option \"Rotate\"") {
                if line.contains("right") {
                    res = Some(90);
                } else if line.contains("inverted") {
                    res = Some(180);
                } else if line.contains("left") {
                    res = Some(270);
                } else if line.contains("normal") {
                    res = Some(0);
                }
            }
            res
        }) {
            return angle;
        };
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo() {
        read_line_demo();
    }

    #[test]
    fn test_get_rotate_angle() {
        let res = get_rotate_angle();
        println!("res = {}", res);
    }
}