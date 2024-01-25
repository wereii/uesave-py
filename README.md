# uesave-py

Python library for editing Unreal Engine save files - wrapper over [uesave-rs](https://github.com/trumank/uesave-rs).

## Notes

### Tests

Pythons `tox` is used to run pytest across multiple python versions.

## Installation

`pip install uesave-py`

Please report any install/build errors.

## Usage

### Reading Save File

## TODO

- [ ] License (probs same as uesave-rs/MIT)
- [ ] Expose uesave ParseError
  - PyO3 currently does not have a facility to create and populate exceptions with instance fields.  
    So, this will require making our own PyErr and properly registering it with the python interface.
- [ ] PyBinaryIO wrapper with Reader interface

--- 

## Disclaimer

- This is a hobby project of mine, my Rust knowledge is still small, 
please make an issue if you see any problem, syntactical error or complete oversight.
- PyO3 does not offer any wrapper over pythons file descriptor/interface, as such, currently,
  this wrapper will **read whole save files into memory** (`.read()` is called on the passed BinaryIO).
