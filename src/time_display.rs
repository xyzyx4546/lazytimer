use crate::sessions::{Penalty, Solve};
use std::time::Duration;

pub trait TimeDisplay {
    fn format(&self, precision: usize) -> String;
}

impl TimeDisplay for Duration {
    fn format(&self, precision: usize) -> String {
        let secs = self.as_secs();
        let mins = (secs % 3600) / 60;
        let seconds = secs % 60;

        let time_str = if mins > 0 {
            format!("{}:{:02}", mins, seconds)
        } else {
            seconds.to_string()
        };

        if precision > 0 {
            let p = precision.min(3);
            let millis = self.subsec_millis() / 10_u32.pow(3 - p as u32);
            format!("{}.{:0width$}", time_str, millis, width = p)
        } else {
            time_str
        }
    }
}

impl TimeDisplay for Solve {
    fn format(&self, precision: usize) -> String {
        match self.effective_time() {
            Some(duration) => match self.penalty {
                Penalty::None => duration.format(precision),
                Penalty::PlusTwo => duration.format(precision) + "+",
                Penalty::Dnf => unreachable!(),
            },
            None => "DNF".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_duration_formatting() {
        let d1 = Duration::from_millis(12345);
        assert_eq!(d1.format(3), "12.345");
        assert_eq!(d1.format(2), "12.34");

        let d2 = Duration::from_millis(65430);
        assert_eq!(d2.format(2), "1:05.43");
        assert_eq!(d2.format(0), "1:05");

        let d3 = Duration::from_millis(12000);
        assert_eq!(d3.format(3), "12.000");
        assert_eq!(d3.format(0), "12");
    }
}
