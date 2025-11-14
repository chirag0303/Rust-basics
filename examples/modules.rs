#![allow(unused)]

mod foo {
    pub fn print(){
        println!("foo");
    }
}
mod my {
    // use super::foo; //this tell to look for one above module
    // pub fn printFoo(){
    //     foo::print();
    // }
    pub fn print(){
        println!("Rust");
    }
    fn private(){
        println!("Private");
    }

    pub mod a{
        use super::super::foo;

        pub fn printFoo(){
        foo::print();
    }

        pub fn print(){
            println!("A");
        }
        pub struct S{
            pub name: String,
            id: i32 // id is private to outer functions
        }
        pub fn build(id: i32) -> S{
            S {
                id,
                name: "Sam".to_string(),
            }
        }
    }
}

fn main(){
    my::print();
    my::a::print();
    // let details = my::a::S{
    //     name: "Alex".to_string(),
    //     id: 1, // this will give error : id is private
    // };
    let details = my::a::build(32);
    my::printFoo();
}