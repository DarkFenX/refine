import typing

from fw.api.commands import (
    ItemAutochargeChangeCmd,
    ItemBoosterChangeCmd,
    ItemCharacterChangeCmd,
    ItemChargeChangeCmd,
    ItemDroneChangeCmd,
    ItemFighterChangeCmd,
    ItemFwEffectChangeCmd,
    ItemImplantChangeCmd,
    ItemModuleChangeCmd,
    ItemProjEffectChangeCmd,
    ItemRigChangeCmd,
    ItemServiceChangeCmd,
    ItemShipChangeCmd,
    ItemSkillChangeCmd,
    ItemStanceChangeCmd,
    ItemSubsystemChangeCmd,
    ItemSwEffectChangeCmd,
)
from fw.api.types.helpers import (
    attr_http_to_fw,
    effect_http_to_fw,
    process_effect_map_request,
    process_muta_change_request,
)
from fw.api.types.stats import ItemStats
from fw.consts import ApiItemInfoMode
from fw.util import Absent, AttrDict, AttrHookDef
from .ability_info import AbilityInfo
from .attr_vals import AttrVals
from .coordinates import Coordinates
from .count import ItemCountInfo
from .effect import EffectInfo
from .mod_info import AttrModInfoMap
from .movement import Movement
from .mutation import ItemMutation
from .npc_prop import ItemNpcPropInfo
from .optional_reload import ItemOptionalReloadInfo
from .proj_range import ProjRangeInfo
from .rearm_minion import ItemRearmMinionInfo
from .side_effect_info import SideEffectInfo

if typing.TYPE_CHECKING:
    from fw.api import ApiClient
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.api.types import ItemStatsOptions
    from fw.consts import (
        ApiEffMode,
        ApiMinionState,
        ApiModRmMode,
        ApiModuleState,
        ApiNpcProp,
        ApiOptionalReload,
        ApiRearmMinion,
        ApiServiceState,
    )


class Item(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict, sol_id: str) -> None:
        super().__init__(data=data, hooks={
            'mutation': AttrHookDef(func=lambda d: ItemMutation(data=d)),
            'charge': AttrHookDef(func=lambda d: Item(client=client, data=d, sol_id=sol_id)),
            'autocharges': AttrHookDef(func=lambda d: {
                effect_http_to_fw(effect_id=k): Item(client=client, data=v, sol_id=sol_id)
                for k, v in d.items()}),
            'spool_cycles': AttrHookDef(func=lambda d: ItemCountInfo(data=d)),
            'count': AttrHookDef(func=lambda d: ItemCountInfo(data=d)),
            'optional_reload': AttrHookDef(func=lambda d: ItemOptionalReloadInfo(data=d)),
            'rearm_minion': AttrHookDef(func=lambda d: ItemRearmMinionInfo(data=d)),
            'npc_prop': AttrHookDef(func=lambda d: ItemNpcPropInfo(data=d)),
            'abilities': AttrHookDef(func=lambda a: {int(k): AbilityInfo(data=v) for k, v in a.items()}),
            'side_effects': AttrHookDef(func=lambda ses: {
                effect_http_to_fw(effect_id=k): SideEffectInfo(data=v) for k, v in ses.items()}),
            'projs': AttrHookDef(func=lambda data: {k: ProjRangeInfo(data=v) for k, v in data}),
            'coordinates': AttrHookDef(func=lambda c: Coordinates(data=c)),
            'movement': AttrHookDef(func=lambda m: Movement(data=m)),
            'attrs': AttrHookDef(func=lambda attrs: {
                attr_http_to_fw(attr_id=k): AttrVals(data=v) for k, v in attrs.items()}),
            'effects': AttrHookDef(func=lambda effects: {
                effect_http_to_fw(effect_id=k): EffectInfo(data=v) for k, v in effects.items()}),
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
            rm_mode: ApiModRmMode | type[Absent] = Absent,
            status_code: int = 204,
            json_predicate: dict | None = None,
    ) -> None:
        resp = self._client.remove_item_request(sol_id=self._sol_id, item_id=self.id, rm_mode=rm_mode).send()
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemAutochargeChangeCmd(
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            side_effects: dict[int | str, bool] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Item | None:
        command = ItemBoosterChangeCmd(
            type_id=type_id,
            state=state,
            side_effects=process_effect_map_request(effect_map=side_effects),
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemCharacterChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemChargeChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            mutation: MutaAdd | MutaChange | type[Absent] | None = Absent,
            npc_prop: ApiNpcProp | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemDroneChangeCmd(
            type_id=type_id,
            state=state,
            mutation=process_muta_change_request(mutation=mutation),
            npc_prop=npc_prop,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            count: int | type[Absent] | None = Absent,
            abilities: dict[int, bool] | type[Absent] = Absent,
            rearm_minion: ApiRearmMinion | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            coordinates: tuple[float, float, float] | type[Absent] = Absent,
            movement: tuple[float, float, float] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemFighterChangeCmd(
            type_id=type_id,
            state=state,
            count=count,
            abilities=abilities,
            rearm_minion=rearm_minion,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemFwEffectChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemImplantChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            mutation: MutaAdd | MutaChange | type[Absent] | None = Absent,
            charge_type_id: int | type[Absent] | None = Absent,
            spool: str | type[Absent] | None = Absent,
            optional_reload: ApiOptionalReload | type[Absent] | None = Absent,
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
            json_predicate: dict | None = None,
    ) -> Item | None:
        command = ItemModuleChangeCmd(
            type_id=type_id,
            state=state,
            mutation=process_muta_change_request(mutation=mutation),
            charge_type_id=charge_type_id,
            spool=spool,
            optional_reload=optional_reload,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            add_proj_item_ids: list[str] | type[Absent] = Absent,
            rm_proj_item_ids: list[str] | type[Absent] = Absent,
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemProjEffectChangeCmd(
            type_id=type_id,
            state=state,
            add_proj_item_ids=add_proj_item_ids,
            rm_proj_item_ids=rm_proj_item_ids,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemRigChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemServiceChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemShipChangeCmd(
            type_id=type_id,
            state=state,
            coordinates=coordinates,
            movement=movement,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemSkillChangeCmd(
            type_id=type_id,
            level=level,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemStanceChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemSubsystemChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
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
            effect_modes: dict[int | str, ApiEffMode] | type[Absent] = Absent,
            item_info_mode: ApiItemInfoMode | type[Absent] = ApiItemInfoMode.id,
            status_code: int = 200,
    ) -> Item | None:
        command = ItemSwEffectChangeCmd(
            type_id=type_id,
            state=state,
            effect_modes=process_effect_map_request(effect_map=effect_modes))
        resp = self._client.item_command_change_request(
            sol_id=self._sol_id,
            item_id=self.id,
            command=command,
            item_info_mode=item_info_mode).send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=status_code)
        if resp.status_code == 200:
            self._data = resp.json()
            return self
        return None
