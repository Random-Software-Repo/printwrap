# printwrap
Rust lib to print wrapped text to stdout with line breaks on spaces.

Currently this is used in the adjacent repos znappr, piper and 
seasonalemoji for printing the help/usahe output of `the-command -h` in 
a somewhat nice manner.

# Usage

To use printwrap, add the following line to the '[dependencies]' section of your `Cargo.toml`:
```
printwrap = { path = "../printwrap" }
```
where `"../printwrap"`` is the actual path to where you've cloned the printwrap repo.

Then, include 'extern crate printwrap;' among the use clauses in your rust source file(s).

Actual calls to printwrap would then look like:
```
printwrap::print_wrap(5,10,"Some text here...");
```