use crate::day::Day;
use crate::day::Solveable;

fn get_looping_iterator<T>(v: Vec<T>) -> impl Iterator<Item = T>
where
    T: Clone,
{
    v.into_iter().cycle()
}

fn get_spiral(n: usize) -> Vec<Vec<usize>> {
    if n % 2 == 0 {
        panic!("n must be odd");
    }
    let mut spiral = vec![vec![0; n]; n];
    let mut row = n / 2;
    let mut col = n / 2;
    spiral[row][col] = 1;
    let mut elt = 2;
    let mut dir_iterator = get_looping_iterator(vec![(0i64, 1i64), (1, 0), (0, -1), (-1, 0)]);
    for leg_len in (1..n)
        .into_iter()
        .map(|k| std::iter::repeat(k).take(if k < n - 1 { 2 } else { 3 }))
        .flatten()
    {
        let (row_dir, col_dir) = dir_iterator.next().unwrap();
        for _ in 0..leg_len {
            row = (row as i64 + row_dir) as usize;
            col = (col as i64 + col_dir) as usize;
            spiral[row][col] = elt;
            elt += 1;
        }
    }
    spiral
}
struct Part1;
impl Solveable for Part1 {
    fn solve(&self, _lines: &Vec<String>) -> String {
        let spiral = get_spiral(1001);
        ((0..1001).map(|i| spiral[i][i] + spiral[i][1000 - i]).sum::<usize>() - 1).to_string()
    }
}

get_day_fn!(Part1);

#[test]
fn test_get_looping_iterator() {
    let iter = get_looping_iterator(vec![1, 2, 3]);
    assert_eq!(iter.take(5).collect::<Vec<_>>(), vec![1, 2, 3, 1, 2]);
}

#[test]
fn test_get_spiral() {
    assert_eq!(
        get_spiral(5),
        vec![
            vec![21, 22, 23, 24, 25],
            vec![20, 7, 8, 9, 10],
            vec![19, 6, 1, 2, 11],
            vec![18, 5, 4, 3, 12],
            vec![17, 16, 15, 14, 13],
        ]
    );
}
