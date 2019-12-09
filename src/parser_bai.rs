use nom::{IResult};
use nom::bytes::streaming::{tag, take};
use nom::number::streaming::{le_u32};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BAI {
    pub magic: String,
    pub n_ref: i32,
    pub refs: ListIndexes,
    pub n_no_coor: u64,
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

pub fn parse_bai(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let _magic = parse_magic(input)?;
    // XXX: Should be le_u32, see: https://gitter.im/Geal/nom?at=5deda6ff46397c721cafdee7
    let (input, n_refs) = le_u32(input)?;
    let (input, _between_n_refs_and_bin) = take(n_refs)(input)?;
    let (input, refs) = le_u32(input)?;
    dbg!(refs);
    Ok((input, _between_n_refs_and_bin))
}

pub fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, magic) = tag("BAI\x01")(input)?;
    Ok((input, magic))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");

    #[test]
    fn magic() {
        let field = &BAI_FILE[..4];
        let res = parse_magic(field);
        assert_eq!(Ok((&b""[..], &b"BAI\x01"[..])), res);
    }

    #[test]
    fn bai() {
        let res = parse_bai(BAI_FILE);
        assert_eq!(Ok((le_u32(res), 56)), res);
    }
}