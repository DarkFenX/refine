from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.consts import EveEffCat, EveEffect, EveItemCat
from tests.support.util import Absent, Default, get_stack_key
from .containers import EveObjects
from .types import BuffModifier, EffectModifier

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.util import StackKey
    from .types import Attribute, Buff, Effect, Group, Item

data_id: int = 10000000  # pylint: disable=C0103


class EveDataClient:

    def __init__(self, data_server, **kwargs):
        super().__init__(**kwargs)
        self.__data_server = data_server
        self.__datas: dict[str, EveObjects] = {}
        self.__defsrc_stack_alias_map: dict[StackKey, str] = {}

    # Data container-related methods
    def mk_eve_data(self) -> EveObjects:
        global data_id  # pylint: disable=C0103,W0603
        alias = str(data_id)
        data = self.__datas[alias] = EveObjects(alias)
        data_id += 1
        return data

    def _get_eve_data(self, data: Union[EveObjects, Type[Default]] = Default) -> EveObjects:
        if data is Default:
            data = self.__default_eve_data
        return data

    @property
    def __default_eve_data(self) -> EveObjects:
        key = get_stack_key()
        if key in self.__defsrc_stack_alias_map:
            alias = self.__defsrc_stack_alias_map[key]
            return self.__datas[alias]
        data = self.mk_eve_data()
        self.__defsrc_stack_alias_map[key] = data.alias
        return data

    @property
    def _eve_datas(self) -> dict[str, EveObjects]:
        return self.__datas

    # EVE entity creation methods
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
            src_attr_id: Union[int, Absent] = Absent,
            tgt_attr_id: Union[int, Absent] = Absent,
    ) -> EffectModifier:
        return EffectModifier(
            func=func,
            domain=dom,
            group=grp,
            skill_req=srq,
            operation=op,
            src_attr_id=src_attr_id,
            tgt_attr_id=tgt_attr_id)

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

    # Server setup-related methods
    def _setup_eve_data_server(self, data: EveObjects) -> None:
        str_data = data.render()
        suffix_cont_map = {
            'fsd_binary/types.json': str_data.types,
            'fsd_binary/groups.json': str_data.groups,
            'fsd_binary/dogmaattributes.json': str_data.dogmaattributes,
            'fsd_binary/typedogma.json': str_data.typedogma,
            'fsd_binary/dogmaeffects.json': str_data.dogmaeffects,
            'fsd_lite/fighterabilities.json': str_data.fighterabilities,
            'fsd_lite/fighterabilitiesbytype.json': str_data.fighterabilitiesbytype,
            'fsd_lite/dbuffcollections.json': str_data.dbuffcollections,
            'fsd_binary/requiredskillsfortypes.json': str_data.requiredskillsfortypes,
            'fsd_binary/dynamicitemattributes.json': str_data.dynamicitemattributes}
        for suffix, container in suffix_cont_map.items():
            self.__setup_handler(f'/{data.alias}/{suffix}', container)

    def __setup_handler(self, url: str, data: str) -> None:
        self.__data_server.expect_request(url).respond_with_data(data)

    @property
    def _eve_data_server_base_url(self) -> str:
        return f'http://localhost:{self.__data_server.port}'
