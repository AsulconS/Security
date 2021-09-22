use std::collections::BTreeMap;

pub fn getFrequencies(contents: &String) -> BTreeMap<char, u32>
{
    let mut frequencies: BTreeMap<char, u32> = BTreeMap::new();
    for c in 'A'..='Z'
    {
        frequencies.insert(c, 0);
    }
    for c in contents.chars()
    {
        frequencies.insert(c, frequencies[&c] + 1);
    }
    frequencies
}
