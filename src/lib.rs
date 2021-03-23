use chrono::{prelude::*, Duration};
use std::error;
use std::fmt;
use std::io;

use ureq;

#[derive(Debug)]
pub enum Interval {
	D1,
	D5,
	W1,
	M1,
	M3,
}

impl fmt::Display for Interval {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match self {
			Interval::D1 => "1d",
			Interval::D5 => "5d",
			Interval::W1 => "1w",
			Interval::M1 => "1m",
			Interval::M3 => "3m",
		};

		write!(f, "{}", s)
	}
}

#[derive(Debug)]
pub struct Error {
	source: Box<dyn error::Error>,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.source)
	}
}

impl From<ureq::Error> for Error {
	fn from(source: ureq::Error) -> Self {
		Error {
			source: source.into(),
		}
	}
}

pub fn get(
	ticker: &str,
	from: NaiveDate,
	to: NaiveDate,
	interval: Interval,
) -> Result<Box<dyn io::Read + Send>, Error> {
	let unix_from = from.and_hms(0, 0, 0).timestamp();
	let unix_to = (to + Duration::days(1)).and_hms(0, 0, 0).timestamp();

	let url = format!(
		"https://query1.finance.yahoo.com/v7/finance/download/{}?period1={}&period2={}&interval={}&events=history&includeAdjustedClose=true",
		ticker, unix_from, unix_to, interval
	);

	match ureq::get(&url).call() {
		Ok(resp) => Ok(Box::new(resp.into_reader())),
		Err(err) => Err(err.into()),
	}
}

pub fn days_from_civil(date: NaiveDate) -> i64 {
	let mut y = date.year() as i64;
	let m = date.month() as i64;
	let d = date.day() as i64;

	if m < 3 {
		y -= 1;
	}

	let era = if y >= 0 { y } else { y - 399 } / 400;
	let yoe = y - era * 400;
	let doy = (153 * (m + (if m > 2 { -3 } else { 9 })) + 2) / 5 + d - 1;
	let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;

	era * 146097 + doe - 719468
}
