from __future__ import annotations

from collections import namedtuple
from typing import TYPE_CHECKING

from tests.support.util import Absent, AttrDict, AttrHookDef
from .mod_info import AttrModInfoMap

if TYPE_CHECKING:
    from collections.abc import Iterable
    from typing import Type, Union

    from tests.support.api import ApiClient
    from tests.support.consts import ApiEffMode, ApiState
    from tests.support.request import Request

AttrVals = namedtuple('AttrVals', ('base', 'dogma', 'extra'))
EffectInfo = namedtuple('EffectInfo', ('running', 'mode'))


class Item(AttrDict):

    def __init__(self, client: ApiClient, data: dict, sol_id: str):
        super().__init__(
            data=data,
            hooks={
                'charge': AttrHookDef(func=lambda charge: Item(client=client, data=charge, sol_id=sol_id)),
                'attrs': AttrHookDef(func=lambda attrs: {int(k): AttrVals(*v) for k, v in attrs.items()}, default={}),
                'effects': AttrHookDef(
                    func=lambda effects: {int(k): EffectInfo(*v) for k, v in effects.items()}, default={}),
                'mods': AttrHookDef(func=AttrModInfoMap, default={})})
        self._client = client
        self._sol_id = sol_id

    def update_request(self) -> Request:
        return self._client.get_item_request(sol_id=self._sol_id, item_id=self.id)

    def update(self) -> Item:
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)
        self._data = resp.json()
        return self

    def remove_request(self) -> Request:
        return self._client.remove_item_request(sol_id=self._sol_id, item_id=self.id)

    def remove(self) -> None:
        resp = self.remove_request().send()
        assert resp.status_code == 204
        self._client.check_sol(sol_id=self._sol_id)

    # Character methods
    def change_char_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_char_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state)

    def change_char(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_char_request(state=state).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Skill methods
    def change_skill_request(
            self,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_skill_request(
            sol_id=self._sol_id, item_id=self.id, level=level, state=state, effect_modes=effect_modes)

    def change_skill(
            self,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_skill_request(level=level, state=state, effect_modes=effect_modes).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Implant methods
    def change_implant_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_implant_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state)

    def change_implant(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_implant_request(state=state).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Ship methods
    def change_ship_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_ship_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state)

    def change_ship(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_ship_request(state=state).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Module methods
    def change_mod_request(
            self,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, None, Type[Absent]] = Absent,
            add_tgts: Union[Iterable[(str, Union[float, None])], Type[Absent]] = Absent,
            rm_tgts: Union[Iterable[str], Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_mod_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            charge=charge,
            add_tgts=add_tgts,
            rm_tgts=rm_tgts,
            effect_modes=effect_modes)

    def change_mod(
            self,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, Type[Absent]] = Absent,
            add_tgts: Union[Iterable[(str, Union[float, None])], Type[Absent]] = Absent,
            rm_tgts: Union[Iterable[str], Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_mod_request(
            state=state,
            charge=charge,
            add_tgts=add_tgts,
            rm_tgts=rm_tgts,
            effect_modes=effect_modes).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Rig methods
    def change_rig_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_rig_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state)

    def change_rig(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_rig_request(state=state).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # System-wide effect methods
    def change_sw_effect_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_sw_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state)

    def change_sw_effect(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_sw_effect_request(state=state).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Fit-wide effect methods
    def change_fw_effect_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_fw_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state)

    def change_fw_effect(
            self,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_fw_effect_request(state=state).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)

    # Projected effect methods
    def change_proj_effect_request(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            add_tgts: Union[Iterable[str], Type[Absent]] = Absent,
            rm_tgts: Union[Iterable[str], Type[Absent]] = Absent,
    ) -> Request:
        return self._client.change_proj_effect_request(
            sol_id=self._sol_id,
            item_id=self.id,
            state=state,
            add_tgts=add_tgts,
            rm_tgts=rm_tgts)

    def change_proj_effect(
            self,
            state: Union[bool, Type[Absent]] = Absent,
            add_tgts: Union[Iterable[str], Type[Absent]] = Absent,
            rm_tgts: Union[Iterable[str], Type[Absent]] = Absent,
    ) -> None:
        resp = self.change_proj_effect_request(state=state, add_tgts=add_tgts, rm_tgts=rm_tgts).send()
        assert resp.status_code == 200
        self._client.check_sol(sol_id=self._sol_id)
