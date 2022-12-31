use std::iter::Iterator;

pub(crate) fn create() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // iter() get the reference of v1

    println!("{:?}", v1);

    for val in v1_iter {
        // for loop: `v1_iter` moved due to this implicit call to `.into_iter()`
        println!("Got: {}", val);
    }

    // println!("{:?}", v1_iter); // value borrowed here after move
}

pub(crate) fn adaptors() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
