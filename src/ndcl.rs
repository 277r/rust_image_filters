use image::*;

fn find_closest(in_vec : & Vec<u8>, val : u8) -> usize {
	for i in 0..in_vec.len(){
		// check if value larger
		if val <= in_vec[i] {
			// check for bounds
			if i == 0 {
				return 0;
			}	

			else {
				// calculate differences, get closest
				let mut d1 = in_vec[i -1] as i32 - val as i32;
				let mut d2 = in_vec[i] as i32 - val as i32;
				d1 = d1.abs();
				d2 = d2.abs();

				// 
				if d1 > d2 {
					return i;
				}
				else {
					return i - 1;
				}

			}
		}
	}

	in_vec.len() - 1
}

pub fn ndcl(img: &mut image::DynamicImage, r : u8, g: u8, b : u8){
	let mut red_colors : Vec<u8> = Vec::new();
	let mut green_colors : Vec<u8> = Vec::new();
	let mut bool_colors : Vec<u8> = Vec::new();

	let r_step : f64 = 255f64 / (r as f64);
	let g_step : f64 = 255f64 / (g as f64);
	let b_step : f64 = 255f64 / (b as f64);
	// calculate colors
	for i in 0..r {
		red_colors.push((r_step * i as f64) as u8);
	}
	
	for i in 0..g {
		green_colors.push((g_step * i as f64) as u8);
	}
	for i in 0..b {
		bool_colors.push((b_step * i as f64) as u8);
	}

	for i in 0..img.width() {
		for ii in 0..img.height() {
			let mut px = img.get_pixel(i,ii);
			px[0] = red_colors[find_closest(&red_colors, px[0])];
			px[1] = green_colors[find_closest(&green_colors, px[1])];
			px[1] = bool_colors[find_closest(&bool_colors, px[1])];	
			
			img.put_pixel(i, ii, px);
			
		}
	}
	
	
	
	

	return;

}