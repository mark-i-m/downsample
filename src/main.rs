//! Down-sample a file by sampling random lines from the input.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

const USAGE: &str = "Usage: ./bin in out rate";

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let infile = args.next().expect(USAGE);
    let outfile = args.next().expect(USAGE);
    let rate = args
        .next()
        .expect(USAGE)
        .parse::<usize>()
        .expect("Sample rate should be usize N: 1/N");

    println!("Downsampling {} -> {} @ {}", infile, outfile, rate);

    let rng = StdRng::seed_from_u64(0);
    let unif = Uniform::new_inclusive(1, rate);

    let inf = File::open(infile)?;
    let reader = BufReader::with_capacity(10 << 20, inf); // 10MB buffer

    let outf = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(outfile)?;
    let mut writer = BufWriter::new(outf);

    let mut lines = reader.lines().skip(2).map(|l| {
        l.expect("unable to read")
            .parse::<usize>()
            .expect("unable to parse as int")
    });

    let mut prev = lines.next().unwrap();

    for line in lines
        .map(|x| {
            let d = x - prev;
            prev = x;
            d
        })
        .filter(|&d| d > 0)
        .zip(rng.sample_iter(unif))
        .filter_map(|(line, r)| if r == 1 { Some(line) } else { None })
    {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}
