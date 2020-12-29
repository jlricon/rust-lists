use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_lists::single_linked::LinkedList;
fn insert_in_the_middle_of_list(size: usize, list: &mut LinkedList<i32>) {
    let mid = size / 2;
    list.insert_after(123, mid);
}
fn insert_in_the_middle_of_array(size: usize, vec: &mut Vec<i32>) {
    let mid = size / 2;
    vec.insert(mid, 123);
}
const SIZE: usize = 1000;
fn criterion_benchmark(c: &mut Criterion) {
    let mut list = LinkedList::new();
    (0..SIZE).for_each(|v| list.push_back(v as i32));
    let mut vec = Vec::new();
    (0..SIZE).for_each(|v| vec.push(v as i32));
    c.bench_function("insert in list of size 1000", |b| {
        b.iter(|| insert_in_the_middle_of_list(black_box(SIZE), &mut list))
    });
    c.bench_function("insert in array of size 1000", |b| {
        b.iter(|| insert_in_the_middle_of_array(black_box(SIZE), &mut vec))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
