fn main() {
    let v = vec![1, 2, 3, 4];
    let mut out: Vec<usize> = v
        .into_par_stream()
        .map(|n| { n * n })
        .collect()
        .await;
    out.sort();
    assert_eq!(out, vec![1, 4, 9, 16]);
}