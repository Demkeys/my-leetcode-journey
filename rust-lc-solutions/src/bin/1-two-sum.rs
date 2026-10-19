use core::num;

fn main() {
    // two_sum(vec![2, 7, 11, 15], 9);
    assert!(two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1]);
    assert!(two_sum(vec![3, 2, 4], 6) == vec![1, 2]);
    assert!(two_sum(vec![3, 3], 6) == vec![0, 1]);
    println!("Hello world!");
}

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
