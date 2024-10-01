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

impl Summary for articles{
    fn summarize(&self)->String{
        format!("{} by {}",self.title,self.author);
    };
}


fn main(){
    let newarticle=article{
        title: String::from("rust trait guide"),
        author: String::from("chandan kumar"),
        content: String::from("understanding rust"),
    }; 

    let rect=rectangle::new(20, 10);
    let rect2=rectangle::new(10,10);

    let issquare=rect2.square();

    if(issquare){
        println!("same hai rect2");
    }else{
        println!("same nhi  hai rect2");
    }

    println!("{}",rect.area());

    println!("{}",newarticle.summarize());
}