# -*- coding: utf-8 -*-
import pytest
import requests
from config import host


@pytest.fixture(scope="function", autouse=True)
def setup():
    print("setupDB")


def test_designs():
    url = host + "designs"
    params = {
        "page": 1,
        "page_size": 11,
    }
    r = requests.get(url, params)
    r.encoding = 'utf-8'
    designs = r.json()
    assert r.status_code == 200
    assert designs["designs"][0] == {
        "id": 0,
        'comment': 11,
        'good': 1,
        'title': '\u30bf\u30a4\u30c8\u30eb',
        'thumbnail': 'http://localhost:3000/images/content1.jpg',
        'userId': 0,
        'userImage': 'http://localhost:3000/images/user1.jpg',
        'userName': 'a\u592a\u90ce',
    }