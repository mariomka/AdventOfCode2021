use std::convert::{TryFrom, TryInto};

pub fn part1(input: &str) -> usize {
    let message: String = hex2binary(input);
    let mut packet_parser = PacketParser::new(message);

    packet_parser.next().unwrap().sum_versions()
}

pub fn part2(input: &str) -> usize {
    let message: String = hex2binary(input);
    let mut packet_parser = PacketParser::new(message);

    packet_parser.next().unwrap().evaluate()
}

fn hex2binary(hex: &str) -> String {
    hex.chars()
        .map(|char| match char {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Unexpected char."),
        })
        .collect()
}

struct PacketParser {
    message: String,
    cursor: usize,
}

impl PacketParser {
    fn new(message: String) -> Self {
        PacketParser { message, cursor: 0 }
    }

    fn forward(&mut self, moves: usize) -> &str {
        let range = self.cursor..self.cursor + moves;
        self.cursor += moves;

        &self.message[range]
    }

    fn forward_as_usize(&mut self, moves: usize) -> usize {
        usize::from_str_radix(self.forward(moves), 2).unwrap()
    }
}

impl Iterator for PacketParser {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.message.len() - self.cursor == 0 {
            return None;
        }

        let version = self.forward_as_usize(3);
        let packet_type = self.forward_as_usize(3).try_into().unwrap();

        let packet = match packet_type {
            PacketType::Literal => {
                let mut value = String::new();

                loop {
                    let start_bit = self.forward_as_usize(1).to_owned();

                    value.push_str(self.forward(4));

                    if 0 == start_bit {
                        break;
                    }
                }

                Packet::Literal {
                    version,
                    value: usize::from_str_radix(value.as_str(), 2).unwrap(),
                }
            }
            _ => {
                let length_type = self.forward_as_usize(1);

                let sub_packets = match length_type {
                    0 => {
                        let length = self.forward_as_usize(15);
                        let sub_message = self.forward(length).to_string();
                        let packet_parser = PacketParser::new(sub_message);

                        packet_parser.into_iter().collect()
                    }
                    1 => {
                        let sub_packets_number = self.forward_as_usize(11);

                        (0..sub_packets_number)
                            .into_iter()
                            .map(|_| self.next().unwrap())
                            .collect()
                    }
                    _ => panic!("Unknown packet type."),
                };

                Packet::Operator {
                    version,
                    packet_type,
                    sub_packets,
                }
            }
        };

        Some(packet)
    }
}

#[allow(dead_code)]
#[repr(usize)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TryFrom<usize> for PacketType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(unsafe { ::std::mem::transmute(value) })
    }
}

enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        packet_type: PacketType,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn evaluate(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                packet_type,
                sub_packets,
                ..
            } => {
                let values: Vec<usize> =
                    sub_packets.iter().map(|packet| packet.evaluate()).collect();

                match packet_type {
                    PacketType::Sum => values.into_iter().sum(),
                    PacketType::Product => values.into_iter().product(),
                    PacketType::Minimum => values.into_iter().min().unwrap(),
                    PacketType::Maximum => values.into_iter().max().unwrap(),
                    PacketType::GreaterThan => {
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    PacketType::LessThan => {
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    PacketType::EqualTo => {
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown packet type."),
                }
            }
        }
    }

    fn sum_versions(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => {
                *version
                    + sub_packets
                        .iter()
                        .map(|packet| packet.sum_versions())
                        .sum::<usize>()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("D2FE28"), 6);
        assert_eq!(part1("EE00D40C823060"), 14);
        assert_eq!(part1("38006F45291200"), 9);
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
