from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import ApiRack, ApiState
from tests.support.util import AttrDict, Absent
from .item import Item

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.api import ApiClient
    from tests.support.eve import EveObjects
    from tests.support.request import Request


class Fit(AttrDict):

    def __init__(self, client: ApiClient, data: EveObjects, ss_id: str):
        super().__init__(data=data)
        self._client = client
        self._ss_id = ss_id

    def update_request(self) -> Request:
        return self._client.update_fit_request(ss_id=self._ss_id, fit_id=self.id)

    def update(self) -> Fit:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def remove_request(self) -> Request:
        return self._client.remove_fit_request(ss_id=self._ss_id, fit_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204

    # Generic item methods
    def remove_item_request(self, item_id: str) -> Request:
        return self._client.remove_item_request(ss_id=self._ss_id, item_id=item_id)

    def remove_item(self, item_id: str) -> None:
        resp = self.remove_item_request(item_id=item_id).send()
        assert resp.status_code == 204

    # Character methods
    def set_char_request(self, type_id: int) -> Request:
        return self._client.set_char_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id)

    def set_char(self, type_id: int) -> Item:
        resp = self.set_char_request(type_id=type_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Skill methods
    def add_skill_request(
            self,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_skill_request(
            ss_id=self._ss_id, fit_id=self.id, type_id=type_id, level=level, state=state)

    def add_skill(
            self,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_skill_request(type_id=type_id, level=level, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Implant methods
    def add_implant_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_implant_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_implant(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_implant_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Booster methods
    def add_booster_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_booster_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_booster(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_booster_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Ship methods
    def set_ship_request(self, type_id: int) -> Request:
        return self._client.set_ship_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id)

    def set_ship(self, type_id: int) -> Item:
        resp = self.set_ship_request(type_id=type_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Structure methods
    def set_struct_request(self, type_id: int) -> Request:
        return self._client.set_struct_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id)

    def set_struct(self, type_id: int) -> Item:
        resp = self.set_struct_request(type_id=type_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Stance methods
    def set_stance_request(self, type_id: int) -> Request:
        return self._client.set_stance_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id)

    def set_stance(self, type_id: int) -> Item:
        resp = self.set_stance_request(type_id=type_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Subsystem methods
    def add_subsystem_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_subsystem_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_subsystem(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_subsystem_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Module methods
    def add_mod_request(
            self,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: str = ApiState.offline,
            charge_type_id: Union[int, Type[Absent]] = Absent,
            mode: str = 'equip',
    ) -> Request:
        return self._client.add_mod_request(
            ss_id=self._ss_id, fit_id=self.id, rack=rack, type_id=type_id,
            state=state, charge_type_id=charge_type_id, mode=mode)

    def add_mod(
            self,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: str = ApiState.offline,
            charge_type_id: Union[int, Type[Absent]] = Absent,
            mode: str = 'equip',
    ) -> Item:
        resp = self.add_mod_request(
            rack=rack,
            type_id=type_id,
            state=state,
            charge_type_id=charge_type_id,
            mode=mode).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Rig methods
    def add_rig_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_rig_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_rig(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_rig_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Drone methods
    def add_drone_request(
            self,
            type_id: int,
            state: str = ApiState.offline,
    ) -> Request:
        return self._client.add_drone_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_drone(
            self,
            type_id,
            state: str = ApiState.offline,
    ) -> Item:
        resp = self.add_drone_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Fighter methods
    def add_fighter_request(
            self,
            type_id: int,
            state: str = ApiState.offline,
    ) -> Request:
        return self._client.add_fighter_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_fighter(
            self,
            type_id,
            state: str = ApiState.offline,
    ) -> Item:
        resp = self.add_fighter_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    # Fit-wide effect methods
    def add_fw_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.add_fw_effect_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_fw_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Item:
        resp = self.add_fw_effect_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item
