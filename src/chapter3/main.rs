const MAX_POINT: u32 = 100_000;

fn main() {
    // 整数类型 integers 8 16 32 64 128 bit
    let x: u32 = 5;

    //f32 单精读 f64 双精度
    let y: f32 = 3.2;

    //bool
    let z: bool = true;

    // tuple
    let tup: (f32, u32, char) = (2.2, 4, 'c');

    for_test();
}

//fn 函数

fn another_function(x: u32) {
    if x > 5 {
        println!("{}", x);
    } else {
        println!("false");
    }
}

fn loop_test() {
    let mut counter: i32 = 0;

    let res = {
        loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
            println!("{}", counter)
        }
    };

    println!("{}", res)
}

fn for_test() {
    let array = [1, 2, 3, 4, 5];

    for i in 0..5 {
        println!("{}", array[i]);
    }
}
