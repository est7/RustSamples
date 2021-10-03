pub fn main() {}

fn loop_label() {
    let mut count = 0;
    'label: loop {
        println!("[{}]go", count);
        count += 1;

        if count >= 5 {
            break 'label;
        };
    };
}

fn loop_break_return_value() {
    let mut count = 0;
    let result = 'label: loop {
        println!("[{}]go", count);
        count += 1;

        if count >= 5 {
            break 'label 10;
        };
    };

    println!("loop result {}", result)
}

fn for_break_cannot_return_value() {
    let a = [1, 2, 3];
    'label: for i in a {
        println!("{}", i);
        if let 3 = i {
            break 'label;
        }
    };
}