/// BinsForRange returns the list of bins that may overlap with the zero-based region
/// defined by [start, end). The minShift and depth parameters control the minimum interval width
/// and number of binning levels, respectively.
pub fn bins_for_range(start: u32, mut end: u32, min_shift: u32, depth: u32) -> Vec<u32> {
    let max_width = maximum_bin_width(min_shift, depth);
    let mut bins: Vec<u32> = Vec::new();

    if end == 0 || end > max_width { end = max_width; }
//    if end <= start { return None }
//    if start > maxWidth { return None }

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