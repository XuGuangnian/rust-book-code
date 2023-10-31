use std::iter::Iterator;

pub fn create() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // iter() get the reference of v1

    println!("{:?}", v1);

    for val in v1_iter {
        // for loop: `v1_iter` moved due to this implicit call to `.into_iter()`
        println!("Got: {}", val);
    }

    // println!("{:?}", v1_iter); // value borrowed here after move
}

pub fn adaptors() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        let v2 = vec![1, 2, 3];
        let mut v2_into_iter = v2.into_iter();
        assert_eq!(v2_into_iter.next(), Some(1));
        assert_eq!(v2_into_iter.next(), Some(2));
        assert_eq!(v2_into_iter.next(), Some(3));
        assert_eq!(v2_into_iter.next(), None);

        let mut v3 = vec![1, 2, 3];
        let mut v3_iter_mut = v3.iter_mut();
        assert_eq!(v3_iter_mut.next(), Some(&mut 1));
        assert_eq!(v3_iter_mut.next(), Some(&mut 2));
        assert_eq!(v3_iter_mut.next(), Some(&mut 3));
        assert_eq!(v3_iter_mut.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // `v1_iter` moved

        // println!("{:?}", v1_iter);

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}