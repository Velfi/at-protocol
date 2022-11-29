# TODO

- I'm tired of converting ABNFs into regex. Can I just create an ABNF rule builder/validator?
  - I wrote a parser for ABNFs with `nom` and the few tests I have are passing. I need to write a
    few more tests and then I need to return structs from the ABNF parsers that allow me to
    construct a parser for that which a given ABNF describes.
  - I still need to update number parsing to handle the "dot" syntax for sequences of numbers.