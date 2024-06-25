struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn gen_demo() {
    // 使用非泛型函数
    reg_fn(S(A));          // 具体的类型
    gen_spec_t(SGen(A));   // 隐式地指定类型参数  `A`.
    gen_spec_i32(SGen(33)); // 隐式地指定类型参数`i32`.

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式地指定类型参数 `char`.
    generic(SGen('a'));
}

// 4
fn sum<T>(x: T, y: T) -> T 
where
    T: std::ops::Add<Output = T>,
{
    x + y
}

fn sum_demo() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

// 5
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn point_demo() {
    let _integeer = Point{x: 5, y:8};
    let _float = Point{x: 1.2, y:2.3};
    let _p = Point2{x: 2, y: "hello".to_string()};
}

struct Val<T> {
    val: T,
}

impl <T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn val_demo() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

// 6
struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    // 实现 mixup，不要修改其它代码！
    fn mixup<V, W>(self, p2: Point3<V, W>) -> Point3<T, W> {
        Point3{
            x: self.x,
            y: p2.y,
        }
    }
}

fn point3_demo() {
    let p1 = Point3 { x: 5, y: 10 };
    let p2 = Point3 { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    // println!("p2.x={}", p2.x);
    // println!("p2.x={}", p2.y);
    // println!("p1.x={}", p1.x);
    // println!("p1.x={}", p1.y);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}

// 7
// 修复错误，让代码工作
struct Point4<T> {
    x: T,
    y: T,
}

impl Point4<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn point4_demo() {
    let p = Point4{x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin())
}