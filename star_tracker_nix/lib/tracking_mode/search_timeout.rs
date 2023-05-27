use star_tracker_lib::tracking_mode::AbandonSearch;
use crate::tracking_mode::SearchTimeout;


impl SearchTimeout
{
	/// Constructs and starts the timer.
	pub fn start_timer ( duration: std::time::Duration ) -> SearchTimeout
	{
		return Self { start_time: std::time::Instant::now(), timeout: duration };
	}
}


impl AbandonSearch for SearchTimeout
{
	/// If the timer is exceeded, returns true.
	fn should_abort ( &self ) -> bool
	{
		return self.timeout < self.start_time.elapsed();
	}
	
}