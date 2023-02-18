use libc::size_t;

#[link(name = "RustTestStatic", kind = "static")]
extern {
    fn print_int_val(value: i32) -> size_t;
}

pub fn print_int(num: i32) -> usize {
    unsafe {
        print_int_val(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_works() {
        let result = print_int(54);
        assert_eq!(result, 13);
    }
}
