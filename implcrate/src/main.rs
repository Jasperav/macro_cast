use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use procmacro::add_casts;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_some_casts() {
        expand_casts(1, 2)
    }

    fn expand_casts(column0: i32, column1: i32) {
        // Should expand to
        // ("some_text", column0 as i32, column1 as i32);
        add_casts!("some_text", column0, column1);
    }
}