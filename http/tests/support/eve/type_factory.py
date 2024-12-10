from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import EveEffCat, EveEffect, EveItemCat, EveItemGrp
from tests.support.util import Absent, Default
from .data_manager import EveDataManager
from .types import BuffModifier, EffectModifier

if TYPE_CHECKING:
    from typing import Type, Union

    from .containers import EveObjects


class EveTypeFactory(EveDataManager):

    def alloc_item_id(self, *, datas: Union[list[EveObjects], Type[Default]] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_item_id() for d in datas)
        while any(id_ in d.items for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_item_id(id_=id_)
        return id_

    def alloc_group_id(self, *, datas: Union[list[EveObjects], Type[Default]] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_group_id() for d in datas)
        while any(id_ in d.item_groups for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_group_id(id_=id_)
        return id_

    def alloc_attr_id(self, *, datas: Union[list[EveObjects], Type[Default]] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_attr_id() for d in datas)
        while any(id_ in d.attributes for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_attr_id(id_=id_)
        return id_

    def alloc_effect_id(self, *, datas: Union[list[EveObjects], Type[Default]] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_effect_id() for d in datas)
        while any(id_ in d.effects for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_effect_id(id_=id_)
        return id_

    def alloc_buff_id(self, *, datas: Union[list[EveObjects], Type[Default]] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_buff_id() for d in datas)
        while any(id_ in d.buffs for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_buff_id(id_=id_)
        return id_

    def mk_eve_item(
            self, *,
            datas: Union[list[EveObjects], Type[Default]] = Default,
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
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_item_id(datas=datas)
        for data in datas:
            data.mk_item(
                id_=id_,
                grp_id=grp_id,
                cat_id=cat_id,
                attrs={} if attrs is Default else attrs,
                eff_ids=[] if eff_ids is Default else eff_ids,
                defeff_id=None if defeff_id is Default else defeff_id,
                srqs={} if srqs is Default else srqs,
                capacity=0.0 if capacity is Default else capacity,
                mass=0.0 if mass is Default else mass,
                radius=0.0 if radius is Default else radius,
                volume=0.0 if volume is Default else volume)
        return id_

    def mk_eve_ship(
            self, *,
            datas: Union[list[EveObjects], Type[Default]] = Default,
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
    ) -> int:
        return self.mk_eve_item(
            datas=datas,
            id_=id_,
            grp_id=grp_id,
            cat_id=EveItemCat.ship,
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
            datas: Union[list[EveObjects], Type[Default]] = Default,
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
    ) -> int:
        return self.mk_eve_item(
            datas=datas,
            id_=id_,
            grp_id=grp_id,
            cat_id=EveItemCat.structure,
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
            datas: Union[list[EveObjects], Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_group_id(datas=datas)
        for data in datas:
            data.mk_item_group(
                id_=id_,
                cat_id=EveItemCat.module if cat_id is Default else cat_id)
        return id_

    def mk_eve_ship_group(
            self, *,
            datas: Union[list[EveObjects], Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
    ) -> int:
        return self.mk_eve_item_group(datas=datas, id_=id_, cat_id=EveItemCat.ship)

    def mk_eve_struct_group(
            self, *,
            datas: Union[list[EveObjects], Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
    ) -> int:
        return self.mk_eve_item_group(datas=datas, id_=id_, cat_id=EveItemCat.structure)

    def mk_eve_attr(
            self, *,
            datas: Union[list[EveObjects], Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            stackable: Union[int, bool, Type[Absent], Type[Default]] = Default,
            high_is_good: Union[int, bool, Type[Absent], Type[Default]] = Default,
            def_val: Union[float, Type[Absent], Type[Default]] = Default,
            min_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            max_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_attr_id(datas=datas)
        for data in datas:
            data.mk_attr(
                id_=id_,
                stackable=1 if stackable is Default else stackable,
                high_is_good=1 if high_is_good is Default else high_is_good,
                def_val=0.0 if def_val is Default else def_val,
                min_attr_id=Absent if min_attr_id is Default else min_attr_id,
                max_attr_id=Absent if max_attr_id is Default else max_attr_id)
        return id_

    def mk_eve_effect(
            self, *,
            datas: Union[list[EveObjects], Type[Default]] = Default,
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
    ) -> id:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_effect_id(datas=datas)
        for data in datas:
            data.mk_effect(
                id_=id_,
                cat_id=EveEffCat.passive if cat_id is Default else cat_id,
                is_assistance=0 if is_assistance is Default else is_assistance,
                is_offensive=0 if is_offensive is Default else is_offensive,
                discharge_attr_id=Absent if discharge_attr_id is Default else discharge_attr_id,
                duration_attr_id=Absent if duration_attr_id is Default else duration_attr_id,
                range_attr_id=Absent if range_attr_id is Default else range_attr_id,
                falloff_attr_id=Absent if falloff_attr_id is Default else falloff_attr_id,
                tracking_attr_id=Absent if tracking_attr_id is Default else tracking_attr_id,
                chance_attr_id=Absent if chance_attr_id is Default else chance_attr_id,
                resist_attr_id=Absent if resist_attr_id is Default else resist_attr_id,
                mod_info=Absent if mod_info is Default else mod_info)
        return id_

    def mk_eve_online_effect(self, *, datas: Union[list[EveObjects], Type[Default]] = Default) -> int:
        return self.mk_eve_effect(
            datas=datas,
            id_=EveEffect.online,
            cat_id=EveEffCat.active,
            is_assistance=0,
            is_offensive=0,
            discharge_attr_id=Absent,
            duration_attr_id=Absent,
            range_attr_id=Absent,
            falloff_attr_id=Absent,
            tracking_attr_id=Absent,
            chance_attr_id=Absent,
            resist_attr_id=Absent,
            mod_info=Absent)

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
            datas: Union[list[EveObjects], Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            aggr_mode: Union[str, Type[Absent], Type[Default]] = Default,
            op: Union[str, Type[Absent], Type[Default]] = Default,
            item_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_grp_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_srq_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_buff_id(datas=datas)
        for data in datas:
            data.mk_buff(
                id_=id_,
                aggr_mode=Absent if aggr_mode is Default else aggr_mode,
                op=Absent if op is Default else op,
                item_mods=Absent if item_mods is Default else item_mods,
                loc_mods=Absent if loc_mods is Default else loc_mods,
                loc_grp_mods=Absent if loc_grp_mods is Default else loc_grp_mods,
                loc_srq_mods=Absent if loc_srq_mods is Default else loc_srq_mods)
        return id_

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
            datas: Union[list[EveObjects], Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            items: Union[list[tuple[list[int], int]], Type[Absent], Type[Default]] = Default,
            attributes: Union[dict[int, tuple[float, float]], Type[Absent], Type[Default]] = Default,
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        # Mutators are a special case of an item, allocate an item ID and create item for it
        id_ = self.mk_eve_item(
            datas=datas,
            id_=id_,
            grp_id=EveItemGrp.mutaplasmids,
            cat_id=EveItemCat.commodity)
        for data in datas:
            data.mk_mutator(
                id_=id_,
                items=[] if items is Default else items,
                attributes={} if attributes is Default else attributes)
        return id_
