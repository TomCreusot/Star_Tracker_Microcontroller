extern crate star_tracker;

use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;

use star_tracker::util::aliases::Decimal;
// use star_tracker::config::TrackingModeConstructConstsStruct;
// use star_tracker::config::TrackingModeConstructConsts;
// use star_tracker::config::TrackingModeConstsStruct;
// use star_tracker::config::TrackingModeConsts;

use star_tracker::nix::Io;
use star_tracker::nix::Star;


fn main (  )
{
	println!("\n\n\nRUNNING");
	
	let mut rdr = Io::get_csv ( NixConstsStruct::HYG_DATABASE_PATH, NixConstsStruct::HYG_DATABASE_URL );
	let iter = rdr.deserialize();
		
	const NUM : usize = 11;
	const MAX : Decimal = 10.0;
	const MIN : Decimal = -1.0; // Sun is the only thing brighter
	let mut thresholds : [Decimal; NUM] = [0.0; NUM];

	for i in 0..NUM
	{
		thresholds[i] = MIN + i as Decimal * (MAX - MIN) / NUM as Decimal;
	}
	
	let mut vals = [SpectStruct::new(); NUM];
	for record in iter
	{
		let record : Star = record.expect("Could not decode.");
		let spec = Spect::new(&record.spec);
		if spec != Spect::NONE
		{
			let mut i = 0;
			while i + 1 < thresholds.len() && thresholds[i] < record.mag
			{
				i += 1;
			}
			vals[i].inc(spec);
		}
	}
	
	println!("Magnitude, o, b, a, f, g, k, m, mean, num");
	let mut num = 0;
	for i in 0..vals.len()
	{
		println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", 
			thresholds[i], 
			vals[i].o,
			vals[i].b,
			vals[i].a,
			vals[i].f,
			vals[i].g,
			vals[i].k,
			vals[i].m,
			vals[i].mean(),
			vals[i].num(),
		);
		num += vals[i].num();
	}
	println!("TOTAL NUM: {}", num);
}


#[derive(Debug, Eq, PartialEq)]
pub enum Spect
{
	O, B, A, F, G, K, M, NONE
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct SpectStruct
{
	pub o : u32,
	pub b : u32,
	pub a : u32,
	pub f : u32,
	pub g : u32,
	pub k : u32,
	pub m : u32,
}

impl Spect
{
	fn new ( str: &String ) -> Spect
	{
		if str.contains("O")		{
			return Spect::O;
		}
		else if str.contains("B")	{
			return Spect::B;
		}
		else if str.contains("A")	{
			return Spect::A;
		}
		else if str.contains("F")	{
			return Spect::F;
		}
		else if str.contains("G")	{
			return Spect::G;
		}
		else if str.contains("K")	{
			return Spect::K;
		}
		else if str.contains("M")	{
			return Spect::M;
		}
		return Spect::NONE;
	}
	
	/// A number frin 0 being blue star and m bein red star.
/*	fn as_num ( &self ) -> u32
	{
		match self
		{
			Spect::O => return 0,
			Spect::B => return 1,
			Spect::A => return 2,
			Spect::F => return 3,
			Spect::G => return 4,
			Spect::K => return 5,
			Spect::M => return 6,
			Spect::NONE => return 1000,
		}
	}*/
	
	/// The main wavelength of the spectral class.
	fn wavelength ( &self ) -> Decimal
	{
		match self
		{
			Spect::O => return 0.0,
			Spect::B => return 1.0,
			Spect::A => return 2.0,
			Spect::F => return 3.0,
			Spect::G => return 4.0,
			Spect::K => return 5.0,
			Spect::M => return 6.0,
			Spect::NONE => return 1000.0,
		}		
	}
}


impl SpectStruct
{
	fn new ( ) -> Self
	{
		return SpectStruct{o: 0, b: 0, a: 0, f: 0, g: 0, k: 0, m: 0};
	}
	
	/// Gets the number of stars.
	fn num ( &self ) -> u32
	{
		return self.o + self.b + self.a + self.f + self.g + self.k + self.m
	}
	
	/// If each star is given a number, the weighted mean.
	fn mean ( &self ) -> Decimal
	{
		return 
		( 
			self.o as Decimal * Spect::O.wavelength() + 
			self.b as Decimal * Spect::B.wavelength() + 
			self.a as Decimal * Spect::A.wavelength() + 
			self.f as Decimal * Spect::F.wavelength() + 
			self.g as Decimal * Spect::G.wavelength() +
			self.k as Decimal * Spect::K.wavelength() +
			self.m as Decimal * Spect::M.wavelength() 
		) / self.num() as Decimal;
	}
	
	/// Increment the spectral class.
	fn inc ( &mut self, spec: Spect )
	{
		match spec
		{
			Spect::O => self.o += 1,
			Spect::B => self.b += 1,
			Spect::A => self.a += 1,
			Spect::F => self.f += 1,
			Spect::G => self.g += 1,
			Spect::K => self.k += 1,
			Spect::M => self.m += 1,
			Spect::NONE => println!("Cannot inc"),
		}
	}
}




