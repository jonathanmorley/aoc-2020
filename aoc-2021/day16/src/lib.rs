use std::iter;

use bitvec::prelude::*;
use bitvec::bits;
use nom::bytes::complete::{tag, take};
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::{many0, many_till};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use nom::multi::many_m_n;
use nom_bitvec::BSlice;

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Sum { version: u8, packets: Vec<Packet> },
    Product { version: u8, packets: Vec<Packet> },
    Minimum { version: u8, packets: Vec<Packet> },
    Maximum { version: u8, packets: Vec<Packet> },
    Literal { version: u8, value: u64 },
    GreaterThan { version: u8, packets: Vec<Packet> },
    LessThan { version: u8, packets: Vec<Packet> },
    EqualTo { version: u8, packets: Vec<Packet> },
}

impl Packet {
    fn version(&self) -> u8 {
        match self {
            Packet::Sum { version, .. }
            | Packet::Product { version, .. }
            | Packet::Minimum { version, .. }
            | Packet::Maximum { version, .. }
            | Packet::Literal { version, .. }
            | Packet::GreaterThan { version, .. }
            | Packet::LessThan { version, .. }
            | Packet::EqualTo { version, .. } => *version,
        }
    }

    fn all_packets(&self) -> Vec<&Packet> {
        iter::once(self)
            .chain(match self {
                Packet::Literal { .. } => vec![],
                Packet::Sum { packets, .. }
                | Packet::Product { packets, .. }
                | Packet::Minimum { packets, .. }
                | Packet::Maximum { packets, .. }
                | Packet::GreaterThan { packets, .. }
                | Packet::LessThan { packets, .. }
                | Packet::EqualTo { packets, .. } => {
                    packets.iter().flat_map(Packet::all_packets).collect()
                }
            })
            .collect()
    }

    fn result(&self) -> u64 {
        match self {
            Packet::Sum { packets, .. } => packets.iter().map(Packet::result).sum(),
            Packet::Product { packets, .. } => packets.iter().map(Packet::result).product(),
            Packet::Minimum { packets, .. } => packets.iter().map(Packet::result).min().unwrap(),
            Packet::Maximum { packets, .. } => packets.iter().map(Packet::result).max().unwrap(),
            Packet::Literal { value, .. } => *value,
            Packet::GreaterThan { packets, .. } => if packets[0].result().gt(&packets[1].result()) { 1 } else { 0 },
            Packet::LessThan { packets, .. } => if packets[0].result().lt(&packets[1].result()) { 1 } else { 0 },
            Packet::EqualTo { packets, .. } => if packets[0].result().eq(&packets[1].result()) { 1 } else { 0 },
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
        map(take(3usize), |bits: BSlice<Msb0, u8>| bits.0.load_be())
    ))(input)
}

fn parse_length(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Length> {
    alt((
        map(preceded(tag(BSlice(bits![0])), take(15usize)), |bits: BSlice<Msb0, u8>| {
            Length::Bits(bits.0.load_be())
        }),
        map(preceded(tag(BSlice(bits![1])), take(11usize)), |bits: BSlice<Msb0, u8>| {
            Length::Packets(bits.0.load_be())
        })
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
        (version, 4) => map(parse_literal, |value| Packet::Literal { version, value })(input),
        (version, type_id) => {
            let (input, length) = parse_length(input)?;
            let (input, packets) = parse_packets(input, length)?;

            Ok((input, match type_id {
                0 => Packet::Sum { version, packets },
                1 => Packet::Product { version, packets },
                2 => Packet::Minimum { version, packets },
                3 => Packet::Maximum { version, packets },
                5 => Packet::GreaterThan { version, packets },
                6 => Packet::LessThan { version, packets },
                7 => Packet::EqualTo { version, packets },
                _ => unreachable!()
            }))
        }
    }
}

fn parse_packets(input: BSlice<Msb0, u8>, length: Length) -> IResult<BSlice<Msb0, u8>, Vec<Packet>> {    
    match length {
        Length::Bits(l) => {
            let (input, packet_data) = take(l as usize)(input)?;
            let (_, packets) = many0(parse_packet)(packet_data)?;
            Ok((input, packets))
        },
        Length::Packets(l) => many_m_n(l as usize, l as usize, parse_packet)(input)
    }
}

pub fn parse(input: &str) -> Packet {
    let bytes = hex::decode(&input.trim_end()).unwrap();
    let bits = BSlice(BitSlice::<Msb0, _>::from_slice(&bytes).unwrap());
    parse_packet(bits).unwrap().1
}

pub fn part1(input: &str) -> u64 {
    parse(input)
        .all_packets()
        .into_iter()
        .map(|packet| packet.version() as u64)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input).result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            super::parse("D2FE28"),
            Packet::Literal {
                version: 6,
                value: 2021
            }
        );
        assert_eq!(
            super::parse("38006F45291200"),
            Packet::LessThan {
                version: 1,
                packets: vec![
                    Packet::Literal {
                        version: 6,
                        value: 10
                    },
                    Packet::Literal {
                        version: 2,
                        value: 20
                    }
                ]
            }
        );
        assert_eq!(
            super::parse("9C0141080250320F1802104A08"),
            Packet::EqualTo {
                version: 4,
                packets: vec![
                    Packet::Sum {
                        version: 2,
                        packets: vec![
                            Packet::Literal {
                                version: 2,
                                value: 1
                            },
                            Packet::Literal {
                                version: 4,
                                value: 3
                            }
                        ]
                    },
                    Packet::Product {
                        version: 6,
                        packets: vec![
                            Packet::Literal {
                                version: 0,
                                value: 2,
                            },
                            Packet::Literal {
                                version: 2,
                                value: 2
                            }
                        ]
                    }
                ]
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
