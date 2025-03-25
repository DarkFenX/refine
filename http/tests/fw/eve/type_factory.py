from __future__ import annotations

import typing

from tests.fw.consts import EveEffCat, EveEffect, EveItemCat, EveItemGrp
from tests.fw.util import Absent, Default
from .data_manager import EveDataManager
from .types import BuffModifier, EffectModifier

if typing.TYPE_CHECKING:
    from .containers import EveObjects


class EveTypeFactory(EveDataManager):

    def alloc_item_id(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_item_id() for d in datas)
        while any(id_ in d.items for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_item_id(id_=id_)
        return id_

    def alloc_group_id(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_group_id() for d in datas)
        while any(id_ in d.item_groups for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_group_id(id_=id_)
        return id_

    def alloc_item_list_id(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_item_list_id() for d in datas)
        while any(id_ in d.item_lists for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_item_list_id(id_=id_)
        return id_

    def alloc_attr_id(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_attr_id() for d in datas)
        while any(id_ in d.attributes for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_attr_id(id_=id_)
        return id_

    def alloc_effect_id(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        id_ = max(d.prealloc_effect_id() for d in datas)
        while any(id_ in d.effects for d in datas):
            id_ += 1
        for data in datas:
            data.alloc_effect_id(id_=id_)
        return id_

    def alloc_buff_id(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
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
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            grp_id: int | type[Default] = Default,
            cat_id: int | type[Absent | Default] = Default,
            attrs: dict[int, float] | type[Absent | Default] = Default,
            eff_ids: list[int] | type[Absent | Default] = Default,
            defeff_id: int | None | type[Absent | Default] = Default,
            srqs: dict[int, int] | type[Absent | Default] = Default,
            capacity: float | type[Absent | Default] = Default,
            mass: float | type[Absent | Default] = Default,
            radius: float | type[Absent | Default] = Default,
            volume: float | type[Absent | Default] = Default,
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_item_id(datas=datas)

        def fetch_or_make_group_id() -> int:
            # Fetch existing group if consistency is not broken:
            # - when requested category ID and group's category ID match
            # - when default category ID is requested, and we have just one group
            if grp_id in data.item_groups:
                group_list = data.item_groups[grp_id]
                if len(group_list) == 1:
                    group = group_list[0]
                    if cat_id is Default or cat_id == group.category_id:
                        return group.id
            # Couldn't find a fitting group - create new one
            group = data.mk_item_group(
                id_=data.prealloc_group_id() if grp_id is Default else grp_id,
                cat_id=EveItemCat.module if cat_id is Default else cat_id)
            return group.id

        for data in datas:
            data.mk_item(
                id_=id_,
                grp_id=fetch_or_make_group_id(),
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
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            grp_id: int | type[Default] = Default,
            attrs: dict[int, float] | type[Absent | Default] = Default,
            eff_ids: list[int] | type[Absent | Default] = Default,
            defeff_id: int | None | type[Absent | Default] = Default,
            srqs: dict[int, int] | type[Absent | Default] = Default,
            capacity: float | type[Absent | Default] = Default,
            mass: float | type[Absent | Default] = Default,
            radius: float | type[Absent | Default] = Default,
            volume: float | type[Absent | Default] = Default,
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
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            grp_id: int | type[Default] = Default,
            attrs: dict[int, float] | type[Absent | Default] = Default,
            eff_ids: list[int] | type[Absent | Default] = Default,
            defeff_id: int | None | type[Absent | Default] = Default,
            srqs: dict[int, int] | type[Absent | Default] = Default,
            capacity: float | type[Absent | Default] = Default,
            mass: float | type[Absent | Default] = Default,
            radius: float | type[Absent | Default] = Default,
            volume: float | type[Absent | Default] = Default,
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
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            cat_id: int | type[Absent | Default] = Default,
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
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
    ) -> int:
        return self.mk_eve_item_group(datas=datas, id_=id_, cat_id=EveItemCat.ship)

    def mk_eve_struct_group(
            self, *,
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
    ) -> int:
        return self.mk_eve_item_group(datas=datas, id_=id_, cat_id=EveItemCat.structure)

    def mk_eve_item_list(
            self, *,
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            inc_type_ids: list[int] | type[Absent] = Default,
            inc_grp_ids: list[int] | type[Absent] = Default,
            inc_cat_ids: list[int] | type[Absent] = Default,
            exc_type_ids: list[int] | type[Absent] = Default,
            exc_grp_ids: list[int] | type[Absent] = Default,
            exc_cat_ids: list[int] | type[Absent] = Default,
    ) -> int:
        if datas is Default:
            datas = [self._get_default_eve_data()]
        if id_ is Default:
            id_ = self.alloc_attr_id(datas=datas)
        for data in datas:
            data.mk_item_list(
                id_=id_,
                inc_type_ids=[] if inc_type_ids is Default else inc_type_ids,
                inc_grp_ids=[] if inc_grp_ids is Default else inc_grp_ids,
                inc_cat_ids=[] if inc_cat_ids is Default else inc_cat_ids,
                exc_type_ids=[] if exc_type_ids is Default else exc_type_ids,
                exc_grp_ids=[] if exc_grp_ids is Default else exc_grp_ids,
                exc_cat_ids=[] if exc_cat_ids is Default else exc_cat_ids)
        return id_

    def mk_eve_attr(
            self, *,
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            stackable: int | bool | type[Absent | Default] = Default,
            high_is_good: int | bool | type[Absent | Default] = Default,
            def_val: float | type[Absent | Default] = Default,
            min_attr_id: int | type[Absent | Default] = Default,
            max_attr_id: int | type[Absent | Default] = Default,
            unit_id: int | type[Absent | Default] = Default,
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
                max_attr_id=Absent if max_attr_id is Default else max_attr_id,
                unit_id=Absent if unit_id is Default else unit_id)
        return id_

    def mk_eve_effect(
            self, *,
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            cat_id: int | type[Absent | Default] = Default,
            is_assistance: int | bool | type[Absent | Default] = Default,
            is_offensive: int | bool | type[Absent | Default] = Default,
            discharge_attr_id: int | type[Absent | Default] = Default,
            duration_attr_id: int | type[Absent | Default] = Default,
            range_attr_id: int | type[Absent | Default] = Default,
            falloff_attr_id: int | type[Absent | Default] = Default,
            tracking_attr_id: int | type[Absent | Default] = Default,
            chance_attr_id: int | type[Absent | Default] = Default,
            resist_attr_id: int | type[Absent | Default] = Default,
            mod_info: list[EffectModifier] | type[Absent | Default] = Default,
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

    def mk_eve_online_effect(self, *, datas: list[EveObjects] | type[Default] = Default) -> int:
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
            func: str | type[Absent] = Absent,
            loc: str | type[Absent] = Absent,
            grp: int | type[Absent] = Absent,
            srq: int | type[Absent] = Absent,
            op: int | type[Absent] = Absent,
            affector_attr_id: int | type[Absent] = Absent,
            affectee_attr_id: int | type[Absent] = Absent,
    ) -> EffectModifier:
        return EffectModifier(
            func=func,
            location=loc,
            group=grp,
            skill_req=srq,
            operation=op,
            affector_attr_id=affector_attr_id,
            affectee_attr_id=affectee_attr_id)

    def mk_eve_buff(
            self, *,
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            aggr_mode: str | type[Absent | Default] = Default,
            op: str | type[Absent | Default] = Default,
            item_mods: list[BuffModifier] | type[Absent | Default] = Default,
            loc_mods: list[BuffModifier] | type[Absent | Default] = Default,
            loc_grp_mods: list[BuffModifier] | type[Absent | Default] = Default,
            loc_srq_mods: list[BuffModifier] | type[Absent | Default] = Default,
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
            attr_id: int | type[Absent] = Absent,
            group_id: int | type[Absent] = Absent,
            skill_id: int | type[Absent] = Absent,
    ) -> BuffModifier:
        return BuffModifier(
            attr_id=attr_id,
            group_id=group_id,
            skill_id=skill_id)

    def mk_eve_mutator(
            self, *,
            datas: list[EveObjects] | type[Default] = Default,
            id_: int | type[Default] = Default,
            items: list[tuple[list[int], int]] | type[Absent | Default] = Default,
            attrs: dict[int, tuple[float, float]] | type[Absent | Default] = Default,
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
                attrs={} if attrs is Default else attrs)
        return id_
