#[macro_use]
extern crate wqactix;
extern crate actix;

use wqactix::com::macros;
use actix::{msgs, Actor, Addr, Arbiter, Context, System};
use wqactix::com::actors;


struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am alive!");
        System::current().stop(); // <- stop system
    }
}

fn kankan_actor(){
    let system = System::new("test");

    let addr = MyActor.start();

    system.run();
}


//------------------------------------------------------------
#[macro_export]
macro_rules! say_hello1{
   ()=>(
       println!("Hello111");
   )
}

fn kankan_macro(){
    say_hello1!();
}

fn kankan(){
    say_hello!();
    //say_hello1!();
    kankan_macro();

    macros::kankan1();
}
//-------------------------------------------------------------

fn main() {
    //kankan();
    //kankan_actor();

    //kankan1();

    actors::kankan();
}

fn multiply(m: i32) -> Box<Fn(i32)->i32> {
    Box::new(move |x|x*m)
}

fn kankan1(){
    let f = multiply(5);
    println!("{}", f(2));
}

fn kankan2(){
    let f = multiply1(5);
    println!("{}", f(2));
}

//fn multiply2<T>(m: i32) -> T where T:Fn(i32)->i32 {
//    move |x|x*m
//}

fn multiply1(m: i32) -> impl Fn(i32)->i32 {
    move |x|x*m
}