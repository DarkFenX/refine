from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.api import ApiClient
from tests.support.eve import EveDataManager, EveTypeFactory

if TYPE_CHECKING:
    from tests.support.log import LogReader


class TestClient(ApiClient, EveTypeFactory, EveDataManager):

    def __init__(self, *, eve_data_server, api_port: int, log_reader: LogReader):
        super().__init__(data_server=eve_data_server, port=api_port, log_reader=log_reader)
