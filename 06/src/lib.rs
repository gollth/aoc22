use std::collections::HashSet;

pub fn start_marker(sequence: &str) -> Option<usize> {
    offset_of_unique_n(4, sequence)
}

pub fn start_message(sequence: &str) -> Option<usize> {
    offset_of_unique_n(14, sequence)
}

fn offset_of_unique_n(marker_length: usize, sequence: &str) -> Option<usize> {
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

    const SAMPLES: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn a_sample1() {
        assert_eq!(start_marker(SAMPLES[0]), Some(7));
    }

    #[test]
    fn a_sample2() {
        assert_eq!(start_marker(SAMPLES[1]), Some(5));
    }

    #[test]
    fn a_sample3() {
        assert_eq!(start_marker(SAMPLES[2]), Some(6));
    }

    #[test]
    fn a_sample4() {
        assert_eq!(start_marker(SAMPLES[3]), Some(10));
    }

    #[test]
    fn a_sample5() {
        assert_eq!(start_marker(SAMPLES[4]), Some(11));
    }

    #[test]
    fn b_sample1() {
        assert_eq!(start_message(SAMPLES[0]), Some(19));
    }

    #[test]
    fn b_sample2() {
        assert_eq!(start_message(SAMPLES[1]), Some(23));
    }

    #[test]
    fn b_sample3() {
        assert_eq!(start_message(SAMPLES[2]), Some(23));
    }

    #[test]
    fn b_sample4() {
        assert_eq!(start_message(SAMPLES[3]), Some(29));
    }

    #[test]
    fn b_sample5() {
        assert_eq!(start_message(SAMPLES[4]), Some(26));
    }
}
