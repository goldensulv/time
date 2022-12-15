use clap::Parser;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{self, Duration};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
struct Cli {
    command: String,
    #[arg(short = 't', long = "times", default_value_t = 2)]
    times: i32,
    #[arg(short = 'o', long = "output", default_value_t = false)]
    output: bool,
}

fn parse_time(time: Duration) -> String {
    format!(
        "{} days, {:>2} hrs {:>2} min {:>2} sec {:>3} ms {:>3} us {:>3} ns",
        time.as_secs() / 86400,
        time.as_secs() / 3600,
        time.as_secs() / 60,
        time.as_secs() % 60,
        time.subsec_millis(),
        time.subsec_micros() % 1000,
        time.subsec_nanos() % 1000
    )
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut output = Command::new(&args.command);
    let mut average_time = time::Duration::from_secs(0);
    let mut error = false;
    if !args.output {
        output.stdout(Stdio::null()).stderr(Stdio::null());
    }
    'outer: for _ in 0..args.times {
        stdout.reset()?;
        stdout.flush()?;
        let start = time::Instant::now();
        let status = output.spawn();
        match status {
            Ok(mut child) => {
                let result = child.wait();
                if let Ok(status) = result {
                    let time = start.elapsed();
                    average_time += time;
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
                    println!("\nProcess finished. {}", status);
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                    println!("Time: {}", parse_time(time));
                }
            }
            Err(e) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                println!("Error: {}", e);
                error = true;
                break 'outer;
            }
        }
    }

    if !error {
        average_time /= args.times as u32;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        println!("\nAverage time for {} runs: {}", args.times, parse_time(average_time));
    }

    stdout.reset()?;
    stdout.flush()?;

    Ok(())
}
