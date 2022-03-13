pub mod maths {
    pub mod add {
        pub fn add_twenty(x: &mut i32) {
            *x = *x + 20;
        }
        pub fn add_twelve(x: &mut i32) {
            *x = *x + 12;
        }
    }

    pub mod multiply {
        pub fn by_twenty(x: &mut i32) {
            *x = *x*20;
        }

        pub fn by_twelve(x: &mut i32) {
            *x = *x*12;
        }
    }
}

pub fn twenty_twelve(x: &mut i32) {
    // Absolute path
    crate::maths::multiply::by_twenty(x);
    // Relative path
   maths::add::add_twelve(x);
}
