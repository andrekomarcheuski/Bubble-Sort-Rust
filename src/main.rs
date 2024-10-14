fn main() {
    let vec = create_vec();
    let ordered_vec = bubble_sort(vec);
    for i in ordered_vec {
        print!("{} ", i);
    }
}

fn create_vec() -> Vec<i32> {
    let numbers = [7, 4, 1, 8, 5, 2, 9, 3, 6, 0];
    let mut vec: Vec<i32> = Vec::new();
    for i in numbers {
        vec.push(i);
    }
    vec
}

fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let size = vec.len();
    let mut aux;
    for i in 0..size {
        for j in 0..size-i-1 {
            if vec[j] > vec [j+1] {
                aux = vec[j];
                vec[j] = vec[j+1];
                vec[j+1] = aux;
            }
        }
    }
    vec
}