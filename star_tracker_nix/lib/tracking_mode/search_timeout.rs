use star_tracker_lib::tracking_mode::AbandonSearch;
use crate::tracking_mode::AbandonSearchTimeout;
use crate::tracking_mode::AbandonSearchTimeoutFailure;

use star_tracker_lib::tracking_mode::AbandonSearchFailures;


impl AbandonSearchTimeout
{
	/// Constructs and starts the timer.
	pub fn new ( duration: std::time::Duration ) -> Self
	{
		return Self { start_time: std::time::Instant::now(), timeout: duration };
	}
}


impl AbandonSearch for AbandonSearchTimeout
{
	/// If the timer is exceeded, returns true.
	fn should_abort ( &mut self ) -> bool
	{
		return self.timeout < self.start_time.elapsed();
	}
	
}


impl AbandonSearchTimeoutFailure
{
	/// Constructs and starts the timer counter.
	/// Max failures is the number of times the algorithm could not find a match.
	pub fn new ( duration: std::time::Duration, max_failures: usize ) -> Self
	{
		return Self { 
			timeout: AbandonSearchTimeout::new(duration), 
			failure: AbandonSearchFailures::new(max_failures) };
	}
}


impl AbandonSearch for AbandonSearchTimeoutFailure
{
	/// If the timer is exceeded or count has reached max, returns true.
	fn should_abort ( &mut self ) -> bool
	{
		return self.timeout.should_abort() || self.failure.should_abort();
	}
	
}