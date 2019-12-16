//use crate::csi::{ bins_for_range };

use nom::{IResult};
//use nom::bytes::streaming::{tag, take};
//use nom::number::streaming::{le_u32};
use nom::bytes::complete::{ tag };
use nom::number::complete::{ le_u32, le_u64 };
use std::convert::TryInto;


#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BAI {
    pub magic: &'static[u8;4],
    pub n_ref: u32,
    pub refs: ListIndexes,
    pub n_no_coor: u64,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ListIndexes {
    pub n_bin: u32,
    pub bins: Bin,
    pub n_intv: u32,
    pub ioffset: u64,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Bin {
    pub bin_id: u32,
    pub n_chunk: u32,
    pub chunk: ChunkPos,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ChunkPos {
    pub chunk_beg: u64,
    pub chunk_end: u64,
}


pub fn parse_bai(input: &'static[u8]) -> IResult<&[u8], BAI> {
    let (input, magic) = parse_magic(input)?;
    let (input, n_ref) = le_u32(input)?;
    let (input, refs) = parse_list_indexes(input)?;
    let (input, n_no_coor) = le_u64(input)?;
    Ok((input, BAI{ magic, n_ref, refs, n_no_coor }))
}

pub fn parse_list_indexes(input: &[u8]) -> IResult<&[u8], ListIndexes> {
    //let bins_range = bins_for_range(2,3,14,5);
    let (input, n_bin) = le_u32(input)?;
    let (input, bins) = parse_bin(input)?;
    let (input, n_intv) = le_u32(input)?;
    let (input, ioffset) = le_u64(input)?;
    Ok((input, ListIndexes { n_bin, bins, n_intv, ioffset }))
}

pub fn parse_bin(input: &[u8]) -> IResult<&[u8], Bin> {
    let (input, bin_id) = le_u32(input)?;
    let (input, n_chunk) = le_u32(input)?;
    let (input, chunk) = parse_chunk(input)?;
    Ok((input, Bin { bin_id, n_chunk, chunk }))
}

pub fn parse_chunk(input: &[u8]) -> IResult<&[u8], ChunkPos> {
    let (input, chunk_beg) = le_u64(input)?;
    let (input, chunk_end) = le_u64(input)?;
    Ok((input, ChunkPos { chunk_beg, chunk_end }))
}

pub fn parse_magic(input: &'static[u8]) -> IResult<&[u8], &'static[u8;4]> {
    let (input, magic) = tag("BAI\x01")(input)?;
    Ok((input, magic.try_into().expect("wrong header length")))
}


#[cfg(test)]
mod tests {
    use super::*;

    const BAI_FILE: &'static [u8] = include_bytes!("../tests/data/htsnexus_test_NA12878.bam.bai");

//    #[test]
//    fn magic() {
//        let field = &BAI_FILE[..4];
//        let res = parse_magic(field);
//        assert_eq!(Ok((&b""[0..4], &b"BAI\x01"[0..4])), res);
//    }

//    #[test]
//    fn bai() {
//        let res = parse_bai(BAI_FILE);
//        assert_eq!(Ok((le_u32(res), 56)), res);
//    }
}