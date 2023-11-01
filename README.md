
# rot13

A command-line tool to encode and decode ROT13 text

## Usage

Run the tool from the command line.
It takes any number of command-line arguments, all of which are treated as text to be encoded or decoded.

The tool only encodes and decodes ASCII alphabetical characters. All other characters in the input are left untouched.

It maintains casing – uppercase letters stay uppercase, and lowercase letters stay lowercase. 

### Examples

`rot13 "gur obhear vqragvgl"` -> `the bourne identity`

`rot13 "the bourne identity"` -> `gur obhear vqragvgl`

You can also use it without quotes:

`rot13 gur obhear vqragvgl` -> `the bourne identity`

`rot13 the bourne identity` -> `gur obhear vqragvgl`

