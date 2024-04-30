use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Bar {
    dyn_hashmap: Mutex<HashMap<&'static str, Arc<dyn Foo>>>,
}

// We cannot use generic type parameters here.
//
// For a trait to be "object safe" it needs to allow building
// a vtable to allow the call to be resolvable dynamically:
// https://doc.rust-lang.org/reference/items/traits.html#object-safety
trait Foo {
    fn do_something<R>(&self, bar: &mut R);
}

fn main() {
    println!("Hello, world!");
}
