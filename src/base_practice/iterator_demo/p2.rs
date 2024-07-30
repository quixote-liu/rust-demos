fn p2_demo() {
    let mut v = Vec::new();
    for n in 1..101 {
       v.push(n);
    }

    assert_eq!(v.len(), 100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2_demo() {
        p2_demo()
    }
}
