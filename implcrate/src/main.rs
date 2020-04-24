use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use procmacro::add_cast_m;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cast() {
        add_cast(1)
    }

    fn add_cast(i: i32) {
        // Should expand to
        // i as i64
        add_cast_m!(i);
    }
}