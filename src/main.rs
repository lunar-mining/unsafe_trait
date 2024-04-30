use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Bar {
    dyn_hashmap: Mutex<HashMap<&'static str, Arc<dyn Foo>>>,
}

trait Foo {
    fn do_something(&self, bar: &mut Box<dyn Stream>);
}

trait Stream {}

fn main() {
    println!("Hello, world!");
}
