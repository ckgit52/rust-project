use std::{fmt::format, io::SeekFrom};

pub trait summary {
    fn summarize(&self)->String;
}

struct article {
    title: String,
    author: String,
    content: String,
}

impl summary for article {
    fn summarize(&self)->String {
        format!("{} by {}",self.title,self.author)
        // return ().to_string();
    }
}

struct rectangle {
    w:i32,
    h:i32,
}

impl rectangle {
    fn area(&self)->i32{
        return self.w * self.h;
    }

    fn new(wid:i32,height:i32)->rectangle{
       
            rectangle{
                w:wid,
                h:height
            }
    }
    fn square(&self)->bool{

        return self.w==self.h;
    }
}

trait Summary {
    fn summarize(&self)->String;
}

struct articles {
    title:String,
    author:String,
}

// impl Summary for articles{
//     fn summarize(&self)->String{
//         format!("{} by {}",self.title,self.author);
//     };
// }


// fn main(){
//     let newarticle=article{
//         title: String::from("rust trait guide"),
//         author: String::from("chandan kumar"),
//         content: String::from("understanding rust"),
//     }; 

//     let rect=rectangle::new(20, 10);
//     let rect2=rectangle::new(10,10);

//     let issquare=rect2.square();

//     if(issquare){
//         println!("same hai rect2");
//     }else{
//         println!("same nhi  hai rect2");
//     }

//     println!("{}",rect.area());

//     println!("{}",newarticle.summarize());
// }


trait Animal {
    fn speak(&self) -> String;
    fn move_around(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) -> String {
        String::from("Woof! Woof!")
    }

    fn move_around(&self) -> String {
        String::from("The dog is running happily.")
    }
}

struct Bird;

impl Animal for Bird {
    fn speak(&self) -> String {
        String::from("Chirp! Chirp!")
    }

    fn move_around(&self) -> String {
        String::from("The bird is flying in the sky.")
    }
}

// fn main() {
//     let dog = Dog;
//     let bird = Bird;

//     println!("Dog speaks: {}", dog.speak());
//     println!("Dog moves: {}", dog.move_around());

//     println!("Bird speaks: {}", bird.speak());
//     println!("Bird moves: {}", bird.move_around());
// }

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("The area is: {}", shape.area());
}

// fn main() {
//     let circle = Circle { radius: 5.0 };
//     let rectangle = Rectangle { width: 4.0, height: 6.0 };

//     print_area(circle);
//     print_area(rectangle);
// }


trait Vehicle {
    fn wheels(&self) -> u32 {
        4
    }

    fn drive(&self);
}

struct Car;

impl Vehicle for Car {
    fn drive(&self) {
        println!("The car is driving on the road.");
    }
}

struct Bike;

impl Vehicle for Bike {
    fn wheels(&self) -> u32 {
        2
    }

    fn drive(&self) {
        println!("The bike is driving on the track.");
    }
}

// fn main() {
//     let car = Car;
//     let bike = Bike;

//     println!("Car has {} wheels.", car.wheels());
//     car.drive();

//     println!("Bike has {} wheels.", bike.wheels());
//     bike.drive();
// }


trait CanFly {
    fn fly(&self) -> String;
}

trait CanSwim {
    fn swim(&self) -> String;
}

struct Duck;

impl CanFly for Duck {
    fn fly(&self) -> String {
        String::from("The duck is flying in the sky.")
    }
}

impl CanSwim for Duck {
    fn swim(&self) -> String {
        String::from("The duck is swimming in the lake.")
    }
}

fn main() {
    let duck = Duck;

    println!("{}", duck.fly());
    println!("{}", duck.swim());
}

