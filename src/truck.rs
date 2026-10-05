use std::io::{self, Write};
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn accelerate(duration: f64) {
    const PROGRESS_BAR_LENGTH: usize = 20;

    let start = Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs_f64();
        let progress = (elapsed / duration).min(1.0);

        let filled = (progress * PROGRESS_BAR_LENGTH as f64).round() as usize;
        let empty = PROGRESS_BAR_LENGTH - filled;

        let progress_bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

        let percentage = (progress * 100.0).round() as u32;

        print!(
            "\r🚚 [{}] {:>3}%  {:.1}s",
            progress_bar,
            percentage,
            elapsed.min(duration)
        );

        io::stdout().flush().unwrap();

        if progress >= 1.0 {
            break;
        }

        sleep(Duration::from_millis(100)).await;
    }

    println!();
    println!("\n🏁 0-60 mph in {:.1} seconds", duration);
}
