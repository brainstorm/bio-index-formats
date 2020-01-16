/// BinsForRange returns the list of bins that may overlap with the zero-based region
/// defined by [start, end). The minShift and depth parameters control the minimum interval width
/// and number of binning levels, respectively.
// This is the generalized CSI implementation from Google's htsget
pub fn bins_for_range(start: u32, mut end: u32, min_shift: u32, depth: u32) -> Vec<u32> {
    let max_width = maximum_bin_width(min_shift, depth);
    let mut bins: Vec<u32> = Vec::new();

    if end == 0 || end > max_width { end = max_width; }
//    if end <= start { return Option(None) }
//    if start > maxWidth { return Option(None) }

    // This is derived from the C examples in the CSI index specification.
    end -= 1;
    let mut l = 0;
    let mut t = 0;
    let mut s = min_shift + depth * 3;

     while l <= depth {
        let b = t + (start >> s);
        let e = t + (end >> s);
        let i = b;

        while i <= e {
            bins.push(i)
        }

        s -= 3;
        t += 1 << (l * 3);
        l += 1;
    }
    return bins;
}

fn maximum_bin_width(min_shift: u32, depth: u32) -> u32 {
    return 1 << min_shift + depth * 3;
}


// This is the SAM spec C code, oxidized by hand.
// See: https://gist.github.com/brainstorm/f76da194a03730a98e23766134d54d7b
pub fn reg2bins(beg: u64, mut end: u64) -> Vec<u64> {
    let mut i: u64 = 0;
    let mut k: u64;
    end -= 1;
    i = i + 1;
    let mut list = Vec::<u64>::new();

    k = 1 + (beg >> 26);
    while k <= 1 + (end >> 26) { i += 1; list.push(k); k += 1 }

    k = 9 + (beg >> 23);
    while k <= 9 + (end >> 23) { i += 1; list.push(k); k += 1 }

    k = 73 + (beg >> 20);
    while k <= 73 + (end >> 20) { i += 1; list.push(k); k += 1 }

    k = 585 + (beg >> 17);
    while k <= 585 + (end >> 17) { i += 1; list.push(k); k += 1 }

    k = 4681 + (beg >> 14);
    while k <= 4681 + (end >> 14) { i += 1; list.push(k); k += 1 }

    return list;
}

#[cfg(test)]
mod tests {
    use crate::csi::reg2bins;

    #[test]
    fn csi_test() {
        reg2bins(3, 10);
    }

//
//    #[test]
//    fn csi_test() {
//        bins_for_range(3, 10,14, 5);
//    }
}