from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.consts import ApiItemInfoMode
from tests.fw.util import Absent, AttrDict, AttrHookDef
from .mod_info import AttrModInfoMap
from .side_effect_info import SideEffectInfo, SideEffectStrInfo

if typing.TYPE_CHECKING:
    from tests.fw.api import ApiClient
    from tests.fw.api.aliases import MutaAdd, MutaChange, ProjRange
    from tests.fw.consts import ApiEffMode, ApiMinionState, ApiModRmMode, ApiModuleState, ApiServiceState


@dataclass(kw_only=True)
class ItemMutation:

    base_type_id: int
    mutator_id: int
    attrs: dict[int, AttrMutation]


@dataclass(kw_only=True)
class AttrMutation:

    roll: float | None
    absolute: float


@dataclass(kw_only=True)
class AttrVals:

    base: float
    dogma: float
    extra: float


@dataclass(kw_only=True)
class EffectInfo:

    running: bool
    mode: ApiEffMode


class Item(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={
            'mutation': AttrHookDef(func=lambda m: ItemMutation(
                base_type_id=m[0],
                mutator_id=m[1],
                attrs={int(k): AttrMutation(roll=v[0], absolute=v[1]) for k, v in m[2].items()})),
            'charge': AttrHookDef(func=lambda charge: Item(client=client, data=charge, sol_id=sol_id)),
            'autocharges': AttrHookDef(func=lambda acs: {
                k: Item(client=client, data=v, sol_id=sol_id)
                for k, v in acs.items()}),
            'side_effects': AttrHookDef(func=lambda ses: {
                k: SideEffectInfo(
                    chance=v[0],
                    status=v[1],
                    str=None if v[2] is None else SideEffectStrInfo(op=v[2][0], val=v[2][1]))
                for k, v in ses.items()}),
            'attrs': AttrHookDef(func=lambda attrs: {
                int(k): AttrVals(base=v[0], dogma=v[1], extra=v[2])
                for k, v in attrs.items()}),
            'effects': AttrHookDef(func=lambda effects: {
                k: EffectInfo(running=v[0], mode=v[1])
                for k, v in effects.items()}),
            'mods': AttrHookDef(func=lambda m: AttrModInfoMap(data=m))})
        self._client = client
        self._sol_id = sol_id

    def update(
            self, *,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.full,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.get_item_request(sol_id=self._sol_id, item_id=self.id, item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def remove(
            self, *,
            mode: ApiModRmMode | type[Absent] = Absent,
            status_code: int = 204,
            json_predicate: dict | None = None,
    ) -> None:
        resp = self._client.remove_item_request(sol_id=self._sol_id, item_id=self.id, mode=mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)

    def change_autocharge(
            self, *,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_autocharge_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_booster(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            side_effects: dict[str, bool] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Item | None:
        resp = self._client.change_booster_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            side_effects=side_effects,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_character(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_character_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_charge(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_charge_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_drone(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: ApiMinionState | type[Absent] = Absent,
            mutation: MutaAdd | MutaChange | None | type[Absent] = Absent,
            add_projs: list[tuple[str, ProjRange] | str] | type[Absent] = Absent,
            change_projs: list[tuple[str, ProjRange]] | type[Absent] = Absent,
            rm_projs: list[str] | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_drone_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            add_projs=add_projs,
            change_projs=change_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_fighter(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: ApiMinionState | type[Absent] = Absent,
            count: int | None | type[Absent] = Absent,
            add_projs: list[tuple[str, ProjRange] | str] | type[Absent] = Absent,
            change_projs: list[tuple[str, ProjRange]] | type[Absent] = Absent,
            rm_projs: list[str] | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_fighter_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            count=count,
            add_projs=add_projs,
            change_projs=change_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_fw_effect(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_fw_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_implant(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_implant_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_module(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: ApiModuleState | type[Absent] = Absent,
            mutation: MutaAdd | MutaChange | None | type[Absent] = Absent,
            charge_type_id: int | None | type[Absent] = Absent,
            add_projs: list[tuple[str, ProjRange] | str] | type[Absent] = Absent,
            change_projs: list[tuple[str, ProjRange]] | type[Absent] = Absent,
            rm_projs: list[str] | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Item | None:
        resp = self._client.change_mod_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            mutation=mutation,
            charge_type_id=charge_type_id,
            add_projs=add_projs,
            change_projs=change_projs,
            rm_projs=rm_projs,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code, json_predicate=json_predicate)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_proj_effect(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            add_projs: list[str] | type[Absent] = Absent,
            rm_projs: list[str] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_proj_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            add_projs=add_projs,
            rm_projs=rm_projs,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_rig(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_rig_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_service(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: ApiServiceState | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_service_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_ship(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_ship_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_skill(
            self, *,
            type_id: int | type[Absent] = Absent,
            level: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_skill_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_stance(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_stance_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_subsystem(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_subsystem_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None

    def change_sw_effect(
            self, *,
            type_id: int | type[Absent] = Absent,
            state: bool | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_sw_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            effect_modes=effect_modes,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None
