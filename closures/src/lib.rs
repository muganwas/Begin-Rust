#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn _size_in_shoes(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn _next() {
        let _nums = vec![1, 2, 3, 4, 5, 6];
        let mut _nums_iter = _nums.iter();
        assert_eq!(_nums_iter.next(), Some(&1));
        assert_eq!(_nums_iter.next(), Some(&2));
        assert_eq!(_nums_iter.next(), Some(&3));
    }
    #[test]
    fn _iter_sum() {
        let _nums = vec![1, 2, 3];
        let _nums_iter = _nums.iter();
        let _nums_sum: i32 = _nums_iter.sum();
        assert_eq!(_nums_sum, 6);
    }
    #[test]
    fn _new_iter() {
        let _nums = vec![1, 2, 3, 4, 5];
        let _new_nums: Vec<_> = _nums.iter().map(|n| n + 1).collect();
        assert_eq!(_new_nums, vec![2, 3, 4, 5, 6]);
    }
    #[test]
    fn size_among() {
        let shoes = vec![
            Shoe {
                size: 32,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 34,
                style: String::from("sandal"),
            },
            Shoe {
                size: 45,
                style: String::from("boot"),
            },
        ];
        let in_my_size = _size_in_shoes(shoes, 32);
        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 32,
                style: String::from("sneaker"),
            }]
        );
    }
}
