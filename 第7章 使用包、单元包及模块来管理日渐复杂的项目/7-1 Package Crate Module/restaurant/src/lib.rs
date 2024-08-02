mod front_of_house {
    // 子Module
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    // 子Module
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}