pub fn computeDTable(data: &Vec<usize>) -> [(usize, usize); 21]
{
    let mut count: [(usize, usize); 21] = [(0, 0); 21];
    let mut c: usize = 0;
    for _iref in count.iter_mut()
    {
        _iref.0 = c;
        c += 1;
    }

    println!("Divisor Table:");
    print!("{:^8}", ' ');
    for i in 2..=20 { print!("{:^3}", i); }
    println!();
    for iref in data.iter()
    {
        print!("{:^8}", iref);
        for i in 2..=20
        {
            if *iref % i == 0
            {
                count[i].1 += 1;
                print!("{:^3}", "X");
            }
            else { print!("{:^3}", ' '); }
        }
        println!();
    }
    print!("{:^8}", "Total:");
    for i in 2..=20 { print!("{:^3?}", count[i].1); }
    println!();

    return count;
}
