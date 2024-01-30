# uesave-py

Python library for editing Unreal Engine save files - PyO3 wrapper around [uesave-rs](https://github.com/trumank/uesave-rs).

> [!WARNING]  
> This is still work in progress and there is a lot optimisiation missing, mainly it will use lot of memory, use with care.

## Usage

Minimal compatible python version is `python3.8`.

`pip install uesave-py`

**Reading Save Files**

```py
from ueasve_py import read_save, UesaveError

try:
    save = read_save(open(file, 'rb'))
except UesaveError as exc:
    print("oops")

# the returned Save instance should be typed, otherwise see uesave_py.save

# This prints a lot of stuff!
print(save.header, save.root, save.extra)
```

**Writing Save Files**

Not Implemented Yet ☹️

## Development

In a way this is a simple PyO3 wrapper around uesave-rs library.

### Tests

Pythons `tox` is used to run pytest across multiple python versions.

## TODO

- CI: build the package first then use it in tests (so that we are not building twice)
- License (probs same as uesave-rs/MIT)
- Expose uesave ParseError
  - PyO3 currently does not have a facility to create and populate exceptions with instance fields.  
    So, this will require making our own PyErr and properly registering it with the python interface.
- Improve transfering (Save) data from/to Python.
  Currently the whole uesave::save struct is immediatelly transferred into python, by itself it's OK.
  Except when/if implementing `write_save` it might get painful really quick because of all the type coercion/conversion back to rust (and it must be correct).

  - Idea 1: Keep the rust-side Save in some "getattr proxy class" (rust-side pyclass struct).
  This would mean not transferring anything immediatelly but writing changes could be painful again (capture setattr, properly coerce types, mutable fields are problem).

## Disclaimer

- This is a hobby project of mine, my Rust knowledge is still rather insufficient, 
  please make an issue if you see any problem, be it syntactical error or complete oversight.
