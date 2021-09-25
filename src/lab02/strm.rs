use std::collections::BTreeMap;

pub fn findRepeatedSubstr<'a>(contents: &'a String, n: usize,
    repeated: &mut BTreeMap<&'a str, (usize, Vec<usize>)>)
{
    for i in 0..contents.len() - (n - 1)
    {
        let slice: &str = &contents[i..i + n];
        if repeated.get(slice) == None
        {
            let mut lastIndex: usize = i;
            for j in i + 1..contents.len() - (n - 1)
            {
                if contents[j..j + n] == *slice
                {
                    repeated.entry(slice).and_modify(|v| {
                        v.0 += 1;
                        v.1.push(j - lastIndex);
                    }).or_insert((2, vec![0, j - lastIndex]));
                    lastIndex = j;
                }
            }
        }
    }
}
