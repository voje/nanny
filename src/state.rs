use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use chrono::{DateTime, Duration, Local};
use log::info;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    limit: Duration,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    limit_left: Duration,
    last_sync: DateTime<Local>,
    start: NaiveTime,
    end: NaiveTime,
}

impl State {
	pub fn new(limit: u32, start: &str, end: &str) -> Self {
		let l = Duration::minutes(limit.into());
		State {
			limit: l,
			limit_left: l,
			last_sync: Local::now(),
			start: NaiveTime::parse_from_str(start, "%H:%M")
				.expect("Failed to parse date"),
			end: NaiveTime::parse_from_str(end, "%H:%M")
				.expect("Failed to parse date"),
		}
	}

	fn _ymd(d: &DateTime<Local>) -> String {
		d.format("%Y-%m-%d").to_string()
	}

	fn _is_inside_timerange(&self, time_now: DateTime<Local>) -> bool {
		let nt = time_now.time();	
		(self.start < nt) && (self.end > nt)	
	}

	fn _last_synced_today(&self, time_now: DateTime<Local>) -> bool {
		time_now.format("%Y-%m-%d").to_string() == 
		self.last_sync.format("%Y-%m-%d").to_string()
	}

	// Tick checks conditions and updates state.   
	// When tick returns 'false', it's time to shutdown.   
	pub fn tick(&mut self, freq: Duration, time_now: DateTime<Local>) -> bool {
		if ! self._last_synced_today(time_now) {
			info!("Resetting daily limit");
			self.limit_left = self.limit;
		}
		self.last_sync = time_now;

		// Countdown limit
		self.limit_left = self.limit_left - freq;

		if ! self._is_inside_timerange(time_now) {
			info!("Outside timerange");
			return false
		}

		if self.limit_left <= Duration::minutes(0) {
			info!("Out of time");
			return false
		}

		return true
	}
}

#[test]
fn test_daily_reset() {
	let tn = Local.datetime_from_str("2014-11-28 12:00", "%Y-%m-%d %H:%M").unwrap();
	let mut s = State::new (
		30,
		"07:00",
		"22:00",	
	);
	let freq = Duration::minutes(1);
	let res = s.tick(freq, tn);
	assert_eq!(res, true);
	assert_eq!(Duration::minutes(29), s.limit_left);

	let freq = Duration::minutes(9);
	let res = s.tick(freq, tn);
	assert_eq!(res, true);
	assert_eq!(Duration::minutes(20), s.limit_left);

	// Over the time limit
	let freq = Duration::minutes(100);
	let res = s.tick(freq, tn);
	assert_eq!(res, false);
}

#[test]
fn test_outside_allowed_hour() {
	let mut s = State::new (
		30,
		"07:00",
		"22:00",	
	);
	let freq = Duration::minutes(1);
	let t1 = Local.datetime_from_str("2014-11-28 12:00", "%Y-%m-%d %H:%M").unwrap();
	assert_eq!(s.tick(freq, t1), true);

	let t2 = Local.datetime_from_str("2014-11-28 06:00", "%Y-%m-%d %H:%M").unwrap();
	assert_eq!(s.tick(freq, t2), false);

	let t3 = Local.datetime_from_str("2014-11-28 23:00", "%Y-%m-%d %H:%M").unwrap();
	assert_eq!(s.tick(freq, t3), false);
}

