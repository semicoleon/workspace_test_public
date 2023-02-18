pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub mod nested {
    pub fn add_two(arg: i64) -> i64 {
        arg + 2i64
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_too() {
        let result = nested::add_two(33);
        assert_eq!(result, 35);
    }
}
