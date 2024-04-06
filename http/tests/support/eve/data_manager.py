from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import Default, get_stack_key
from .containers import EveObjects

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.util import StackKey

data_id: int = 10000000  # pylint: disable=C0103


class EveDataManager:

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.__datas: dict[str, EveObjects] = {}
        self.__defsrc_stack_alias_map: dict[StackKey, str] = {}

    def mk_eve_data(self) -> EveObjects:
        global data_id  # pylint: disable=C0103,W0603
        alias = str(data_id)
        data = self.__datas[alias] = EveObjects(alias)
        data_id += 1
        return data

    def _get_eve_data(self, data: Union[EveObjects, Type[Default]] = Default) -> EveObjects:
        if data is Default:
            data = self.__default_eve_data
        return data

    @property
    def __default_eve_data(self) -> EveObjects:
        key = get_stack_key()
        if key in self.__defsrc_stack_alias_map:
            alias = self.__defsrc_stack_alias_map[key]
            return self.__datas[alias]
        data = self.mk_eve_data()
        self.__defsrc_stack_alias_map[key] = data.alias
        return data

    @property
    def _eve_datas(self) -> dict[str, EveObjects]:
        return self.__datas
