use terminal_size::{Width, Height, terminal_size};

/*
	print_wrap
		margins          minimum left and right margins
		subsequent_tab   minimum additional left margin on lines after the first
		line             the line to format

	Prints <line> to standard output with minimum padding on 
	both left and right sides. If the line is longer than the
	terminal window (adding the margins), the line will be 
	split on spaces and overflow will be printed on additional
	lines as needed. The subsequent lines will have additional
	padding on the left if subsequent_tab is greater than zero.
*/
pub fn print_wrap(margins:usize, subsequent_tab:usize, line:&str)
{
	let x:usize;

	if let Some((Width(w), Height(_h))) = terminal_size()
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
		let mut index= width - current_tab - margins - 1;
		let mut start= 0 as usize;
		while (index > start) && (index < bytes.len())
		{
			if bytes[index] == b' '
			{
				let slice = &line[start..index];
				println!("{:current_tab$}{}", "",slice);
				current_tab = subsequent_tab + margins;
				start = index + 1; // skip the space we're currently on
				index = start + width - current_tab - margins;

				if index >= bytes.len()
				{
					index = bytes.len() -1;
				}
				//println!("\tindex {}, start {}, len {}, (index-start) {}, (width-current_tab - margins -1) {}",
				//	index,start,bytes.len(),(index-start),(width-current_tab - margins -1));
				
				if (index-start) < (width - current_tab - margins -1)
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


/*
	split
		width          maximum line width
		line           the line to split

	Splits <line> on spaces into one or more Strings where the
	maximum length of each string is no more than <width>
	characters wide. Weird things happen if <line> or part of
	<line> has no spaces and is longer than width.
*/
pub fn split(width:usize, line:&str) -> Vec<String>
{
	let mut strings:Vec<String> = Vec::new();

	if line.len() <= width
	{
		strings.push(String::from(line));
	}
	else
	{
		let bytes = line.as_bytes();
		let mut index= width - 1;
		let mut start= 0 as usize;
		while (index > start) && (index < bytes.len())
		{
			if bytes[index] == b' '
			{
				let slice = &line[start..index];
				strings.push(String::from(slice));
				start = index + 1; // skip the space we're currently on
				index = start + width ;

				if index > bytes.len()
				{
					index = bytes.len() -1;
				}
				if (index-start) < (width -1)
				{
					// end of the line. not enough left to split, so just print it and quit
					let slice = &line[start..bytes.len()];
					strings.push(String::from(slice));
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
			strings.push(String::from(slice));
		}
	}
	return strings;
}
