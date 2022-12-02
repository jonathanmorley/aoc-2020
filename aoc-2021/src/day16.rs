use std::iter;

use bitvec::bits;
use bitvec::prelude::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::multi::many_m_n;
use nom::multi::{many0, many_till};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use bitvec_nom::BSlice;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    version: u8,
    content: PacketContent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PacketContent {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl IntoIterator for Packet {
    type Item = Packet;
    type IntoIter = iter::Chain<iter::Once<Self::Item>, std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.clone()).chain(match self.content {
            PacketContent::Literal(_) => vec![],
            PacketContent::Sum(packets)
            | PacketContent::Product(packets)
            | PacketContent::Minimum(packets)
            | PacketContent::Maximum(packets)
            | PacketContent::GreaterThan(packets)
            | PacketContent::LessThan(packets)
            | PacketContent::EqualTo(packets) => {
                packets.into_iter().flat_map(Packet::into_iter).collect()
            }
        })
    }
}

impl From<Packet> for u64 {
    fn from(p: Packet) -> Self {
        match p.content {
            PacketContent::Sum(packets) => packets.into_iter().map(u64::from).sum(),
            PacketContent::Product(packets) => packets.into_iter().map(u64::from).product(),
            PacketContent::Minimum(packets) => packets.into_iter().map(u64::from).min().unwrap(),
            PacketContent::Maximum(packets) => packets.into_iter().map(u64::from).max().unwrap(),
            PacketContent::Literal(value) => value,
            PacketContent::GreaterThan(packets) => {
                if u64::from(packets[0].clone()).gt(&u64::from(packets[1].clone())) {
                    1
                } else {
                    0
                }
            }
            PacketContent::LessThan(packets) => {
                if u64::from(packets[0].clone()).lt(&u64::from(packets[1].clone())) {
                    1
                } else {
                    0
                }
            }
            PacketContent::EqualTo(packets) => {
                if u64::from(packets[0].clone()).eq(&u64::from(packets[1].clone())) {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Length {
    Bits(u16),
    Packets(u16),
}

fn parse_header(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, (u8, u8)> {
    tuple((
        map(take(3usize), |bits: BSlice<Msb0, u8>| bits.0.load_be()),
        map(take(3usize), |bits: BSlice<Msb0, u8>| bits.0.load_be()),
    ))(input)
}

fn parse_length(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Length> {
    alt((
        map(
            preceded(tag(BSlice(bits![0])), take(15usize)),
            |bits: BSlice<Msb0, u8>| Length::Bits(bits.0.load_be()),
        ),
        map(
            preceded(tag(BSlice(bits![1])), take(11usize)),
            |bits: BSlice<Msb0, u8>| Length::Packets(bits.0.load_be()),
        ),
    ))(input)
}

fn parse_literal(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, u64> {
    let (input, (continues, terminal)) = many_till(
        preceded(tag(BSlice(bits![1])), take(4usize)),
        preceded(tag(BSlice(bits![0])), take(4usize)),
    )(input)?;

    let mut bits = BitVec::<Msb0, u8>::new();

    for slice in continues {
        bits.extend(slice.0);
    }
    bits.extend(terminal.0);

    Ok((input, bits.load_be()))
}

fn parse_packet(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Packet> {
    let (input, header) = parse_header(input)?;

    match header {
        (version, 4) => {
            let (input, content) = parse_literal(input)?;
            Ok((
                input,
                Packet {
                    version,
                    content: PacketContent::Literal(content),
                },
            ))
        }
        (version, type_id) => {
            let (input, length) = parse_length(input)?;
            let (input, packets) = parse_packets(input, length)?;

            Ok((
                input,
                Packet {
                    version,
                    content: match type_id {
                        0 => PacketContent::Sum(packets),
                        1 => PacketContent::Product(packets),
                        2 => PacketContent::Minimum(packets),
                        3 => PacketContent::Maximum(packets),
                        5 => PacketContent::GreaterThan(packets),
                        6 => PacketContent::LessThan(packets),
                        7 => PacketContent::EqualTo(packets),
                        _ => unreachable!(),
                    },
                },
            ))
        }
    }
}

fn parse_packets(
    input: BSlice<Msb0, u8>,
    length: Length,
) -> IResult<BSlice<Msb0, u8>, Vec<Packet>> {
    match length {
        Length::Bits(l) => {
            let (input, packet_data) = take(l as usize)(input)?;
            let (_, packets) = many0(parse_packet)(packet_data)?;
            Ok((input, packets))
        }
        Length::Packets(l) => many_m_n(l as usize, l as usize, parse_packet)(input),
    }
}

pub fn parse(input: &str) -> Packet {
    let bytes = hex::decode(&input.trim_end()).unwrap();
    let bits = BSlice(BitSlice::<Msb0, _>::from_slice(&bytes).unwrap());
    parse_packet(bits).unwrap().1
}

pub fn part1(input: &str) -> u64 {
    parse(input)
        .into_iter()
        .map(|packet| packet.version as u64)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            super::parse("D2FE28"),
            Packet {
                version: 6,
                content: PacketContent::Literal(2021)
            }
        );
        assert_eq!(
            super::parse("38006F45291200"),
            Packet {
                version: 1,
                content: PacketContent::LessThan(vec![
                    Packet {
                        version: 6,
                        content: PacketContent::Literal(10)
                    },
                    Packet {
                        version: 2,
                        content: PacketContent::Literal(20)
                    },
                ])
            }
        );
        assert_eq!(
            super::parse("9C0141080250320F1802104A08"),
            Packet {
                version: 4,
                content: PacketContent::EqualTo(vec![
                    Packet {
                        version: 2,
                        content: PacketContent::Sum(vec![
                            Packet {
                                version: 2,
                                content: PacketContent::Literal(1)
                            },
                            Packet {
                                version: 4,
                                content: PacketContent::Literal(3)
                            },
                        ])
                    },
                    Packet {
                        version: 6,
                        content: PacketContent::Product(vec![
                            Packet {
                                version: 0,
                                content: PacketContent::Literal(2)
                            },
                            Packet {
                                version: 2,
                                content: PacketContent::Literal(2)
                            },
                        ])
                    }
                ])
            }
        );
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1("8A004A801A8002F478"), 16);
        assert_eq!(super::part1("620080001611562C8802118E34"), 12);
        assert_eq!(super::part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("C200B40A82"), 3);
        assert_eq!(super::part2("04005AC33890"), 54);
        assert_eq!(super::part2("880086C3E88112"), 7);
        assert_eq!(super::part2("CE00C43D881120"), 9);
        assert_eq!(super::part2("D8005AC2A8F0"), 1);
        assert_eq!(super::part2("F600BC2D8F"), 0);
        assert_eq!(super::part2("9C005AC2F8F0"), 0);
        assert_eq!(super::part2("9C0141080250320F1802104A08"), 1);
    }
}
