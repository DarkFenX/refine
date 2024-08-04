from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import ApiFitInfoMode, ApiItemInfoMode, ApiModAddMode, ApiRack, ApiState
from tests.support.util import AttrDict, Absent
from .item import Item

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.api import ApiClient
    from tests.support.request import Request


class Fit(AttrDict):

    def __init__(self, client: ApiClient, data: dict, sol_id: str):
        super().__init__(data=data)
        self._client = client
        self._sol_id = sol_id

    def update_request(
            self,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.get_fit_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def update(
            self,
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Fit, None]:
        resp = self.update_request(fit_info_mode=fit_info_mode, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove_request(self) -> Request:
        return self._client.remove_fit_request(sol_id=self._sol_id, fit_id=self.id)

    def remove(self, status_code: int = 204) -> None:
        resp = self.remove_request().send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    # Fleet methods
    def set_fleet_request(
            self,
            fleet_id: Union[str, None],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.set_fit_fleet_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fleet_id=fleet_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode)

    def set_fleet(
            self,
            fleet_id: Union[str, None],
            fit_info_mode: Union[ApiFitInfoMode, Type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> None:
        resp = self.set_fleet_request(
            fleet_id=fleet_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    # Generic item methods
    def remove_item_request(self, item_id: str) -> Request:
        return self._client.remove_item_request(sol_id=self._sol_id, item_id=item_id)

    def remove_item(self, item_id: str, status_code: int = 204) -> None:
        resp = self.remove_item_request(item_id=item_id).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    # Character methods
    def set_char_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.set_char_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def set_char(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.set_char_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Skill methods
    def add_skill_request(
            self,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_skill_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            level=level,
            state=state,
            item_info_mode=item_info_mode)

    def add_skill(
            self,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_skill_request(type_id=type_id, level=level, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Implant methods
    def add_implant_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_implant_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_implant(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_implant_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Booster methods
    def add_booster_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            side_effects: Union[dict[int, bool], Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_booster_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            item_info_mode=item_info_mode)

    def add_booster(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            side_effects: Union[dict[int, bool], Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_booster_request(
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Ship methods
    def set_ship_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.set_ship_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def set_ship(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.set_ship_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Stance methods
    def set_stance_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.set_stance_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def set_stance(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.set_stance_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Subsystem methods
    def add_subsystem_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_subsystem_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_subsystem(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_subsystem_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Module methods
    def add_mod_request(
            self,
            type_id: int,
            rack: ApiRack,
            state: ApiState,
            charge_type_id: Union[int, Type[Absent]],
            mode: ApiModAddMode,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_mod_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            rack=rack,
            type_id=type_id,
            state=state,
            charge_type_id=charge_type_id,
            mode=mode,
            item_info_mode=item_info_mode)

    def add_mod(
            self,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: ApiState = ApiState.offline,
            charge_type_id: Union[int, Type[Absent]] = Absent,
            mode: ApiModAddMode = ApiModAddMode.equip,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_mod_request(
            rack=rack,
            type_id=type_id,
            state=state,
            charge_type_id=charge_type_id,
            mode=mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Rig methods
    def add_rig_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_rig_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_rig(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_rig_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Drone methods
    def add_drone_request(
            self,
            type_id: int,
            state: ApiState,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_drone_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_drone(
            self,
            type_id,
            state: ApiState = ApiState.offline,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_drone_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Fighter methods
    def add_fighter_request(
            self,
            type_id: int,
            state: ApiState,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_fighter_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_fighter(
            self,
            type_id,
            state: ApiState = ApiState.offline,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_fighter_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    # Fit-wide effect methods
    def add_fw_effect_request(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]],
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]],
    ) -> Request:
        return self._client.add_fw_effect_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode)

    def add_fw_effect(
            self,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, Type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self.add_fw_effect_request(type_id=type_id, state=state, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None
