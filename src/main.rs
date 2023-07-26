use rand::{thread_rng, Rng, distributions::Uniform};

fn stalin_sort(array: &mut Vec<i8>) {
    let mut index = 0;
    while let Some(x) = array.get(index + 1) {
        if array[index] > *x {
            array.remove(index + 1);
        } else {
            index += 1;
        }
    }
}

fn main() {
    let range = Uniform::from(-128..127);

    let mut array: Vec<i8> = thread_rng()
        .sample_iter(&range)
        .take(100)
        .collect();

    println!("Initial array:\n{:?}", array);

    stalin_sort(&mut array);

    println!("Purged array:\n{:?}", array)
}
