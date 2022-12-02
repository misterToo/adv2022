use eyre::{Result, Context};
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("res/1.txt").wrap_err("opening 1.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut counts = Vec::new();
    let mut running_count = 0;


    for line in buf_reader.lines() {
        let line = line.wrap_err("getting next line")?;
        if line.is_empty() {
            counts.push(running_count);
            running_count = 0;
        } else {
            running_count += line.parse::<u32>()?;
        }
    }

    let (a,b,c) = counts.iter().fold((0,0,0), |(a,b,c), &it| {
        println!("{it}");

        if a <= b && a <= c && a <= it{
            (it, b, c)
        } else if b <= a && b <= c && b <= it {
            (a , it , c  )
        } else if c <= a && c <= b && c <= it {
            (a, b , it)
        } else {
            ( a , b,c)
        }

    });

    println!("{:#?}", a+b+c);

    //println!("{counts:#?}");


    Ok(())
}
