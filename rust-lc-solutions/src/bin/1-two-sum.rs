use std::{collections::HashMap, hash::Hash};

fn main() {
    // two_sum(vec![2, 7, 11, 15], 9);
    assert!(two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1]);
    assert!(two_sum(vec![3, 2, 4], 6) == vec![1, 2]);
    assert!(two_sum(vec![3, 3], 6) == vec![0, 1]);

    assert!(two_sum_02(vec![2, 7, 11, 15], 9) == vec![0, 1]);
    assert!(two_sum_02(vec![3, 2, 4], 6) == vec![1, 2]);
    assert!(two_sum_02(vec![3, 3], 6) == vec![0, 1]);

    // two_sum_02(vec![2, 7, 11, 15], 9);
    // two_sum_02(vec![3, 2, 4], 6);
    // two_sum_02(vec![3, 3], 6);

    println!("Hello world!");
}

// Solution 1: Brute-force | O(n^2) Time
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, val_i) in nums.iter().enumerate() {
        let x = *val_i;
        for (j, val_j) in nums.iter().enumerate() {
            let y = target - x;

            if i != j && *val_j == y {
                return vec![i as i32, j as i32];
            }
        }
    }

    <Vec<i32>>::new()
}

// Solution 2: Two passes | O(n) Time
fn two_sum_02(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm01 = <HashMap<i32, i32>>::new();

    nums.iter().enumerate().for_each(|(i, v)| {
        hm01.insert(*v, i as i32);
    });

    println!("{:?}", hm01);

    for (i, v) in nums.iter().enumerate() {
        let comp = target - *v;
        if let Some(comp_v) = hm01.get(&comp)
            && *comp_v != i as i32
        {
            let res = vec![i as i32, *comp_v];
            println!("{:?}", res);
            return res;
        }
    }

    <Vec<i32>>::new()
}
