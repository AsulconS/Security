pub fn run(contents: &mut String)
{
    contents.replace_range(.., contents.to_uppercase().as_str());
    println!("{}", contents);
}
