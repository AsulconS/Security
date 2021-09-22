pub fn run(contents: &mut String)
{
    let nc: String = contents.chars().map(
        |c| match c {
            'J' | 'H' => 'I',
            'j' | 'h' => 'i',
            '\u{00D1}' => 'N',
            '\u{00F1}' => 'n',
            'K' => 'L',
            'k' => 'l',
            'U' | 'W' => 'V',
            'u' | 'w' => 'v',
            'Y' => 'Z',
            'y' => 'z',
            _ => c
    }).collect();
    contents.replace_range(.., nc.as_str());
    println!("{}", contents);
}
