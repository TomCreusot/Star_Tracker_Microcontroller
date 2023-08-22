//! implementation of [The threshold trait](crate::image_processing::Threshold).

//###############################################################################################//
//										--- Percent ---
//###############################################################################################//

impl ThresholdPercent
{
	/// Generates a threshold for the given image.
	fn new ( img: &Image, percent: Decimal ) -> Self
	{
		let mut histogram : [Byte; 255] = [0; 255];
		img.histogram(&mut histogram);
		Self{threshold: img.percent_threshold(percent, &histogram)};
	}
}


pub impl Threshold for ThresholdPercent
{
	/// Returns true if the pixel is in the foreground (stars).
	fn foreground ( &self, point: Pixel, value: Byte ) -> bool
	{
		return self.threshold <= value;
	}
}



//###############################################################################################//
//										--- Nodal ---
//###############################################################################################//

impl ThresholdNodal<const NUM_H: usize, const NUM_V: usize>
{
	/// Generates a threshold for the given image.
	/// Returns the threshold.
	fn new ( img: &Image ) -> Self
	{
		let nodes = BasicImage::new();
	
		let node_reach = Pixel {
			x: img.width()  / (NODES.x - 1),
			y: img.height() / (NODES.y - 1),
		};

		// |1      |...|2     2|...|3  3  3|...|4 4 4 4|
		let node_spacing = Pixel {
			x: img.width()  / (NODES.x - 1),
			y: img.height() / (NODES.y - 1),
		};

		for n_x in 0..NODES.x
		{
			for n_y in 0..NODES.y
			{
				let node = Pixel{x: n_x, y: n_y};
				let min = Pixel{
					x: node.x.saturating_sub(node_reach.x), 
					y: node.y.saturating_sub(node_reach.y)};
					
				let max = Pixel{
					x: std::cmp::min(node.x + node_reach.x, img.width()), 
					y: std::cmp::min(node.y + node_reach.y, img.height())};
				
				let mut avg = 0.0;
				for p_x in min.x .. max.x
				{
					for p_y in min.y .. max.y
					{
						let node = Pixel{x: p_x, y: p_y};			
						avg += img.get(pos) as Decimal;
					}
				}
				avg /= node_reach.x * node_reach.y;
				nodes.get(node) = avg as Byte;
			}
		}
		
		return Ok(Self{nodes: nodes});
	}
}



pub impl <const NUM_H: usize, const NUM_V: usize> Threshold for ThresholdNodal<NUM_H, NUM_V>
{
	/// Returns true if the pixel is in the foreground (stars).
	fn foreground ( &self, point: Pixel, value: Byte ) -> bool
	{
		
	}
}


