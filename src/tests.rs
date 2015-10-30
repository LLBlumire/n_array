#![cfg(test)]

use super::*;

#[test]
fn it_works() { }


#[test]
#[should_panic]
fn incorrect_initialisation() {
    let x = NArray::<i32>::new(2, &[3, 4, 2]);
}

#[test]
fn correct_initialisation() {
    let x = NArray::<i32>::new(2, &[3, 3]);
}

#[test]
fn set_and_get_values() {
    let mut my_n_array = NArray::<i32>::new(2, &[5, 3]);
    for x in 0..5 {
        for y in 0..3 {
            my_n_array[&[x, y]] = (x*y) as i32;
        }
    }
    assert_eq!(my_n_array[&[4, 2]], 8);
}

#[test]
fn set_with_function() {
    let mut n_array = NArray::<i32>::from_function(2, &[5, 3], |n: &[usize]| -> i32 {
        let r = n.to_vec().iter().fold(1, |acc, &item| acc*item as i32);
        println!("{:?} = {:?}", n, r);
        r
    });
    println!("{:?}", n_array);
    assert_eq!(n_array[&[3, 2]], 6);
}

#[test]
#[should_panic]
fn get_out_of_range() {
    let my_n_array = NArray::<i32>::new(2, &[3, 3]);
    my_n_array[&[3, 1]];
}

#[test]
#[should_panic]
fn set_out_of_range() {
    let mut my_n_array = NArray::<i32>::new(2, &[3, 3]);
    my_n_array[&[2, 5]] = 2;
}
