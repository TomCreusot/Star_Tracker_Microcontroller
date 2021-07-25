/// Implementation for Template
use super::Template;

impl Template
{
	/// Constructs a blank template.
	pub fn new ( ) -> Self
	{
		return Self{keys: Vec::new(), values: Vec::new()};
	}
	
	/// Setter for key and value.
	pub fn add_patten ( &mut self, key: String, value: String )
	{
		self.keys.push(key);
		self.values.push(value);
	}

	/// Replaces the keys in a line with the values.
	pub fn replace_line ( &self, template: &mut String )
	{
		for i in 0..self.keys.len()
		{
			let key = format!("$({})", self.keys[i]);
			let replaced = template.replace(&key, &self.values[i]);
			*template = replaced;
		}
	}
	
	/// Replaces the keys in a vector of strings with the values.
	pub fn replace_lines ( &self, template: &mut Vec<String>) 
	{
		for e in template
		{
			self.replace_line(e);
		}
	}
	
	
	/// Replaces the line containing a key with a vector which may be large.
	pub fn replace_line_with_vec ( &self, template: &mut Vec<String>, 
									key: &String, value: &Vec<String> )
	{
		for ii in 0..template.len()
		{
			if template[ii].contains(&format!("$({})", key).to_string())
			{
				template.remove(ii);
				for jj in 0..value.len()
				{
					template.insert(ii + jj, value[jj].clone());
				}
				break;
			}
		}
	}
	
}