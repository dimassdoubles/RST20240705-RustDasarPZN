use crate::third_module::say_hellomas as say_hellomas_third;

pub fn say_hellomas() {
    println!("Hello Mas");
    say_hellomas_third();
}

pub mod sub_first {
    pub mod sub_sub_first {
        pub fn say_hello_sub() {
            crate::first_module::say_hellomas();
            super::super::say_hellomas();
        }
    }
}