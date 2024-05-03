from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import EveEffCat, EveEffect, EveItemCat
from tests.support.util import Absent, Default
from .data_manager import EveDataManager
from .types import BuffModifier, EffectModifier

if TYPE_CHECKING:
    from typing import Type, Union

    from .containers import EveObjects
    from .types import Attribute, Buff, Effect, Group, Item


class EveTypeFactory(EveDataManager):

    def mk_eve_item(
            self,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            grp_id: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Default]] = EveItemCat.module,
            attrs: Union[dict[int, float], Type[Default]] = Default,
            eff_ids: Union[list[int], Type[Default]] = Default,
            defeff_id: Union[int, None] = None,
            srqs: Union[dict[int, int], Type[Default]] = Default,
            capacity: Union[float, Type[Default]] = Default,
            mass: Union[float, Type[Default]] = Default,
            radius: Union[float, Type[Default]] = Default,
            volume: Union[float, Type[Default]] = Default,
    ) -> Item:
        data = self._get_eve_data(data=data)
        return data.mk_item(
            id_=id_,
            group_id=grp_id,
            category_id=cat_id,
            attributes={} if attrs is Default else attrs,
            effect_ids=[] if eff_ids is Default else eff_ids,
            default_effect_id=defeff_id,
            skill_reqs={} if srqs is Default else srqs,
            capacity=0.0 if capacity is Default else capacity,
            mass=0.0 if mass is Default else mass,
            radius=0.0 if radius is Default else radius,
            volume=0.0 if volume is Default else volume)

    def mk_eve_item_group(
            self,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Default]] = EveItemCat.module,
    ) -> Group:
        data = self._get_eve_data(data=data)
        return data.mk_item_group(
            id_=id_,
            category_id=cat_id)

    def mk_eve_attr(
            self,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            stackable: bool = True,
            high_is_good: bool = True,
            def_val: float = 0.0,
            max_attr_id: Union[int, Type[Absent]] = Absent,
    ) -> Attribute:
        data = self._get_eve_data(data=data)
        return data.mk_attr(
            id_=id_,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            max_attribute_id=max_attr_id)

    def mk_eve_effect(
            self,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            cat_id: int = EveEffCat.passive,
            is_assistance: bool = False,
            is_offensive: bool = False,
            discharge_attr_id: Union[int, Type[Absent]] = Absent,
            duration_attr_id: Union[int, Type[Absent]] = Absent,
            range_attr_id: Union[int, Type[Absent]] = Absent,
            falloff_attr_id: Union[int, Type[Absent]] = Absent,
            tracking_attr_id: Union[int, Type[Absent]] = Absent,
            chance_attr_id: Union[int, Type[Absent]] = Absent,
            resist_attr_id: Union[int, Type[Absent]] = Absent,
            mod_info: Union[list[EffectModifier], tuple[EffectModifier], Type[Absent]] = Absent,
    ) -> Effect:
        data = self._get_eve_data(data=data)
        return data.mk_effect(
            id_=id_,
            category_id=cat_id,
            is_assistance=is_assistance,
            is_offensive=is_offensive,
            discharge_attribute_id=discharge_attr_id,
            duration_attribute_id=duration_attr_id,
            range_attribute_id=range_attr_id,
            falloff_attribute_id=falloff_attr_id,
            tracking_attribute_id=tracking_attr_id,
            usage_chance_attribute_id=chance_attr_id,
            resist_attribute_id=resist_attr_id,
            modifier_info=mod_info)

    def mk_eve_online_effect(self, data: Union[EveObjects, Type[Default]] = Default) -> Effect:
        data = self._get_eve_data(data=data)
        return data.mk_effect(
            id_=EveEffect.online,
            category_id=EveEffCat.active,
            is_assistance=False,
            is_offensive=False,
            discharge_attribute_id=Absent,
            duration_attribute_id=Absent,
            range_attribute_id=Absent,
            falloff_attribute_id=Absent,
            tracking_attribute_id=Absent,
            usage_chance_attribute_id=Absent,
            resist_attribute_id=Absent,
            modifier_info=Absent)

    @staticmethod
    def mk_eve_effect_mod(
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
            self,
            data: Union[EveObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            aggr_mode: Union[str, Type[Absent]] = Absent,
            op: Union[str, Type[Absent]] = Absent,
            item_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
            loc_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
            loc_grp_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
            loc_srq_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
    ) -> Buff:
        data = self._get_eve_data(data=data)
        return data.mk_buff(
            id_=id_,
            aggregate_mode=aggr_mode,
            operation_name=op,
            item_modifiers=item_mods,
            location_modifiers=loc_mods,
            location_group_modifiers=loc_grp_mods,
            location_skillreq_modifiers=loc_srq_mods)

    @staticmethod
    def mk_eve_buff_mod(
            attr_id: Union[int, Type[Absent]] = Absent,
            group_id: Union[int, Type[Absent]] = Absent,
            skill_id: Union[int, Type[Absent]] = Absent,
    ) -> BuffModifier:
        return BuffModifier(
            attr_id=attr_id,
            group_id=group_id,
            skill_id=skill_id)
