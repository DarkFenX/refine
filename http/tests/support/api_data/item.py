from __future__ import annotations

from collections import namedtuple
from typing import TYPE_CHECKING

from tests.support.util import AttrDict, Absent
from .mod_info import AttrModInfoMap

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.client import TestClient
    from tests.support.consts import ApiEffMode, ApiState
    from tests.support.eve_data import TestObjects
    from tests.support.request import Request

AttrVals = namedtuple('AttrVals', ('base', 'dogma', 'extra'))
EffectInfo = namedtuple('EffectInfo', ('running', 'mode'))


class Item(AttrDict):

    def __init__(self, client: TestClient, data: TestObjects, ss_id: str):
        super().__init__(
            data=data,
            hooks={
                'charge': lambda charge: Item(client=client, data=charge, ss_id=ss_id),
                'attrs': lambda attrs: {int(k): AttrVals(*v) for k, v in attrs.items()},
                'effects': lambda effects: {int(k): EffectInfo(*v) for k, v in effects.items()},
                'mods': AttrModInfoMap})
        self._client = client
        self._ss_id = ss_id

    def update_request(self) -> Request:
        return self._client.get_item_request(ss_id=self._ss_id, item_id=self.id)

    def update(self) -> Item:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def change_skill_request(
            self,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_skill_request(
            ss_id=self._ss_id, item_id=self.id, level=level, state=state, effect_modes=effect_modes)

    def change_skill(
            self,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_skill_request(level=level, state=state, effect_modes=effect_modes).send()
        assert resp.status_code == 200

    def change_mod_request(
            self,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_mod_request(
            ss_id=self._ss_id,
            item_id=self.id,
            state=state,
            charge=charge,
            effect_modes=effect_modes)

    def change_mod(
            self,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_mod_request(state=state, charge=charge, effect_modes=effect_modes).send()
        assert resp.status_code == 200

    def remove_request(self) -> Request:
        return self._client.remove_item_request(ss_id=self._ss_id, item_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204
