use std::{mem::size_of_val};

fn get_size_of_char() {
    let c1 = 'a';
    
    assert_eq!(size_of_val(&c1), 4);

    let c2 = "中";
    assert_eq!(size_of_val(&c2), 16);
}

fn uint_demo() {
    let uint = ();
    assert_eq!(size_of_val(&uint), 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_size_of_val() {
        get_size_of_char();
    }

    #[test]
    fn test_uint_demo() {
        uint_demo();
    }
}