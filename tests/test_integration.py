from io import BytesIO
from pathlib import Path
from pprint import pprint

import pytest

BASE_DIR = Path(__file__).resolve().parent


@pytest.fixture()
def drg_test_save_file():
    return open(BASE_DIR / "drg-save-test.sav", "rb")


def test_UESaveError():  # noqa
    from uesave_py import UesaveError  # noqa


def test_load_empty_buffer():
    from io import BytesIO

    fixture_io = BytesIO(b"")
    import uesave_py

    with pytest.raises(uesave_py.UesaveError):
        uesave_py.read_save(fixture_io)


def test_invalid_fd_arg():
    import uesave_py

    with pytest.raises(TypeError):
        uesave_py.read_save(dict())  # noqa


def test_invalid_type_map_arg():
    from io import BytesIO

    import uesave_py

    with pytest.raises(TypeError):
        uesave_py.read_save(BytesIO(b"aaa"), {"some": 0.1})  # noqa

    with pytest.raises(TypeError):
        uesave_py.read_save(BytesIO(b"aaa"), {"some": 1})  # noqa

    with pytest.raises(TypeError):
        uesave_py.read_save(BytesIO(b"aaa"), {"some": None})  # noqa

    with pytest.raises(TypeError):
        uesave_py.read_save(BytesIO(b"aaa"), {1: "some"})  # noqa


def test_load_bytesio(drg_test_save_file):
    import uesave_py

    ret = uesave_py.read_save(BytesIO(drg_test_save_file.read()), {})  # noqa

    assert type(ret) is uesave_py.save.Save
    assert type(ret.header) is uesave_py.save.Header
    assert type(ret.root) is uesave_py.save.Root
    assert type(ret.extra) is bytearray


def test_load_file(drg_test_save_file):
    import uesave_py

    ret = uesave_py.read_save(drg_test_save_file, {})  # noqa

    assert type(ret) is uesave_py.save.Save
    assert type(ret.header) is uesave_py.save.Header
    assert type(ret.root) is uesave_py.save.Root
    assert type(ret.extra) is bytearray


# TODO: Test read/save integrity
