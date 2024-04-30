mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// The code above shows how some of the modules nest inside one another. The hosting module is nested inside the front_of_house module.
// Some modules are siblings to each other, this means they're defined in the same module; hosting
// and servings are siblings defined within front_of_house

// If module A is contained inside module B, then module A is a child of module B and module B is a  parent of module A.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// This code defines a module called `front_of_house` with a nested module called `hosting`, which contains a function `add_to_waitlist()`.

//Outside the module, there's a public function `eat_at_restaurant()` that calls `add_to_waitlist()` using both absolute and relative paths.

// Choosing whether to use a relative or absolute path is a decision you'll make based off your project structure
// and depends on whether you're more likely to move item definition code separately from or
// together with the code that uses it.

// In Rust, all items [functions, methods, structs, enums, modules and constants] are private to
// parent modules by default.
// If you want to make an item like a function private, you put it in a module.
// Items in a parent module can't use the private items inside child modules, but items in child
// modules can use the items in their parent modules. Child modules wrap and hide their
// implementation details from parent modules, but the child modules can see the context in which
// they're defined.

// Rust chose to have the module systen function this way so that hiding inner implementation
// details is the default. This way you know which parts of the inner code can be changed without
// breaking the outer code. Rust does give the third option to expose inner parts of child modules
// code to outer parent modules by using the pub keyword to make an item public.

// Making a module public doesn't make its contents public either.
