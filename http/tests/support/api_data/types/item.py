from __future__ import annotations

from collections import namedtuple
from typing import Union, TYPE_CHECKING

from tests.support.util import AttrDict, Absent

if TYPE_CHECKING:
    from tests.support.client import TestClient
    from tests.support.consts import State
    from tests.support.eve_data import TestObjects
    from tests.support.request import Request

AttrVals = namedtuple('AttrVals', ('base', 'dogma', 'extra'))


class Item(AttrDict):

    def __init__(self, client: TestClient, data: TestObjects, ss_id: str):
        super().__init__(
            data=data,
            hooks={'attr_vals': lambda attr_vals: {int(k): AttrVals(*v) for k, v in attr_vals.items()}})
        self._client = client
        self._ss_id = ss_id

    def update_request(self) -> Request:
        return self._client.get_item_request(ss_id=self._ss_id, item_id=self.id)

    def update(self) -> Item:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def change_mod_request(self, state: Union[State, Absent] = Absent) -> Request:
        return self._client.change_mod_request(ss_id=self._ss_id, item_id=self.id, state=state)

    def change_mod(self, state: Union[State, Absent] = Absent) -> None:
        resp = self.change_mod_request(state=state).send()
        assert resp.status_code == 200

    def remove_request(self) -> Request:
        return self._client.remove_item_request(ss_id=self._ss_id, item_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204
