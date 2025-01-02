from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import ApiFitInfoMode, ApiItemInfoMode, ApiModAddMode, ApiRack, ApiState
from tests.support.util import Absent, AttrDict, AttrHookDef
from .dmg_types import DmgTypes
from .item import Item

if TYPE_CHECKING:
    from typing import Union

    from tests.support.api import ApiClient


class Fit(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str):
        super().__init__(data=data, hooks={
            'rah_incoming_dmg': AttrHookDef(
                func=lambda dp: DmgTypes(em=dp[0], thermal=dp[1], kinetic=dp[2], explosive=dp[3]))})
        self._client = client
        self._sol_id = sol_id

    def update(
            self, *,
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Union[Fit, None]:
        resp = self._client.get_fit_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove(self, *, status_code: int = 204) -> None:
        resp = self._client.remove_fit_request(sol_id=self._sol_id, fit_id=self.id).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def set_fleet(
            self, *,
            fleet_id: Union[str, None],
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> None:
        resp = self._client.set_fit_fleet_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            fleet_id=fleet_id,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def set_rah_incoming_dmg(
            self, *,
            dmg_profile: Union[tuple[float, float, float, float], None, type[Absent]],
            fit_info_mode: Union[ApiFitInfoMode, type[Absent]] = ApiFitInfoMode.full,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> None:
        resp = self._client.set_fit_rah_incoming_dmg_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            dmg_profile=dmg_profile,
            fit_info_mode=fit_info_mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    # Item methods
    def remove_item(self, *, item_id: str, status_code: int = 204) -> None:
        resp = self._client.remove_item_request(sol_id=self._sol_id, item_id=item_id).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)

    def set_char(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.set_char_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_skill(
            self, *,
            type_id: int,
            level: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_skill_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            level=level,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_implant(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_implant_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_booster(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            side_effects: Union[dict[int, bool], type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_booster_request(
            sol_id=self._sol_id,
            fit_id=self.id,
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

    def set_ship(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.set_ship_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def set_stance(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.set_stance_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_subsystem(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_subsystem_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_mod(
            self, *,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: ApiState = ApiState.offline,
            mutation: Union[int, tuple[int, dict[int, dict[str, float]]], type[Absent]] = Absent,
            charge_type_id: Union[int, type[Absent]] = Absent,
            mode: ApiModAddMode = ApiModAddMode.equip,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_mod_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            rack=rack,
            type_id=type_id,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            mode=mode,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_rig(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_rig_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_drone(
            self, *,
            type_id,
            state: ApiState = ApiState.offline,
            mutation: Union[int, tuple[int, dict[int, dict[str, float]]], type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_drone_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_fighter(
            self, *,
            type_id,
            state: ApiState = ApiState.offline,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_fighter_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None

    def add_fw_effect(
            self, *,
            type_id: int,
            state: Union[bool, type[Absent]] = Absent,
            item_info_mode: Union[ApiItemInfoMode, type[Absent]] = ApiItemInfoMode.id,
            status_code: int = 201,
    ) -> Union[Item, None]:
        resp = self._client.add_fw_effect_request(
            sol_id=self._sol_id,
            fit_id=self.id,
            type_id=type_id,
            state=state,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 201:
            item = Item(client=self._client, data=resp.json(), sol_id=self._sol_id)
            return item
        return None
