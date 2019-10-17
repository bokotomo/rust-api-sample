import pytest
import requests
from e2e.config import host
from e2e.driver.mysql import db


class TestDesigner():
    @pytest.fixture(scope='function', autouse=True)
    def setup(self):
        print("trancate db")
        yield
        print("tear down")

    def test_index(self):
        db.table('test').insert(id=4, name='212112')

        url = host + "designer"
        params = {
            "page": 1,
            "page_size": 11,
        }
        r = requests.get(url, params)
        r.encoding = 'utf-8'
        designer = r.json()

        assert r.status_code == 200
        assert designer["designers"][0]["id"] == 0
