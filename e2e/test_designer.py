import pytest
from helper.query import desiger_add
import requests
from config import host


@pytest.fixture(scope='function', autouse=True)
def setup():
    print("setupDB")


def test_desiger_index():
    url = host + "designer"
    params = {
        "page": 1,
        "page_size": 11,
    }
    r = requests.get(url, params)
    r.encoding = 'utf-8'
    designer = r.json()

    desiger_add(id=1, name="Ok")

    assert r.status_code == 200
    assert False
