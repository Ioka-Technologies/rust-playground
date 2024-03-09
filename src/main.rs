use playground::lessons::borrow;
use playground::errors;
use playground::utils;


fn main() {
    let mut vec_option = None;

    println!("Before calling fun: {:?}", vec_option);

    let mut test_val = 5;

    borrow::other_test(&mut vec_option);

    borrow::alter_vec(&mut test_val, 3, &mut vec_option);
    borrow::alter_vec(&mut test_val, 3, &mut vec_option);
    borrow::alter_vec(&mut test_val, 3, &mut vec_option);

    borrow::other_test(&mut vec_option);

    println!("After calling fun: {:?}", test_val);

    println!("After calling fun: {:?}", vec_option);

    let result = errors::test_error("test.txt".to_string());

    if let Err(e) = result {
        println!("Error opening file: {}", e);
    }

    println!("File opened successfully");

    println!("5 + 10 is {}", utils::add::do_add(5, 10));
}
