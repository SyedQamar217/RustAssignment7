pub mod person{
    pub mod student{
        pub fn details (){
            println!("I am a sub module function!!");
        }
    }
}
fn main() {
    println!("Hello, world!");
        person::student::details();
}
