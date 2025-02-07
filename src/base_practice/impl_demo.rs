struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 };
    }

    fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }
}

fn impl_demo() {
    let p = Point::origin();
    println!("p.x={}, p.y={}",p.x, p.y);

    let pp = Point::new(1.2, 2.3);
    println!("pp.x={}, pp.y={}", pp.x, pp.y);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 完成 area 方法，返回矩形 Rectangle 的面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn rectangle_demo() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);
}


#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    fn color(&self) -> String {
        "yellow".to_string()
    }
}

fn traffic_light_color_demo() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_impl_demo() {
        impl_demo();
    }

    #[test]
    fn test_rectangle_demo() {
        rectangle_demo();
    }

    #[test]
    fn test_traffic_light_color_demo() {
        traffic_light_color_demo();
    }
}