pub fn adding_i32(a:i32, b:i32)->i32{
    a+b
}

#[cfg(test)]
mod tests {
    use crate::adding_i32;

    #[test]
    fn it_works() {
        let result = adding_i32(2, 2);
        assert_eq!(result, 4);
    }
}
