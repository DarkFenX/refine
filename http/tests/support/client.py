from __future__ import annotations

from tests.support.api import ApiClient
from tests.support.eve import EveDataClient


class TestClient(EveDataClient, ApiClient):

    def __init__(self, eve_data_server, api_port: int):
        EveDataClient.__init__(self, data_server=eve_data_server)
        ApiClient.__init__(self, port=api_port)
