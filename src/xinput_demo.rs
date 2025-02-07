use std::process::Command;
use std::str::from_utf8;

fn xinput_demo() {
    let xinput_list = Command::new("xinput")
        .arg("list")
        .output();
    let device_id: Option<i32> = match xinput_list {
        Ok(output) => {
            from_utf8(&output.stdout).unwrap().lines().find_map(|line| -> Option<i32> {
                // println!("line = {}", line);
                if line.contains("Virtual core XTEST pointer") { 
                    let parts: Vec<&str> = line.split(" ").collect();
                    println!("parts = {:?}", parts.clone());
                    for part in parts {
                        let ids: Vec<&str> = part.trim().split("=").collect();   
                        if ids.len() == 2 && ids[0] == "id" {
                            if let Ok(id) = ids[1].parse::<i32>() {
                                return Some(id);
                            }
                        }
                    }
                }
                None
            })
        },
        Err(e) => {
            println!("矫正屏幕坐标系失败：获取设备ID失败, {:?}", e);
            return
        },
    };
    match device_id {
        Some(id) => {
            println!("device_id = {}", id);
        },
        None => {
            println!("获取设备ID错误");
        },
    }
}

fn ls_demo() {
    let output = Command::new("ls").output();
    match output {
        Err(e) => {
            println!("ls 发生错误 {:?}", e)
        },
        Ok(output) => {
            from_utf8(&output.stdout).unwrap().lines().for_each(|line|
                println!("line = {}", line)
            );
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xinput_demo() {
        xinput_demo();
    }

    #[test]
    fn test_ls_demo() {
        ls_demo()
    }
}