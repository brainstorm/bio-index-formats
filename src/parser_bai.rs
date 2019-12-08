use nom::{IResult};
use nom::bytes::complete::{tag, take};
use nom::number::streaming::{le_i32};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BAI {
    pub magic: String,
    pub n_ref: i32,
    pub refs: ListIndexes,
    pub n_no_coor: u64,
}

pub fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, magic) = tag("BAI")(input)?;
    Ok((input, magic))
}

pub fn parse_n_ref(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, n_ref) = le_i32(&input[5..5])?;
    Ok((input, n_ref))
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ListIndexes {
    pub n_bin: i32,
    pub bins: Bin,
    pub n_intv: i32,
    pub ioffset: u64,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Bin {
    pub bin: u32,
    pub n_chunk: i32,
    pub chunks: Chunk,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Chunk {
    pub chunk_beg: u64,
    pub chunk_end: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");

    #[test]
    fn magic() {
        let field = &BAI_FILE[..3];
        let res = parse_magic(field);
        assert_eq!(Ok((&b""[..], &b"BAI"[..])), res);
    }

    #[test]
    fn n_refs() {
        let field = &BAI_FILE[5..5];
        let res = parse_n_ref(field);
        assert_eq!(Ok((le_i32(field), &b"32")), res);
    }
}