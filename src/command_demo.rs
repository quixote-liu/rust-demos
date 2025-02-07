use std::process::{Command, Output};

use encoding::codec::utf_8::from_utf8;

pub fn exe_cmd() {
    // let res = Command::new("pactl")
    //     .arg("list")
    //     .arg("sinks")
    //     .output();
    // match res {
    //     Ok(output) => {
    //         if !output.status.success() {
    //             let err = output.stderr.as_slice();
    //             let errstr = from_utf8(err).unwrap();
    //             println!("the pactl is not success, error = {}", errstr);
    //             return
    //         }
    //         let res = from_utf8(&output.stdout).unwrap();
    //         // println!("{res}");
    //         parse_volume(res);
    //     },
    //     Err(e) =>{
    //         println!("command error = {:?}", e);
    //     },
    // }
    restore_sh()
}

fn restore_sh() {
    println!("屏幕转角设置完毕，开始重启浏览器");
    if let Err(e) = Command::new("bash").arg("restore.sh").output() {
        println!("重启浏览器失败: {:?}", e);
        return;
    }
    println!("屏幕转角设置完毕，完成重启浏览器");
}

// const FPAD7_SINK_NAME :&str = "alsa_output.platform-cs4344-sound.stereo-fallback"; // 15 arm
// const FPAD7_SINK_NAME :&str = "alsa_output.usb-C-Media_Electronics_Inc._USB_Audio_Device-00.analog-stereo"; // 82
const FPAD7_SINK_NAME :&str = "alsa_output.pci-0000_00_05.0.analog-stereo"; // 15

#[derive(Default, Debug)]
pub struct Sound {
    pub volume: Option<u8>,
    /// 启用声音
    pub enabled: Option<bool>,
}

pub fn get_fpad7_sound() -> Option<Sound> {
    let cmd_res = Command::new("pactl")
            .arg("list")
            .arg("sinks")
            .output();
    if let Ok(res) = cmd_res {
        if !res.status.success() {
            return None
        }
        let mut matched = false;
        let mut volume_msg = String::new();
        let mut mute_msg = String::new();
        for line in from_utf8(&res.stdout).unwrap().lines() {
            let line = line.trim();
            if line == "" { // 每个sink之间存在空行
                if !matched {
                    volume_msg = String::new();
                    mute_msg = String::new();
                    continue
                }
                break
            }
            let mut parts: Vec<&str> = line.splitn(2, "：").collect(); // 系统语言为中文
            if parts.len() != 2 {
                parts = line.splitn(2, ":").collect(); // 系统语言为英文
            }
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim();
                if (key == "Name" || key == "名称") && val == FPAD7_SINK_NAME {
                    matched = true;
                }
                if key == "Volume" || key == "音量" {
                    volume_msg = val.to_string();
                }
                if key == "Mute" || key == "静音" {
                    mute_msg = val.to_string();
                }
            }
        }
        println!("get_fpad7_sound: matched = {}, volume_msg = {}, mute_msg = {}", matched, volume_msg, mute_msg);
        if matched {
            let mut sound = Sound::default();

            // 是否启动音量
            if mute_msg == "no" || mute_msg == "否" {
                sound.enabled = Some(true)
            } else if mute_msg == "yes" || mute_msg == "是" {
                sound.enabled = Some(false)
            } else {
                println!("FPAD7声道使能信息解析失败, mute_msg = [{}]", mute_msg);
            }

            // 音量大小
            let volume_parts: Vec<&str> = volume_msg.split(",").collect();
            if volume_parts.len() > 0 {
                // 如果有多个声道，则取第一个，假设所有声道相等
                let volume_vals: Vec<&str> = volume_parts[0].split("/").collect(); 
                for val in volume_vals {
                    let val = val.trim();
                    if val.ends_with("%") {
                        if let Ok(v) = val.trim_end_matches("%").parse::<u8>() {
                            sound.volume = Some(v);
                        }
                    }
                }
            }
            if sound.volume == None {
                println!("FPAD7声道音量获取失败, volume_msg = {}", volume_msg);
            }

            return Some(sound)
        }
    }
    None
}

pub fn set_fpad7_volumn(volume: u8) -> bool {
    let cmd_res = Command::new("pactl")
        .arg("set-sink-volume")
        .arg(FPAD7_SINK_NAME)
        .arg(format!("{}%", volume))
        .status();
    if let Ok(res) = cmd_res {
        return res.success()
    }
    false
}

pub fn set_fpad7_enable(enable: bool) -> bool {
    let mute_arg = if enable {"0"} else {"1"};
    let cmd_res = Command::new("pactl")
        .arg("set-sink-mute")
        .arg(FPAD7_SINK_NAME)
        .arg(mute_arg)
        .status();
    if let Ok(res) = cmd_res {
        return res.success();
    }
    false
}