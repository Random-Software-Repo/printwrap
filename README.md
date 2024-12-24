# printwrap
Rust lib to print wrapped text to stdout with line breaks on spaces.

Currently this is used in the adjacent repos znappr, piper and 
seasonalemoji for printing the help/usage output of `the-command -h` in 
a somewhat nice manner.

# Usage

To use printwrap, add the following line to the '[dependencies]' section of your `Cargo.toml`:
```
printwrap = { path = "../printwrap" }
```
where `"../printwrap"`` is the actual path to where you've cloned the printwrap repo.

Then, include 'extern crate printwrap;' among the use clauses in your rust source file(s).

Actual calls to printwrap would then look something like:
```
printwrap::print_wrap(5,10,"Some text here...");
```
The parameters are: 
 - margins 
     + Minimum margin on both left and right sides of the terminal output.
 - subsequent_tab 
     + When a line wraps, the second (and all subsequent) line will have a left margin of this value + the base margin (margins).
 - line 
     + The text, already formatted, to print.