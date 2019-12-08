use nom::{IResult, Err};
use nom::bytes::complete::tag;
//use nom::number::streaming::{le_u32, le_u64};

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
    fn bai_magic() {
        let magic = parse_magic(&BAI_FILE[..3]);
        assert_eq!(Ok((&[][..], b"BAI")), magic);
    }
}