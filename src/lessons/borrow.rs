pub fn alter_vec(alter_count: &mut i32, minimum: usize, vec_option: &mut Option<Vec<i32>>) {
    *alter_count += 1;

    if let Some(vec) = vec_option {
        vec.push(vec.len() as i32 + 1);
    } else {
        vec_option.replace(vec![1]);
    }

    // This doesn't compile because the unwrap attempts to "consume" or move
    // the value but vec_option is a mutable reference and that is not allowed.
    // if vec_option.unwrap().len() < 3 {
    //     alter_vec(alter_count, minimum, vec_option);
    // }

    // First convert the vec_option using as_ref which essentially moves the
    // reference from the Option to the Vec. Once you do that you can use unwrap
    // since moving the Option is now allowed.
    if vec_option.as_ref().unwrap().len() < minimum {
        alter_vec(alter_count, minimum, vec_option);
    }
}

// Simple way of testing for a single enum variant using `if let`
pub fn other_test(vec_option: &mut Option<Vec<i32>>) {
    let val = vec![10];

    if let None = vec_option {
        vec_option.replace(val);
    };
}

// Alternate way of checking if an option is None
pub fn other_test2(vec_option: &mut Option<Vec<i32>>) {
    let val = vec![10];

    match vec_option {
        None => {
            vec_option.replace(val);
            ()
        },
        _ => (),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alter_vec() {
        let mut alter_count = 0;
        let mut vec_option = None;

        alter_vec(&mut alter_count, 3, &mut vec_option);

        assert_eq!(alter_count, 3);
        assert_eq!(vec_option, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_other_test() {
        let mut vec_option = Some(vec![1, 2, 3]);

        other_test(&mut vec_option);
        assert_eq!(vec_option, Some(vec![1, 2, 3]));

        let mut vec_option = None;

        other_test(&mut vec_option);
        assert_eq!(vec_option, Some(vec![10]));
    }

    #[test]
    fn test_other_test2() {
        let mut vec_option = Some(vec![1, 2, 3]);

        other_test2(&mut vec_option);
        assert_eq!(vec_option, Some(vec![1, 2, 3]));

        let mut vec_option = None;

        other_test2(&mut vec_option);
        assert_eq!(vec_option, Some(vec![10]));
    }
}
