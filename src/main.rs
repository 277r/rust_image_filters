mod ndcl;
use std::env;
use std::path::Path;
//use crate::ndcl;



fn main() {

	
	let args: Vec<String> = env::args().collect();
	if args.len() < 6{
		print!("run ndcl input output levelR levelG, levelB\nwhere the R G B levels can be a number from 0 to 255");
		return;
	}

	// retrieve arguments
	let in_path = Path::new(&args[1]); 
	let out_path = Path::new(&args[2]); 

	let level_r = args[3].parse::<u8>().unwrap();
	let level_g = args[4].parse::<u8>().unwrap();
	let level_b = args[5].parse::<u8>().unwrap();


	// check to open image
	let input_image_res = image::open(in_path);
	if !input_image_res.is_ok() {	
		// error
		return;
	}

	// check if rgb range is correct
	if  level_r < 1 || level_g < 1 || level_b < 1 {
		// error
		return;

	}


	// run filter function
	let mut input_image = input_image_res.unwrap();
	ndcl::ndcl(&mut input_image, level_r, level_g, level_b);
	

	// save image
	let save_res = input_image.save(out_path);
	if save_res.is_ok() {
		// well done it worked
	}
	else {
		// error
		return;
	}


}
