use nom::{IResult};
//use nom::bytes::streaming::{tag, take};
//use nom::number::streaming::{le_u32};
use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_u32};

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

pub fn parse_bai(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, _) = parse_magic(input)?;
    let (input, n_refs) = le_u32(input)?;
    //let (input, _) = take(n_refs)(input)?;  // Jump through until n_ref target offset
    //let (input, refs) = le_u32(input)?;
    Ok((input, n_refs))
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

//    #[test]
//    fn bai() {
//        let res = parse_bai(BAI_FILE);
//        assert_eq!(Ok((le_u32(res), 56)), res);
//    }
}