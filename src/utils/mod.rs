use std::panic;

pub fn set_panics_message(error_str: &'static str) {
    panic::set_hook(Box::new(move |_| {
        println!("{}", error_str);
    }));
}
