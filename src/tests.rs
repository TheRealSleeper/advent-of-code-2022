#[allow(dead_code, unused_variables)]
#[cfg(test)]
fn part1() {
    let sample = std::fs::read_to_string("sample1.txt").unwrap();
    let expected: isize = 15;
    let actual = super::part1(&sample);
    assert_eq!(actual, expected);
}

#[allow(dead_code, unused_variables)]
#[cfg(test)]
fn part2() {
    let sample = std::fs::read_to_string("sample2.txt").unwrap();
    let expected: isize = 12;
    let actual = super::part2(&sample);
    assert_eq!(actual, expected);
}
