use bit_vec::BitVec;
const CAPACITY: usize = 512;

struct Packet {
    buffer: BitVec<u8>,
    pos: u16,
}

impl Packet {
    fn new() -> Self {
        Self {
            buffer: BitVec::from_elem_general(CAPACITY, false),
            pos: 0,
        }
    }

    fn get(&self, pos: usize) -> Option<bool> {
        self.buffer.get(pos)
    }
}

impl Iterator for Packet {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = &mut self.pos;
        let usized_pos: usize = (*pos).into();
        let ret = self.buffer.get(usized_pos);
        if usized_pos < CAPACITY {
            *pos = *pos + 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_iterator_test() {
        let packet = Packet::new();
        println!("{:?}", packet.get(1000));
        let len = packet.fold(0, |mut acc, x| {
            if x {
                acc = acc + 1;
            }
            acc
        });
        assert_eq!(len, 0);
    }
}
