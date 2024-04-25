fn slice_demo() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_demo() {
        slice_demo();
    }
}