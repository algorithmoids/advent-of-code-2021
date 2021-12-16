use std::collections::VecDeque;

const INPUT_STR: &str =include_str!("input/day_16.txt");

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    Literal(u64),
    Subpackets(Vec<Packet>),
}


pub fn part_1() -> String {
    let input = parse_packet(&get_input(INPUT_STR), 0);
    format!("{:?}", sum_version(input.0))
}

pub fn part_2() -> String {
    let input = parse_packet(&get_input(INPUT_STR), 0);
    format!("{:?}", execute(&input.0))
}

fn sum_version(packet: Packet) -> u32 {
    return packet.version as u32 + match packet.payload {
        Payload::Literal(_) => 0,
        Payload::Subpackets(packets) => {
            packets.into_iter().map(|x| sum_version(x)).sum()
        }
    }
}

fn execute(packet: &Packet) -> i64 {
    match (&packet.payload, packet.type_id) {
        (Payload::Subpackets(subpacket), 0) => subpacket.into_iter().map(|x| execute(x)).sum(),
        (Payload::Subpackets(subpacket), 1) => subpacket.into_iter().map(|x| execute(x)).product(),
        (Payload::Subpackets(subpacket), 2) => subpacket.into_iter().map(|x| execute(x)).min().unwrap(),
        (Payload::Subpackets(subpacket), 3) => subpacket.into_iter().map(|x| execute(x)).max().unwrap(),
        (Payload::Literal(data), 4) => *data as i64,
        (Payload::Subpackets(subpacket), 5) => {
            if execute(&subpacket[0]) > execute(&subpacket[1]) {1} else {0}
        },
        (Payload::Subpackets(subpacket), 6) => {
            if execute(&subpacket[0]) < execute(&subpacket[1]) {1} else {0}
        },
        (Payload::Subpackets(subpacket), 7) => {
            if execute(&subpacket[0]) == execute(&subpacket[1]) {1} else {0}
        },
        _ => panic!("wrong type"),
    }
}

fn parse_packet(data: &[u8], pointer: usize) -> (Packet, usize) {
    let version = get_int(&data[pointer .. pointer + 3]) as u8;
    let type_id = get_int(&data[pointer + 3 .. pointer + 6]) as u8;

    let (payload, next_pointer) = match type_id {
        4 => {
            let mut literal = 0;
            let mut i = pointer + 6;
            loop {
                literal = literal << 4;
                literal += get_int(&data[i + 1 .. i + 5]) as u64;
                if data[i] == 0 {
                    break
                }
                i += 5;
            }
            (Payload::Literal(literal), i + 5)
        },
        _ => {
            let mut i = pointer + 6;
            let mut subpackets = vec![];

            if data[i] == 0 {
                let stop = i+16 + get_int(&data[i+1 .. i+16]) as usize;
                i = i + 16;
                loop {
                    let out = parse_packet(data, i);
                    i = out.1;
                    subpackets.push(out.0);

                    if i == stop {
                        break
                    }
                }
            }
            else {
                let count = get_int(&data[i+1 .. i+12]) as usize;
                i += 12;

                for _ in 0 .. count {
                    let out = parse_packet(data, i);
                    i = out.1;
                    subpackets.push(out.0);
                }
            }

            (Payload::Subpackets(subpackets), i)
        },
    };

    (Packet {
        version,
        type_id,
        payload,
    }, next_pointer)
}

fn get_int(data: &[u8]) -> u32 {
    let mut result = 0_u32;
    for x in data {
        result = result << 1;
        result += *x as u32;
    }

    result
}


fn get_input(input: &str) -> Vec<u8> {
    input.chars()
            .map(|x| {
                (0 .. 4).rev()
                    .map(move |d| (1 << d) & x.to_digit(16).unwrap())
                    .map(|x| if x == 0 {0_u8} else {1_u8})
            })
            .flatten()
            .collect()
}

#[cfg(test)]
mod test {
    use super::execute;
    use super::Payload::Subpackets;
    use super::{get_input, get_int, Packet, parse_packet};
    use super::Payload::Literal;
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_literal() {
        let input = get_input("D2FE28");
        let expected = (Packet { version: 6, type_id: 4, payload: Literal(2021)}, 21);

        assert_eq!(parse_packet(&input, 0), expected);
    }

    #[test]
    fn test_subpackets() {
        let input = get_input("38006F45291200");
        let expected = (Packet { version: 1, type_id: 6, payload: Subpackets(vec![
            Packet { version: 6, type_id: 4, payload: Literal(10) },
            Packet { version: 2, type_id: 4, payload: Literal(20) }
        ]) }, 49);

        assert_eq!(parse_packet(&input, 0), expected);

        let input = get_input("EE00D40C823060");
        let expected = (Packet { version: 7, type_id: 3, payload: Subpackets(vec![
            Packet { version: 2, type_id: 4, payload: Literal(1) },
            Packet { version: 4, type_id: 4, payload: Literal(2) },
            Packet { version: 1, type_id: 4, payload: Literal(3) }
        ]) }, 51);

        assert_eq!(parse_packet(&input, 0), expected);
    }

    #[test]
    fn test_rules() {
        assert_eq!(execute(&parse_packet(&get_input("D2FE28"), 0).0), 2021);
        assert_eq!(execute(&parse_packet(&get_input("EE00D40C823060"), 0).0), 3);
        assert_eq!(execute(&parse_packet(&get_input("04005AC33890"), 0).0), 54);

        assert_eq!(execute(&parse_packet(&get_input("C200B40A82"), 0).0), 3);
        assert_eq!(execute(&parse_packet(&get_input("04005AC33890"), 0).0), 54);
        assert_eq!(execute(&parse_packet(&get_input("880086C3E88112"), 0).0), 7);
        assert_eq!(execute(&parse_packet(&get_input("CE00C43D881120"), 0).0), 9);
        assert_eq!(execute(&parse_packet(&get_input("D8005AC2A8F0"), 0).0), 1);
        assert_eq!(execute(&parse_packet(&get_input("F600BC2D8F"), 0).0), 0);
        assert_eq!(execute(&parse_packet(&get_input("9C005AC2F8F0"), 0).0), 0);
        assert_eq!(execute(&parse_packet(&get_input("9C0141080250320F1802104A08"), 0).0), 1);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), "971")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), "20247770907")
    }
}
