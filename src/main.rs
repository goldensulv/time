use clap::Parser;
use std::io::{self, Write};
use std::process::Command;
use std::time::{self, Duration};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
struct Cli {
    command: String,
    #[arg(short = 't', long = "times", default_value_t = 1)]
    times: i32,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut output = Command::new(&args.command);
    let mut average_time: Duration = time::Duration::from_secs(0);
    let mut error = false;
    'outer: for _ in 0..args.times {
        stdout.reset()?;
        stdout.flush()?;
        let start = time::Instant::now();
        let status = output.spawn();
        match status {
            Ok(mut child) => {
                let result = child.wait();
                if let Ok(status) = result {
                    let time = time::Instant::now().duration_since(start);
                    average_time += time;
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
                    println!("\nProcess finished. {}", status);
                    let time = format!(
                        "{} days, {:0>2} hrs {:0>2} min {:0>2} sec {:>3} ms {:>3} us {:>3} ns",
                        time.as_secs() / 86400,
                        time.as_secs() / 3600,
                        time.as_secs() / 60,
                        time.as_secs() % 60,
                        time.subsec_millis(),
                        time.subsec_micros() % 1000,
                        time.subsec_nanos() % 1000
                    );
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                    println!("Time: {}", time);
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
        let time = format!(
            "{} days, {:0>2} hrs {:0>2} min {:0>2} sec {:>3} ms {:>3} us {:>3} ns",
            average_time.as_secs() / 86400,
            average_time.as_secs() / 3600,
            average_time.as_secs() / 60,
            average_time.as_secs() % 60,
            average_time.subsec_millis(),
            average_time.subsec_micros() % 1000,
            average_time.subsec_nanos() % 1000
        );
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        println!("\nAverage time for {} runs: {}", args.times, time);
    }

    stdout.reset()?;
    stdout.flush()?;

    Ok(())
}
