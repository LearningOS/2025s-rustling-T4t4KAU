fn main() {
    // Fix 1: Safe handling of None
    let my_option: Option<()> = None;
    match my_option {
        None => println!("Option is None"),
        Some(_) => println!("Option has a value"),
    }

    // Fix 2: Proper array formatting
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Fix 3: Proper vector clearing
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // Fix 4: Improved value swapping
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}