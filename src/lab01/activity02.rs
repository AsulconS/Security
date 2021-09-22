pub fn run(contents: &mut String)
{
    let nc: String = contents.chars().map(
        |c| match c {
            'Á' => 'A',
            'á' => 'a',
            'É' => 'E',
            'é' => 'e',
            'Í' => 'I',
            'í' => 'i',
            'Ó' => 'O',
            'ó' => 'o',
            'Ú' => 'U',
            'ú' => 'u',
            _ => c
    }).collect();
    contents.replace_range(.., nc.as_str());
    println!("{}", contents);
}
