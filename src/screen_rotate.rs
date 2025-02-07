pub fn output_message() {
    let content = "Screen 0: minimum 320 x 200, current 1280 x 800, maximum 16384 x 16384
HDMI-1 disconnected (normal left inverted right x axis y axis)
DSI-1 connected primary 1280x800+0+0 (normal left inverted right x axis y axis) 0mm x 0mm
   1280x800      60.22*+
DP-1 disconnected (normal left inverted right x axis y axis)";
    let content2 = "Screen 0: minimum 320 x 200, current 800 x 1280, maximum 16384 x 16384
HDMI-1 disconnected primary (normal left inverted right x axis y axis)
DSI-1 connected 800x1280+0+0 right (normal left inverted right x axis y axis) 0mm x 0mm
    1280x800      60.22*+
DP-1 disconnected (normal left inverted right x axis y axis)";

    let rotate_angle = content2.lines().into_iter().find_map(|line|-> Option<u16> {
        if line.contains("HDMI") {
            return None;
        }
        if line.contains("connected") && !line.contains("disconnected") {
            let parts: Vec<&str> = line.split(' ').collect();
            if let Some(part) = parts.get(3) {
                if let Some(angle) = direction_to_angle(part) {
                    return Some(angle);
                } else {
                    return Some(0);
                }
            }
        }
        None
    });
    if let Some(angle) = rotate_angle {
        println!("angle = {}", angle);
    } else {
        println!("angle is None !!!")
    }
}

fn direction_to_angle(direction: &str) -> Option<u16> {
    match direction {
        "right" => Some(90),
        "inverted" => Some(180),
        "left" => Some(270),
        "normal" => Some(0),
        _ => None
    }
}