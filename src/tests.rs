#[allow(dead_code, unused_variables)]
#[test]
fn part1() {
    let sample = std::fs::read_to_string("sample1.txt").unwrap();
    let expected: isize = 2;
    let actual = super::part1(&sample);
    assert_eq!(actual, expected);
}

#[allow(dead_code, unused_variables)]
#[test]
fn part2() {
    let sample = std::fs::read_to_string("sample2.txt").unwrap();
    let expected: isize = 4;
    let actual = super::part2(&sample);
    assert_eq!(actual, expected);
}
