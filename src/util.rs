
pub fn usize_to_i32(input: usize) -> i32 {
    let ret: i32 = match input.try_into() {
        Ok(result) => result,
        Err(_) => panic!("Could not convert usize value into i32")
    };

    ret
}
