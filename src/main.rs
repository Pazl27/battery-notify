use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

fn get_battery_percentage() -> Result<f32> {
    let mut file = File::open("/sys/class/power_supply/BAT1/capacity")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let percentage: f32 = contents.trim().parse().unwrap();
    Ok(percentage)
}

fn send_notification(summary: &str, body: &str) -> Result<()> {
    Command::new("notify-send")
        .arg("-i")
        .arg("dialog-warning")
        .arg(summary)
        .arg(body)
        .output()?;

    Ok(())
}

fn main() -> Result<()> {
    let mut warned_15 = false;
    let mut warned_10 = false;
    let mut warned_5 = false;

    loop {
        let percentage = get_battery_percentage().unwrap();

        if percentage < 5.0 && !warned_5 {
            send_notification("Battery Warning", "Battery is below 5%!")?;
            warned_5 = true;
        } else if percentage < 10.0 && !warned_10 {
            send_notification("Battery Warning", "Battery is below 10%!")?;
            warned_10 = true;
        } else if percentage < 15.0 && !warned_15 {
            send_notification("Battery Warning", "Battery is below 15%!")?;
            warned_15 = true;
        }

        if percentage > 15.0 {
            warned_15 = false;
        }
        if percentage > 10.0 {
            warned_10 = false;
        }
        if percentage > 5.0 {
            warned_5 = false;
        }

        thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_battery_percentage() {
        let percentage = get_battery_percentage().unwrap();
        println!("Battery percentage: {}", percentage);
        assert!(percentage >= 0.0 && percentage <= 100.0);
    }

}
