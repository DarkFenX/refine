from __future__ import annotations

from tests.support.api import ApiClient
from tests.support.eve import EveDataClient


class TestClient(ApiClient, EveDataClient):

    def __init__(self, eve_data_server, api_port: int):
        super().__init__(data_server=eve_data_server, port=api_port)
