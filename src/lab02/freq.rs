use std::collections::BTreeMap;

pub fn getAEOFrequencies(contents: &String) -> [(char, usize); 3]
{
    let mut freq: BTreeMap<char, usize> = BTreeMap::new();
    for c in contents.chars()
    {
        if freq.get(&c) == None
        {
            freq.insert(c, 0);
        }
        freq.insert(c, freq[&c] + 1);
    }
    let mut vfreq: Vec<(char, usize)> = Vec::new();
    let mut higher: [(char, usize); 3] = [('\0', 0); 3];
    for iref in freq
    {
        vfreq.push(iref);
    }
    vfreq.sort_by(|l: &(char, usize), r: &(char, usize)| r.1.cmp(&l.1));
    higher[0] = vfreq[0];
    higher[1] = vfreq[1];
    higher[2] = vfreq[2];
    return higher;
}
