from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import Absent, Default
from .data_manager import EveDataManager
from .types import BuffModifier, EffectModifier

if TYPE_CHECKING:
    from typing import Type, Union

    from .containers import EveObjects
    from .types import Attribute, Buff, Effect, Group, Item, Mutator


class EveTypeFactory(EveDataManager):

    def alloc_item_id(self, *, data: Union[EveObjects, Type[Default]] = Default) -> int:
        data = self._get_eve_data(data=data)
        return data.alloc_item_id()

    def alloc_group_id(self, *, data: Union[EveObjects, Type[Default]] = Default) -> int:
        data = self._get_eve_data(data=data)
        return data.alloc_group_id()

    def alloc_attr_id(self, *, data: Union[EveObjects, Type[Default]] = Default) -> int:
        data = self._get_eve_data(data=data)
        return data.alloc_attr_id()

    def alloc_effect_id(self, *, data: Union[EveObjects, Type[Default]] = Default) -> int:
        data = self._get_eve_data(data=data)
        return data.alloc_effect_id()

    def alloc_buff_id(self, *, data: Union[EveObjects, Type[Default]] = Default) -> int:
        data = self._get_eve_data(data=data)
        return data.alloc_buff_id()

    def mk_eve_item(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            grp_id: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
            attrs: Union[dict[int, float], Type[Absent], Type[Default]] = Default,
            eff_ids: Union[list[int], tuple[int], Type[Absent], Type[Default]] = Default,
            defeff_id: Union[int, None, Type[Absent], Type[Default]] = Default,
            srqs: Union[dict[int, int], Type[Absent], Type[Default]] = Default,
            capacity: Union[float, Type[Absent], Type[Default]] = Default,
            mass: Union[float, Type[Absent], Type[Default]] = Default,
            radius: Union[float, Type[Absent], Type[Default]] = Default,
            volume: Union[float, Type[Absent], Type[Default]] = Default,
    ) -> Item:
        data = self._get_eve_data(data=data)
        return data.mk_item(
            id_=id_,
            grp_id=grp_id,
            cat_id=cat_id,
            attrs=attrs,
            eff_ids=eff_ids,
            defeff_id=defeff_id,
            srqs=srqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)

    def mk_eve_ship(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            grp_id: Union[int, Type[Default]] = Default,
            attrs: Union[dict[int, float], Type[Absent], Type[Default]] = Default,
            eff_ids: Union[list[int], tuple[int], Type[Absent], Type[Default]] = Default,
            defeff_id: Union[int, None, Type[Absent], Type[Default]] = Default,
            srqs: Union[dict[int, int], Type[Absent], Type[Default]] = Default,
            capacity: Union[float, Type[Absent], Type[Default]] = Default,
            mass: Union[float, Type[Absent], Type[Default]] = Default,
            radius: Union[float, Type[Absent], Type[Default]] = Default,
            volume: Union[float, Type[Absent], Type[Default]] = Default,
    ) -> Item:
        data = self._get_eve_data(data=data)
        return data.mk_ship(
            id_=id_,
            grp_id=grp_id,
            attrs=attrs,
            eff_ids=eff_ids,
            defeff_id=defeff_id,
            srqs=srqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)

    def mk_eve_struct(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            grp_id: Union[int, Type[Default]] = Default,
            attrs: Union[dict[int, float], Type[Absent], Type[Default]] = Default,
            eff_ids: Union[list[int], tuple[int], Type[Absent], Type[Default]] = Default,
            defeff_id: Union[int, None, Type[Absent], Type[Default]] = Default,
            srqs: Union[dict[int, int], Type[Absent], Type[Default]] = Default,
            capacity: Union[float, Type[Absent], Type[Default]] = Default,
            mass: Union[float, Type[Absent], Type[Default]] = Default,
            radius: Union[float, Type[Absent], Type[Default]] = Default,
            volume: Union[float, Type[Absent], Type[Default]] = Default,
    ) -> Item:
        data = self._get_eve_data(data=data)
        return data.mk_struct(
            id_=id_,
            grp_id=grp_id,
            attrs=attrs,
            eff_ids=eff_ids,
            defeff_id=defeff_id,
            srqs=srqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)

    def mk_eve_item_group(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
    ) -> Group:
        data = self._get_eve_data(data=data)
        return data.mk_item_group(id_=id_, cat_id=cat_id)

    def mk_eve_ship_group(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
    ) -> Group:
        data = self._get_eve_data(data=data)
        return data.mk_ship_group(id_=id_)

    def mk_eve_struct_group(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
    ) -> Group:
        data = self._get_eve_data(data=data)
        return data.mk_struct_group(id_=id_)

    def mk_eve_attr(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            stackable: Union[int, bool, Type[Absent], Type[Default]] = Default,
            high_is_good: Union[int, bool, Type[Absent], Type[Default]] = Default,
            def_val: Union[float, Type[Absent], Type[Default]] = Default,
            min_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            max_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
    ) -> Attribute:
        data = self._get_eve_data(data=data)
        return data.mk_attr(
            id_=id_,
            stackable=stackable,
            high_is_good=high_is_good,
            def_val=def_val,
            min_attr_id=min_attr_id,
            max_attr_id=max_attr_id)

    def mk_eve_effect(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
            is_assistance: Union[int, bool, Type[Absent], Type[Default]] = Default,
            is_offensive: Union[int, bool, Type[Absent], Type[Default]] = Default,
            discharge_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            duration_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            range_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            falloff_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            tracking_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            chance_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            resist_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            mod_info: Union[list[EffectModifier], tuple[EffectModifier], Type[Absent], Type[Default]] = Default,
    ) -> Effect:
        data = self._get_eve_data(data=data)
        return data.mk_effect(
            id_=id_,
            cat_id=cat_id,
            is_assistance=is_assistance,
            is_offensive=is_offensive,
            discharge_attr_id=discharge_attr_id,
            duration_attr_id=duration_attr_id,
            range_attr_id=range_attr_id,
            falloff_attr_id=falloff_attr_id,
            tracking_attr_id=tracking_attr_id,
            chance_attr_id=chance_attr_id,
            resist_attr_id=resist_attr_id,
            mod_info=mod_info)

    def mk_eve_online_effect(self, *, data: Union[EveObjects, Type[Default]] = Default) -> Effect:
        data = self._get_eve_data(data=data)
        return data.mk_online_effect()

    @staticmethod
    def mk_eve_effect_mod(
            *,
            func: Union[str, Type[Absent]] = Absent,
            dom: Union[str, Type[Absent]] = Absent,
            grp: Union[int, Type[Absent]] = Absent,
            srq: Union[int, Type[Absent]] = Absent,
            op: Union[int, Type[Absent]] = Absent,
            affector_attr_id: Union[int, Absent] = Absent,
            affectee_attr_id: Union[int, Absent] = Absent,
    ) -> EffectModifier:
        return EffectModifier(
            func=func,
            domain=dom,
            group=grp,
            skill_req=srq,
            operation=op,
            affector_attr_id=affector_attr_id,
            affectee_attr_id=affectee_attr_id)

    def mk_eve_buff(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            aggr_mode: Union[str, Type[Absent], Type[Default]] = Default,
            op: Union[str, Type[Absent], Type[Default]] = Default,
            item_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_grp_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_srq_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
    ) -> Buff:
        data = self._get_eve_data(data=data)
        return data.mk_buff(
            id_=id_,
            aggr_mode=aggr_mode,
            op=op,
            item_mods=item_mods,
            loc_mods=loc_mods,
            loc_grp_mods=loc_grp_mods,
            loc_srq_mods=loc_srq_mods)

    @staticmethod
    def mk_eve_buff_mod(
            *,
            attr_id: Union[int, Type[Absent]] = Absent,
            group_id: Union[int, Type[Absent]] = Absent,
            skill_id: Union[int, Type[Absent]] = Absent,
    ) -> BuffModifier:
        return BuffModifier(
            attr_id=attr_id,
            group_id=group_id,
            skill_id=skill_id)

    def mk_eve_mutator(
            self, *,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            items: Union[list[tuple[Union[list[int], Type[Absent]], Union[int, Type[Absent]]]], Type[Absent], Type[Default]] = Default,
            attributes: Union[dict[int, tuple[Union[float, Type[Absent]], Union[float, Type[Absent]]]], Type[Absent], Type[Default]] = Default,
    ) -> Mutator:
        data = self._get_eve_data(data=data)
        return data.mk_mutator(
            id_=id_,
            items=items,
            attributes=attributes)
