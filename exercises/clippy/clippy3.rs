fn main() {
    // Fix 1: Remove dangerous unwrap() on None
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Instead of unwrapping None (which would panic), 
        // we can just handle the None case
        println!("Option is None");
    }

    // Fix 2: Add missing commas in array declaration
    let my_arr = &[
        -1, -2, -3,  // Added comma
        -4, -5, -6   // Added comma
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Fix 3: Proper way to clear a vector
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();  // Use clear() instead of resize()
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // Fix 4: Correct variable swapping
    let mut value_a = 45;
    let mut value_b = 66;
    // Proper way to swap values in Rust
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}