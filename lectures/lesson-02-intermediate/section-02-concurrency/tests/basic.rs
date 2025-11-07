use lesson02_section02_concurrency::parallel_sum;

fn seq_sum(xs: &[i32]) -> i32 { xs.iter().copied().sum() }

#[test]
fn small_inputs() {
    let data = [1, 2, 3, 4, 5];
    for n in 0..=6 {
        assert_eq!(parallel_sum(&data, n), seq_sum(&data));
    }
}

#[test]
fn larger_input_many_threads() {
    let data: Vec<i32> = (1..=10_000).collect();
    assert_eq!(parallel_sum(&data, 1), seq_sum(&data));
    assert_eq!(parallel_sum(&data, 2), seq_sum(&data));
    assert_eq!(parallel_sum(&data, 8), seq_sum(&data));
}

