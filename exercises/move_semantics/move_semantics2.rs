// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

fn main() {
    let mut vec0 = Vec::new();

    // // Do not change the following line!
    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let mut vec1 = vec0.clone();

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {
    // let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}
