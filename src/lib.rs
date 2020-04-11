const CHUNK_BITS: u32 = 7;
const CHUNK_MASK: u32 = (1 << CHUNK_BITS) - 1;
const CONT_BIT: u8 = 1 << CHUNK_BITS;

pub fn encode(data: &[u32], mut sink: impl FnMut(u8)) {
    for value in data {
        let mut value = *value;
        loop {
            let mut byte = (value & CHUNK_MASK) as u8;
            value >>= CHUNK_BITS;
            if value != 0 {
                byte |= CONT_BIT
            }
            sink(byte);
            if value == 0 {
                break;
            }
        }
    }
}

pub fn decode(data: &[u8], mut sink: impl FnMut(u32)) {
    let mut value = 0;
    for &byte in data.iter() {
        value <<= CHUNK_BITS;
        value |= byte as u32 & CHUNK_MASK;
        if byte & CONT_BIT != CONT_BIT {
            sink(value);
            value = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(xs: &[u32]) -> bool {
        let mut packed: Vec<u8> = Vec::new();
        encode(xs, |byte| packed.push(byte));
        let mut decoded: Vec<u32> = Vec::new();
        decode(&packed, |i| decoded.push(i));
        xs == decoded.as_slice()
    }

    quickcheck::quickcheck! {
        fn is_encoding(xs: Vec<u32>) -> bool {
            id(&xs)
        }
    }
}
