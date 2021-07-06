// Let's create a library, and then see how to link it to another crate.


pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}


// compilation code
// $ rustc --crate-type=lib rary.rs
// $ ls lib*
// library.rlib
