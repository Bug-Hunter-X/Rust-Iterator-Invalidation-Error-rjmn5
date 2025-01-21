fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Clone the vector to avoid invalidation
    let cloned_vec = vec.clone();
    
    let mut iter = cloned_vec.iter();

    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());

    // Modify the original vector
    vec.push(4);

    println!("Third element: {}", iter.next().unwrap());
    println!("Vector after modification {:?}", vec);
    println!("Iterated Vector {:?}", cloned_vec);
} 