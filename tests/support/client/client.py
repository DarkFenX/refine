from collections import defaultdict

from .data import TestObjects
from ..consts import EffectCategory
from ..util import Absent, Default


class TestClient:

    def __init__(self, data_server):
        self.__data = defaultdict(lambda: TestObjects())
        self.__data_server = data_server

    @property
    def data(self):
        return self.__data

    @property
    def __default_data(self):
        return self.data['tq']

    def add_item(
            self,
            data=Default,
            id=Default,
            grp_id=Default,
            cat_id=Default,
            attrs=Default,
            eff_ids=Default,
            defeff_id=None,
            srqs=Default,
    ):
        if data is Default:
            data = self.__default_data
        return data.add_item(
            id=id,
            group_id=grp_id,
            category_id=cat_id,
            attributes={} if attrs is Default else attrs,
            effect_ids=[] if eff_ids is Default else eff_ids,
            default_effect_id=defeff_id,
            skill_reqs={} if srqs is Default else srqs)

    def add_attr(
            self,
            data=Default,
            id=Default,
            stackable=1,
            high_is_good=1,
            def_val=0.0,
            max_attr_id=Absent,
    ):
        if data is Default:
            data = self.__default_data
        return data.add_attr(
            id=id,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            max_attribute_id=max_attr_id)

    def add_effect(
            self,
            data=Default,
            id=Default,
            cat_id=EffectCategory.passive,
            is_assistance=False,
            is_offensive=False,
            discharge_attr_id=Absent,
            duration_attr_id=Absent,
            range_attr_id=Absent,
            falloff_attr_id=Absent,
            tracking_attr_id=Absent,
            chance_attr_id=Absent,
            resist_attr_id=Absent,
            mod_info=Absent,
    ):
        if data is Default:
            data = self.__default_data
        return data.add_effect(
            id=id,
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

    def add_buff(
            self,
            data=Default,
            id=Default,
            aggr_mode=Default,
            op=Default,
            item_mods=Default,
            loc_mods=Default,
            loc_grp_mods=Default,
            loc_srq_mods=Default,
    ):
        if data is Default:
            data = self.__default_data
        return data.add_buff(
            id=id,
            aggregate_mode=aggr_mode,
            operation_name=op,
            item_modifiers=item_mods,
            location_modifiers=loc_mods,
            location_group_modifiers=loc_grp_mods,
            location_skillreq_modifiers=loc_srq_mods)
