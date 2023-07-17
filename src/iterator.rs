#[derive(PartialEq,Debug)]
pub struct shoe {
    size: u32,
    style: String,
}

pub fn shoe_in_size(shoes: Vec<shoe>, shoe_size: u32) -> Vec<shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            shoe {
                size: 10,
                style: String::from("shoe1"),
            },
            shoe {
                size: 20,
                style: String::from("shoe2"),
            },
            shoe {
                size: 20,
                style: String::from("shoe3"),
            },
        ];
        let in_my_size = shoe_in_size(shoes, 20);
        assert_eq!(
            in_my_size,
            vec![
                shoe {
                    size: 20,
                    style: String::from("shoe2")
                },
                shoe {
                    size: 20,
                    style: String::from("shoe3")
                },
            ]
        );
    }
}