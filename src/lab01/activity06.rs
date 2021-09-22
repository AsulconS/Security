use std::collections::BTreeMap;

pub fn run(contents: &String)
{
    let mut triplets:
        BTreeMap<&str, (u32, Vec<usize>)> = BTreeMap::new();
    for i in 0..contents.len() - 2
    {
        let slice: &str = &contents[i..i + 3];
        if triplets.get(slice) == None
        {
            let mut lastIndex: usize = i;
            triplets.insert(slice, (1, vec![0]));
            for j in i + 1..contents.len() - 2
            {
                if &contents[j..j + 3] == slice
                {
                    triplets.entry(slice).and_modify(|v| {
                        v.0 += 1;
                        v.1.push(j - lastIndex);
                    });
                    lastIndex = j;
                }
            }
        }
    }
    println!("{:?}", triplets);
}
