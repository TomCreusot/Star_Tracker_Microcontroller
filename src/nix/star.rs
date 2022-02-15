use std::fmt;
use std::cmp::Ordering;
use serde::de::{self, SeqAccess, MapAccess, Visitor, Deserializer};
use serde::Deserialize;

use crate::config::NixConstsStruct;
use crate::config::NixConsts;

use crate::nix::Star;
// use crate::nix::SerialStar;
use crate::util::test::TestEqual;
use crate::util::aliases::Decimal;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::units::Hours;

use crate::util::list::List;

impl Star
{
	// Default Constructor
//	pub fn new ( )
//	{
//		return Self{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0), spec: ""}};
//	}


	pub fn to_equatorial ( list: &dyn List<Star> ) -> Vec<Equatorial>
	{
		let mut val : Vec<Equatorial> = Vec::new();
		for i in 0..list.size()
		{
			val.push_back(list.get(i).pos).expect("Could not push back");
		}
		return val;
	}

}




impl Ord for Star
{
	/// Allows ordering with magnitude.
	fn cmp(&self, other: &Self) -> Ordering
	{
		if self.mag > other.mag + 0.01
		{
			return Ordering::Greater;
		}
		else if other.mag > other.mag + 0.01
		{
			return Ordering::Less;
		}
		else
		{
			return Ordering::Less;
		}
	}
}

impl Eq for Star {}

impl PartialEq for Star
{
	fn eq ( &self, other: &Self ) -> bool
	{
		return 
		self.mag.test_equal(&other.mag) &&
		self.pos.ra.test_equal(&other.pos.ra) &&
		self.pos.dec.test_equal(&other.pos.dec) &&
		self.spec == other.spec;
	}
}

impl PartialOrd for Star
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.mag > other.mag + 0.01
		{
			return Some(Ordering::Greater);
		}
		else if other.mag > other.mag + 0.01
		{
			return Some(Ordering::Less);
		}
		else
		{
			return Some(Ordering::Less);
		}
    }
}





impl<'de> Deserialize<'de> for Star
{

	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(field_identifier, rename_all = "lowercase")]
		#[derive(Debug)]
        enum Field { Mag, Ra, Dec, Spect }
/*
        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field 
		{
            
			fn deserialize<D>(deserializer: D) -> Result<Field, D::Error> where D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;
					
					//
					// QUERY
					// Sends the fields that are expected in the object.
					// In Order?
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        // formatter.write_str("`secs` or `nanos`")
						println!("EXPECTING FIELD");
						let s = format!("`{}` or `{}` or `{}` or `{}`", FIELDS[0], FIELDS[1], FIELDS[2], FIELDS[3]);
						// let s = format!("`{}`", FIELDS[0]);
						return formatter.write_str(&s);
                    }

					//
					// Returns the enumeration of the field requested.
                    fn visit_str<E>(self, value: &str) -> Result<Field, E> where E: de::Error
                    {
						println!("Visit str");
						
						// Rust doesnt like match with arrays?
						if FIELDS[0].eq(value) 		{ return Ok(Field::Mag); }
						else if FIELDS[1].eq(value) { return Ok(Field::Ra); }
						else if FIELDS[2].eq(value) { return Ok(Field::Dec); }
						else if FIELDS[3].eq(value) { return Ok(Field::Spec); }
						else { return Err(de::Error::unknown_field(value, FIELDS)); }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
		
		
		*/
		
		
		struct DurationVisitor;

		impl<'de> Visitor<'de> for DurationVisitor 
		{
			type Value = Star;
			
			//
			// QUERY
			// The name of the struct.
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result 
			{
				println!("EXPECTING STAR");
				return formatter.write_str("struct Star");
			}

			//
			// Creates object.
			// This uses the order of expecting in FieldVisitor.expecting ?
			fn visit_seq<V>(self, mut seq: V) -> Result<Star, V::Error> where V: SeqAccess<'de>,
			{
				let mag  = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
				let ra   = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
				let dec  = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
				let spec = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
				let pos  = Equatorial{ra: Radians(ra), dec: Radians(dec)};
				return Ok(Star{mag: mag, pos: pos, spec: spec});
			}

			//
			// Creates object with map.
			fn visit_map<V>(self, mut map: V) -> Result<Star, V::Error> where V: MapAccess<'de>,
			{				
				// MapAccess is an iterator of keys and values.
				let mut mag = None;
				let mut ra = None;
				let mut dec = None;
				let mut spec = None;
				let mut continue_loop = true;
				// Assigns secs and nanos if there is the correct entries of that variable.
				while continue_loop
				{
					let key_wrapped = map.next_key::<Field>();
					let next = map.next_value::<String>();
					
					// The values are from the enum
					if key_wrapped.is_ok()
					{
						// The end of the list has not yet been reached.
						let key_unwrapped = key_wrapped.unwrap();
	
						if key_unwrapped.is_some()
						{
							match key_unwrapped.unwrap()
							{
								Field::Ra =>
								{
									ra = Some(next.expect("RA NOT FILLED").parse::<Decimal>().expect("RA"));
								}
								Field::Dec =>
								{
									dec = Some(next.expect("DEC NOT FILLED").parse::<Decimal>().expect("DEC"));
								}
								Field::Mag =>
								{
									mag = Some(next.expect("MAG NOT FILLED").parse::<Decimal>().expect("MAG"));
								}
								Field::Spect =>
								{
									spec = Some(next);
								}
							}
						}
						// End of List
						else
						{
							continue_loop = false;
						}
					}
				}
				// If variables correct, returns object.
				let mag  = mag.ok_or_else(|| de::Error::missing_field(FIELDS[0]))?;
				let mut ra   = ra.ok_or_else(|| de::Error::missing_field(FIELDS[1]))?;
				let mut dec  = dec.ok_or_else(|| de::Error::missing_field(FIELDS[2]))?;
				let spec = spec.ok_or_else(|| de::Error::missing_field(FIELDS[3]))?;
				
				if NixConstsStruct::HYG_DATABASE_DEC_DEGREES
				{
					dec = Degrees(dec).to_radians().0;
				} 
				if NixConstsStruct::HYG_DATABASE_RA_HOURS
				{
					ra = Hours(ra).to_radians().0;
				}
				
				let pos  = Equatorial{ra: Radians(ra), dec: Radians(dec)};
				return Ok(Star{mag: mag, pos: pos, spec: spec.expect("")});
			}
		}

		const FIELDS: &'static [&'static str] = 
			&[
				NixConstsStruct::HYG_DATABASE_HEADER_MAGNITUDE, 
				NixConstsStruct::HYG_DATABASE_HEADER_RIGHT_ASCENTION, 
				NixConstsStruct::HYG_DATABASE_HEADER_DECLINATION, 
				NixConstsStruct::HYG_DATABASE_HEADER_SPECULARITY,
			];
		deserializer.deserialize_struct("Star", FIELDS, DurationVisitor)
	}
}