#![allow(dead_code)]

pub fn main() {
    // loopx();
}


fn loopm() {
    let array = [10, 20, 30];
    for a in array {
        println!("{a}")
    }
    println!();
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}


fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // ANCHOR_END: transpose
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

fn loopx() {
    let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let trans: [[i32; 3]; 3] = transpose(array);
    println!("trans: {trans:#?}");
}
