import typing

from tests.fw.api.types.stats import ItemStats
from tests.fw.consts import ApiItemInfoMode
from tests.fw.util import Absent, AttrDict, AttrHookDef
from .ability_info import AbilityInfo
from .adj_count import AdjustableCount
from .attr_vals import AttrVals
from .coordinates import Coordinates
from .effect import EffectInfo
from .mod_info import AttrModInfoMap
from .movement import Movement
from .mutation import ItemMutation
from .proj_range import ProjRangeInfo
from .side_effect_info import SideEffectInfo

if typing.TYPE_CHECKING:
    from tests.fw.api import ApiClient
    from tests.fw.api.aliases import MutaAdd, MutaChange
    from tests.fw.api.types import ItemStatsOptions
    from tests.fw.consts import ApiEffMode, ApiMinionState, ApiModRmMode, ApiModuleState, ApiServiceState


class Item(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={
            'mutation': AttrHookDef(func=lambda m: ItemMutation(data=m)),
            'charge': AttrHookDef(func=lambda charge: Item(client=client, data=charge, sol_id=sol_id)),
            'autocharges': AttrHookDef(func=lambda acs: {
                k: Item(client=client, data=v, sol_id=sol_id)
                for k, v in acs.items()}),
            'spool_cycles': AttrHookDef(func=lambda sc: AdjustableCount(data=sc)),
            'count': AttrHookDef(func=lambda c: AdjustableCount(data=c)),
            'abilities': AttrHookDef(func=lambda a: {int(k): AbilityInfo(data=v) for k, v in a.items()}),
            'side_effects': AttrHookDef(func=lambda ses: {k: SideEffectInfo(data=v) for k, v in ses.items()}),
            'projs': AttrHookDef(func=lambda data: {k: ProjRangeInfo(data=v) for k, v in data}),
            'coordinates': AttrHookDef(func=lambda c: Coordinates(data=c)),
            'movement': AttrHookDef(func=lambda m: Movement(data=m)),
            'attrs': AttrHookDef(func=lambda attrs: {int(k): AttrVals(data=v) for k, v in attrs.items()}),
            'effects': AttrHookDef(func=lambda effects: {k: EffectInfo(data=v) for k, v in effects.items()}),
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

    def get_stats(
            self, *,
            options: ItemStatsOptions | type[Absent],
            status_code: int = 200,
    ) -> ItemStats | None:
        resp = self._client.get_item_stats_request(
            sol_id=self._sol_id,
            item_id=self.id,
            options=options).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        return ItemStats(data=resp.json())

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
            add_projs: list[str] | type[Absent] = Absent,
            rm_projs: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            prop_mode: str | type[Absent] = Absent,
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
            rm_projs=rm_projs,
            coordinates=coordinates,
            movement=movement,
            prop_mode=prop_mode,
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
            abilities: dict[int, bool] | type[Absent] = Absent,
            add_projs: list[str] | type[Absent] = Absent,
            rm_projs: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
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
            abilities=abilities,
            add_projs=add_projs,
            rm_projs=rm_projs,
            coordinates=coordinates,
            movement=movement,
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
            spool: str | None | type[Absent] = Absent,
            add_projs: list[str] | type[Absent] = Absent,
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
            spool=spool,
            add_projs=add_projs,
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
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        resp = self._client.change_ship_request(
            sol_id=self._sol_id,
            item_id=self.id,
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
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
