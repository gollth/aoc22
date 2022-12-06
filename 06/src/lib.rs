use std::collections::HashSet;

pub fn start_marker(sequence: &str) -> Option<usize> {
    let marker_length = 4;
    let seq = sequence.chars().collect::<Vec<_>>();
    let seq = seq
        .windows(marker_length)
        .take_while(|marker| marker.iter().collect::<HashSet<_>>().len() != marker_length)
        .collect::<Vec<_>>();

    if seq.is_empty() {
        return None;
    }

    Some(seq.len() + marker_length)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_sample1() {
        let sequence = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(start_marker(sequence), Some(7));
    }

    #[test]
    fn a_sample2() {
        let sequence = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(start_marker(sequence), Some(5));
    }

    #[test]
    fn a_sample3() {
        let sequence = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(start_marker(sequence), Some(6));
    }

    #[test]
    fn a_sample4() {
        let sequence = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(start_marker(sequence), Some(10));
    }

    #[test]
    fn a_sample5() {
        let sequence = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(start_marker(sequence), Some(11));
    }
}
