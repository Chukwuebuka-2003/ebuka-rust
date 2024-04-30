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

// If module A is contained inside module B, then module A is a child of module B and module B is a parent of module A
