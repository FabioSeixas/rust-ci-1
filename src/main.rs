fn main() {
    sum(1, 1);
}

fn sum(a: usize, b: usize) -> usize {
    a + b
}

#[test]
fn it_works() {
    let result = sum(2, 2);
    assert_eq!(result, 4);
}
