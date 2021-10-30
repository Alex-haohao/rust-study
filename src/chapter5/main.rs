struct User {
    username: String,
    password: String,
    email: String,
    active: bool,
}

impl User {
    fn admin() -> User {
        User {
            username: String::from("admin"),
            password: String::from("admin"),
            email: String::from("admin@qq.com"),
            active: true,
        }
    }
}

fn main() {
    //struct
    // let mut s = User {
    //     username: String::from("alex"),
    //     password: String::from("haha"),
    //     email: String::from("123@qq.com"),
    //     active: true,
    // };

    // let w = 30;
    // let h = 50;
    // println!("{}", area(w, h));

    let user = User::admin();
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
