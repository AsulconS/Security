fn tor(c: u8) -> i32
{
    return c as i32 - 65;
}

pub fn decypher(contents: &String, key: &String, n: i32) -> String
{
    let cbytes: Vec<u8> = contents.as_bytes().to_vec();
    let kbytes: Vec<u8> = key.as_bytes().to_vec();
    let mut res: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    for c in cbytes
    {
        let val: i32 = (tor(c) + (n - tor(kbytes[i % key.len()])) % n) % n;
        res.push(65 + val as u8);
        i += 1;
    }
    return String::from_utf8(res).unwrap();
}
