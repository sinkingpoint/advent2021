use libadvent::*;

#[derive(Debug)]
enum PacketType {
    Literal,
    Operator(u32, Vec<Packet>)
}

#[derive(Debug)]
struct Packet {
    ty: PacketType,
    version: u32,
    value: u128
}

fn get_bit(n: usize, bytes: &[u8]) -> u8 {
    let arr_index = n / 8;
    let bit_index = 7 - (n % 8);
    let byte = if arr_index >= bytes.len() {
        0
    }
    else {
        bytes[arr_index]
    };
    return (byte & (1 << bit_index)) >> bit_index;
}

fn decode_input(s: &[char]) -> Vec<u8> {
    let mut i = 0;
    let mut out = Vec::new();

    while i < s.len() {
        out.push(parse_to_base(&format!("{}{}", s[i], s[i+1]), 16) as u8);
        i += 2;
    }

    out
}

fn get_sequence(start: usize, len: usize, bytes: &[u8]) -> u32 {
    let mut out = 0;
    for i in 0..len {
        out = (out << 1) | get_bit(start + i, bytes) as u32;
    }

    out
}

fn parse_packet(start_index: usize, bytes: &[u8]) -> (Packet, usize) {
    let initial_start = start_index;
    let version = get_sequence(start_index, 3, bytes);
    let id = get_sequence(start_index+3, 3, bytes);
    
    let mut start_index = start_index + 6;
    let packet = match id {
        4 => {
            let mut next = get_sequence(start_index, 5, bytes) as u128;
            let mut out: u128 = 0;
            while next >> 4 & 0x1 == 1 {
                out = (out << 4) | (next & 0b01111);
                start_index += 5;
                next = get_sequence(start_index, 5, bytes) as u128;
            }
            out = (out << 4) | (next & 0b01111);
            start_index += 5;
            Packet {
                ty: PacketType::Literal,
                value: out,
                version
            }
        },
        i => {
            let length_type_id = get_bit(start_index, bytes);
            let mut packets = Vec::new();
            let mut version_sum = version;
            if length_type_id == 0 {
                // We're parsing by bits
                let mut num_bits = get_sequence(start_index + 1, 15, bytes) as usize;
                start_index += 16;

                while num_bits > 0 {
                    let (p, bits) = parse_packet(start_index, bytes);
                    start_index += bits;
                    num_bits -= bits;
                    version_sum += p.version;
                    packets.push(p);
                }
            }
            else {
                let num_packets = get_sequence(start_index + 1, 11, bytes);
                start_index += 12;

                for _ in 0..num_packets {
                    let (p, bits) = parse_packet(start_index, bytes);
                    start_index += bits;
                    version_sum += p.version;
                    packets.push(p);
                }
            }

            let value = match i {
                0 => {
                    packets.iter().map(|p| p.value).sum()
                },
                1 => {
                    packets.iter().map(|p| p.value).product()
                },
                2 => {
                    packets.iter().map(|p| p.value).min().unwrap()
                },
                3 => {
                    packets.iter().map(|p| p.value).max().unwrap()
                },
                5 => {
                    if packets[0].value > packets[1].value {
                        1
                    }
                    else {
                        0
                    }
                },
                6 => {
                    if packets[0].value < packets[1].value {
                        1
                    }
                    else {
                        0
                    }
                },
                7 => {
                    if packets[0].value == packets[1].value {
                        1
                    }
                    else {
                        0
                    }
                },
                _ => panic!()
            };

            Packet{
                ty: PacketType::Operator(id, packets),
                value,
                version: version_sum
            }
        }
    };

    if start_index >= bytes.len() * 8 {
        start_index = bytes.len() * 8;
    }

    return (packet, start_index - initial_start);
}

fn main() {
    let input = must_read_input();
    let bytes = decode_input(&input.chars().collect::<Vec<char>>());

    let out_packet = parse_packet(0, &bytes).0;
    println!("Version: {:?}", out_packet.version);
    println!("Value: {:?}", out_packet.value);
}
