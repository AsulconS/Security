pub fn run(contents: &mut String)
{
    let mut i: usize = 0;
    while i < contents.len()
    {
        contents.replace_range(i..i + 4, "AQUÍ");
        i += 25;
    }
    let nextMul: usize = contents.chars().count() -
                         contents.chars().count() % 4 + 4;
    contents.push_str((contents.chars().count()..nextMul)
        .map(|_| "X").collect::<String>().as_str());
    println!("{}", contents);
}
