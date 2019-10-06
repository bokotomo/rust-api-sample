import pytest
from helper.query import desiger_add

@pytest.fixture(scope='function', autouse=True)
def setup():
    print("11")


def test_desiger_index():
    desiger_add(id=1, name="Ok")
    assert 1 == 2
