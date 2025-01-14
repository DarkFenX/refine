from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

from tests.support import eve
from tests.support.log import LogEntryNotFound
from tests.support.request import Request
from tests.support.util import Default
from .base import ApiClientBase

if TYPE_CHECKING:
    from typing import Union


class ApiClientSrc(ApiClientBase, eve.EveDataManager, eve.EveDataServer):

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.__fast_src_log_check: bool = True
        self.__created_data_aliases: set[str] = set()

    def create_source_request(
            self, *,
            data: Union[eve.EveObjects, type[Default]],
    ) -> Request:
        if data is Default:
            data = self._get_default_eve_data()
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/src/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'{self._eve_data_server_base_url}/{data.alias}/'})

    def create_source(
            self, *,
            data: Union[eve.EveObjects, type[Default]] = Default,
            log_check: bool = True,
    ) -> None:

        def process() -> None:
            resp = self.create_source_request(data=data).send()
            assert resp.status_code == 201
            self.__created_data_aliases.add(data.alias)

        if data is Default:
            data = self._get_default_eve_data()
        self._setup_eve_data_server(data=data)
        if log_check:
            with self._log_reader.get_collector() as log_collector:
                process()
                if self.__fast_src_log_check:
                    # Check if there are any "cleaned" entries in log upon completion w/o any
                    # waiting for a fast way
                    with pytest.raises(LogEntryNotFound):
                        log_collector.wait_log_entry(msg='re:cleaned .+', level='INFO', span='src-new:adg', timeout=0)
                else:
                    # Wait for negative report to appear for regular check
                    log_collector.wait_log_entry(
                        msg='no unused data found during cleanup',
                        level='INFO',
                        span='src-new:adg',
                        timeout=3)
        else:
            process()

    def remove_source_request(self, *, src_alias: str) -> Request:
        return Request(
            client=self,
            method='DELETE',
            url=f'{self._base_url}/src/{src_alias}')

    def remove_source(self, *, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def create_sources(self, *, log_check: bool = True) -> None:

        def process(*, src_log_check: bool) -> None:
            for data in self._eve_datas.values():
                self.create_source(data=data, log_check=src_log_check)

        # If no data was created, create default one
        if not self._eve_datas:
            self._get_default_eve_data()
        # Fast log check is done when we create multiple sources if possible, it becomes more
        # reliable this way; we check if there are any "cleaned" entries in log upon completion w/o
        # any waiting
        if log_check and self.__fast_src_log_check:
            with self._log_reader.get_collector() as log_collector:
                process(src_log_check=False)
                with pytest.raises(LogEntryNotFound):
                    log_collector.wait_log_entry(msg='re:cleaned .+', level='INFO', span='src-new:adg', timeout=0)
        else:
            process(src_log_check=log_check)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)
