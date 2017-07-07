// A module named `joesan`
mod joesan {
    // The default visibility is private
    fn private_function() {
        println!("called `joesan::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn public_function() {
        println!("called `joesan::public_function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `joesan::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Modules allow disambiguation between items that have the same name.
    function();
    my::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my::indirect_access();
    my::nested::function();

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my::private_nested::function();
    // TODO ^ Try uncommenting this line
}