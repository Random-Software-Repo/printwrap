use terminal_size::{Width, Height, terminal_size};

pub fn print_wrap(margins:usize, subsequent_tab:usize, line:&str)
{
	let x;

	let size = terminal_size();
	if let Some((Width(w), Height(_h))) = size
	{
		x = w as usize;
	}
	else
	{
		x = 100 as usize;
	}

	let width = x as usize;
	let mut current_tab = margins;
	if (line.len() + (margins+margins)) <= width
	{
		println!("{:current_tab$}{}", "",line);
	}
	else
	{
		let bytes = line.as_bytes();
		let mut index= width - current_tab - margins - margins - 1;
		let mut start= 0 as usize;
		while (index > start) && (index < bytes.len())
		{
			if bytes[index] == b' '
			{
				let slice = &line[start..index];
				println!("{:current_tab$}{}", "",slice);
				current_tab = subsequent_tab + margins;
				start = index + 1; // skip the space we're currently on
				let pre_index = start + (width - current_tab - margins) -1 ;
				index = pre_index;
				if index > bytes.len()
				{
					index = bytes.len() -1;
				}
				if (index-start) < (width - current_tab -margins -1)
				{
					// end of the line. not enough left to split, so just print it and quit
					let slice = &line[start..bytes.len()];
					println!("{:current_tab$}{}", "",slice);
					break;
				}
			}
			else
			{
				index = index - 1;
			}
		}
		if index == start
		{
			// no spaces before width, so just print the first width characters
			let slice = &line[start..(bytes.len())];
			println!("{:current_tab$}{}", "",slice);
		}

	}
}
