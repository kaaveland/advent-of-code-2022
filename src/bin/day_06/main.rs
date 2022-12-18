use std::collections::VecDeque;
use std::io;

#[cfg(test)]
const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn start_of_packet(stream: &str, packet_length: usize) -> usize {
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(packet_length);
    let chars: Vec<char> = stream.chars().clone().collect();
    let mut position = 0;
    while position < chars.len() {
        let ch = *chars.get(position).unwrap();
        if buffer.len() == packet_length {
            buffer.pop_front();
        }
        buffer.push_back(ch);
        let mut dupes = false;
        for i in 0..buffer.len() {
            for j in (i + 1)..buffer.len() {
                if buffer[i] == buffer[j] {
                    dupes = true;
                }
            }
        }
        position += 1;
        if buffer.len() == packet_length && !dupes {
            return position;
        }
    }
    position
}

#[cfg(test)]
#[test]
fn test_start_of_packet() {
    assert_eq!(start_of_packet(EXAMPLE, 4), 7);
    assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

fn main() {
    let stream = io::stdin().lines().next().expect("Need 1 line of input").expect("Need 1 line of input");
    let part_1 = start_of_packet(stream.as_str(), 4);
    println!("part 1: {}", part_1);
    let part_2 = start_of_packet(stream.as_str(), 14);
    println!("part 2: {}", part_2);
}
