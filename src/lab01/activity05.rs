mod frecuencias;

use std::collections::BTreeMap;

pub fn run(contents: &String)
{
    let frequencies: BTreeMap<char, u32> =
        frecuencias::getFrequencies(&contents);
    println!("Frequencies: {:#?}", frequencies);

    let mut maxPairs: Vec<(char, u32)> = Vec::new();
    for _ in 0..5
    {
        let mut currentMax: (char, u32) = ('\0', 0);
        for (key, val) in &frequencies
        {
            if maxPairs.is_empty()
            {
                if *val > currentMax.1
                {
                    currentMax = (*key, *val);
                }
            }
            else
            {
                if  *val > currentMax.1 &&
                    *key != maxPairs[maxPairs.len() - 1].0 &&
                    *val <= maxPairs[maxPairs.len() - 1].1
                {
                    currentMax = (*key, *val);
                }
            }
        }
        maxPairs.push(currentMax);
    }
    println!("5 more frequent: {:?}", maxPairs);
}
