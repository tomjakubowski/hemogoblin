# hemogoblin

Rust bindings for [libPlasma](https://github.com/plasma-hamper/plasma).

What works so far:

- Construction and emission of slaw:
    - Nil
    - Strings
    - Booleans
- Proteins:
    - Read from file (YAML or binary)
    - Access to descrips/ingests
- An incomplete attempt to port ObRetort to Rust error handling idioms

Incomplete list of planned features to support in hemogoblin:
- Slaw:
    - Conses
    - Lists and maps, including builders
    - The full numeric ilk rumpus (including vector + multivector types, and arrays
      thereof)
- Poolhosen:
    - mmap and tcp
    - Full complement of verbs - rewind, nth, await, seek, etc.
- Protein:
    - Construct in code
    - Write to file (YAML or binary)
- Macro sugar: `slaw!()`, `slaw_list!()`, `slaw_map!()`o

