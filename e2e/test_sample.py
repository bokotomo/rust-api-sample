import pytest
import requests
from config import host


def test_health():
    url = host + "health"
    r = requests.get(url)
    assert r.status_code == 200
