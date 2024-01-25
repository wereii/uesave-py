from typing import BinaryIO, Literal

from uesave_py.save import Save

TypeEnum = (
    Literal[
        "Guid",
        "DateTime",
        "Timespan",
        "Vector2D",
        "Vector",
        "Box",
        "IntPoint",
        "Quat",
        "Rotator",
        "LinearColor",
        "Color",
        "SoftObjectPath",
        "Struct",
    ]
    | str
)
TypeMap = dict[str, TypeEnum]

class UesaveError(Exception): ...

def load(fd: BinaryIO, types: TypeMap | None = None) -> Save: ...
def dump(fd: BinaryIO, data: Save): ...
