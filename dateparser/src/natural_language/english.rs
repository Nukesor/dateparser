use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./src/natural_language/english.pest"]
struct TimeParser;

pub fn parse<'z, Tz2>(tz: &'z Tz2, input: &str) -> Option<Result<DateTime<Utc>>>
where
    Tz2: TimeZone,
{
    // Check whether we can parse the input. If this doesn't work, the input doesn't match our
    // syntax. Return `None` to signal that the next parser is to be tested.
    let Ok(rule) = TimeParser::parse(Rule::expression, input) else {
        return None;
    };

    println!("{rule:?}");

    Some(Ok(Utc::now()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn english_natural_language() -> Result<()> {
        let tz = &Utc;

        let test_cases = [
            ("3 weeks ago", Duration::days(3 * 7)),
            ("3 weeks and 5 days ago", Duration::days(3 * 7 + 5)),
            (
                "3 weeks and 5 minutes and 12 seconds",
                Duration::days(3) + Duration::minutes(5) + Duration::seconds(12),
            ),
            ("in 5 days", Duration::days(5)),
            (
                "in 5 days and 6 minutes",
                Duration::days(5) + Duration::minutes(6),
            ),
            (
                "in 5 days and 2 hours and 6 minutes",
                Duration::days(5) + Duration::minutes(2) + Duration::minutes(6),
            ),
            //("friday 2pm", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            //("friday 215pm", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            //("friday 2.15pm", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            //("friday 11.15am", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            //("friday 11:15", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            //("friday 0815", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            //("friday 0815", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
        ];

        // Go through all test cases and check whether they're parsed as expected.
        //
        // Since we're working with relative times, we cannot just compare times, as we have to
        // take the current time into account.
        //
        // For this reason, we read the time before and after each test case, which gives us a date
        // range for the current time in which the test ran.
        // We then take the result and check whether it is in that range + the expected relative delta.
        for &(input, delta) in test_cases.iter() {
            let start = Utc::now();
            let result = parse(tz, input).unwrap().context("Failed to parse input")?;
            let end = Utc::now();

            let start = start + delta;
            let end = end + delta;
            assert!(
                start < result && result < end,
                "The parsed result lies outside the expected date range:\n\
                Start: {start:?}, Actual: {result:?}, End: {result:?}"
            )
        }

        Ok(())
    }
}
