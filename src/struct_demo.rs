use crate::input::input_number;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self)-> u32{
        self.width*self.height
    }

    fn can_hold(&self, other:&Rectangle)-> bool{
        self.width > other.width && self.height > other.height

    }

    fn square(size: u32)->Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn struct_example() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("Aflac01"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("aflac01@gmail.com");
}

fn build_user(email: String, username: String)-> User {
    User {
        email,
        username,
        active :true,
        sign_in_count : 1,
    }
}

fn update_user(user1: User, email: String, username: String)-> User{
    User {
        email,
        username,
        ..user1
    }
}

pub fn rectangles(){
    println!("Rectangel 1");
    let rect1 = ask_for_rectangle();
    println!("Rectangel 2");
    let rect2 = ask_for_rectangle();
    println!("Square");
    let rect3 = ask_for_square();
    println!("rect1 is {:#?}", rect1);

    println!("The area of the rectangle 1 is {} square pixels.", rect1.area());
    println!("can rect1 hold rect 2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect 3? {}", rect1.can_hold(&rect3));

}



fn ask_for_rectangle()->Rectangle{
    println!("Enter width of rectangle:");
    let width = input_number();
    println!("Enter width of rectangle:");
    let height = input_number();
    Rectangle{width,height}
}

fn ask_for_square()->Rectangle{
    println!("Enter square edge length:");
    let length = input_number();

    Rectangle::square(length)
}