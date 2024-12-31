fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Consume the iterator to release the immutable borrow before modifying the vector
    let mut iter = vec.iter();
    for _ in iter {};

    vec.push(4);
    println!("Vector after modification: {:?}", vec);
} 