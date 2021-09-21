pub fn run(contents: &mut String)
{
    const PUNCT: &str = ".,:;()[]{}¡!¿?-…";
    contents.retain(|c| !(c.is_whitespace() || PUNCT.contains(c)));
    println!("{}", contents);
}
