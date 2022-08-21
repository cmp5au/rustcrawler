use std::io::Result;
use std::env;
use crawler::Crawler;

mod tmatch;
mod tournament;
mod crawler;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let n: usize;

    if args.len() > 1 {
        n = match args[1].parse() {
            Ok(n) => n,
            Err(_) => 5,
        };
    } else { n = 5 }

    let crawler = Crawler::new("https://www.tennis.com/scores/");
    
    for tournament in crawler.get_tournaments() {
        println!("\n\n{}\n", tournament.name);
        for tmatch_result in tournament.top_n_matches(n) {
            match tmatch_result {
                Ok(tmatch) => println!("{}", tmatch),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    println!("\n");
    
    Ok(())
}