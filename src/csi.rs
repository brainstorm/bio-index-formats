// This is the SAM spec C code, oxidized by hand.
// See: https://gist.github.com/brainstorm/f76da194a03730a98e23766134d54d7b
pub fn reg2bins(beg: u32, mut end: u32) -> Vec<u32> {
    let mut k: u32;
    end -= 1;

    let mut list = Vec::<u32>::new();

    k = 1 + (beg >> 26);
    while k <= 1 + (end >> 26) { list.push(k); k += 1 }
    k = 9 + (beg >> 23);
    while k <= 9 + (end >> 23) { list.push(k); k += 1 }
    k = 73 + (beg >> 20);
    while k <= 73 + (end >> 20) { list.push(k); k += 1 }
    k = 585 + (beg >> 17);
    while k <= 585 + (end >> 17) { list.push(k); k += 1 }
    k = 4681 + (beg >> 14);
    while k <= 4681 + (end >> 14) { list.push(k); k += 1 }

    return list;
}

// Instead of returning a list with the R-Tree path traversal, returns
// the bin we are interested in directly
pub fn reg2bin(beg: u32, mut end: u32) -> u32
{
    end -= 1;
    if beg>>14 == end>>14 { return ((1<<15)-1)/7 + (beg>>14) }
    if beg>>17 == end>>17 { return ((1<<12)-1)/7 + (beg>>17) }
    if beg>>20 == end>>20 { return ((1<<9)-1)/7 + (beg>>20) }
    if beg>>23 == end>>23 { return ((1<<6)-1)/7 + (beg>>23) }
    if beg>>26 == end>>26 { return ((1<<3)-1)/7 + (beg>>26) }
    
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::csi::reg2bins;

    #[test]
    fn csi_test() {
        let res = reg2bins(3, 10);
        let expected = [1, 9, 73, 585, 4681];
        assert_eq!(res, expected);
    }

// Not complying with SAM spec, page 15, section 4.2.1 since I don't agree with
// the (signed) types they use, see: https://github.com/samtools/hts-specs/pull/460
//    #[test]
//    fn csi_test_4680() {
//        // As seen in page 15 of SAMv1 spec
//        let res = reg2bins(-1, 0);
//        let expected = [4680];
//        assert_eq!(res, expected);
//    }
}