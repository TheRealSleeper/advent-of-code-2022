#[allow(dead_code, unused_variables)]
#[test]
fn part1() {
    assert_eq!(super::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(super::part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(super::part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(super::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(super::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[allow(dead_code, unused_variables)]
#[test]
fn part2() {
    assert_eq!(super::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(super::part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(super::part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(super::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(super::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
