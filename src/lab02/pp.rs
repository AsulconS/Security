pub fn preprocess(contents: &mut String)
{
    const PUNCT: &str = ".,:;()[]{}¡!¿?-…";
    contents.retain(|c| !(c.is_whitespace() || PUNCT.contains(c)));
    contents.replace_range(.., contents.to_uppercase().as_str());
    let nc: String = contents.chars().map(
        |c| match c {
            'Ñ' => 'N',
            'Á' => 'A',
            'É' => 'E',
            'Í' => 'I',
            'Ó' => 'O',
            'Ú' => 'U',
            _ => c
    }).collect();
    contents.replace_range(.., nc.as_str());
}
