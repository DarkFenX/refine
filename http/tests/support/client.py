from __future__ import annotations

from typing import TYPE_CHECKING

import requests

from tests.support.consts import ApiRack, ApiState, EveEffCat, EveEffect, EveItemCat
from tests.support.request import Request
from tests.support.util import Absent, Default, conditional_insert, get_stack_key
from tests.support import api_data, eve_data

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.consts import ApiEffMode
    from tests.support.eve_data import BuffModifier, EffectModifier
    from tests.support.util import StackKey

data_id: int = 10000000  # pylint: disable=C0103


class TestClient:

    def __init__(self, data_server, port: int):
        self.__datas: dict[str, eve_data.TestObjects] = {}
        self.__data_server = data_server
        self.__defsrc_stack_alias_map: dict[StackKey, str] = {}
        self.__session: requests.Session = requests.Session()
        self.__base_url: str = f'http://localhost:{port}'
        self.__created_data_aliases: set[str] = set()
        self.created_sss: set[api_data.SolarSystem] = set()

    def send_prepared(self, req: Request) -> requests.models.Response:
        return self.__session.send(req)

    # Data-related methods
    def mk_eve_data(self) -> eve_data.TestObjects:
        global data_id  # pylint: disable=C0103,W0603
        alias = str(data_id)
        data = self.__datas[alias] = eve_data.TestObjects(alias)
        data_id += 1
        return data

    @property
    def __default_data(self) -> eve_data.TestObjects:
        key = get_stack_key()
        if key in self.__defsrc_stack_alias_map:
            alias = self.__defsrc_stack_alias_map[key]
            return self.__datas[alias]
        data = self.mk_eve_data()
        self.__defsrc_stack_alias_map[key] = data.alias
        return data

    def mk_eve_item(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
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
    ) -> eve_data.Item:
        if data is Default:
            data = self.__default_data
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
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Default]] = EveItemCat.module,
    ) -> eve_data.Group:
        if data is Default:
            data = self.__default_data
        return data.mk_item_group(
            id_=id_,
            category_id=cat_id)

    def mk_eve_attr(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            stackable: bool = True,
            high_is_good: bool = True,
            def_val: float = 0.0,
            max_attr_id: Union[int, Type[Absent]] = Absent,
    ) -> eve_data.Attribute:
        if data is Default:
            data = self.__default_data
        return data.mk_attr(
            id_=id_,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            max_attribute_id=max_attr_id)

    def mk_eve_effect(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
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
    ) -> eve_data.Effect:
        if data is Default:
            data = self.__default_data
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

    def mk_eve_online_effect(self, data: Union[eve_data.TestObjects, Type[Default]] = Default) -> eve_data.Effect:
        if data is Default:
            data = self.__default_data
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

    def mk_eve_effect_mod(
            self,
            func: Union[str, Type[Absent]] = Absent,
            dom: Union[str, Type[Absent]] = Absent,
            grp: Union[int, Type[Absent]] = Absent,
            srq: Union[int, Type[Absent]] = Absent,
            op: Union[int, Type[Absent]] = Absent,
            src_attr_id: Union[int, Absent] = Absent,
            tgt_attr_id: Union[int, Absent] = Absent,
    ) -> eve_data.EffectModifier:
        return eve_data.EffectModifier(
            func=func,
            domain=dom,
            group=grp,
            skill_req=srq,
            operation=op,
            src_attr_id=src_attr_id,
            tgt_attr_id=tgt_attr_id)

    def mk_eve_buff(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
            id_: Union[int, Type[Default]] = Default,
            aggr_mode: Union[str, Type[Absent]] = Absent,
            op: Union[str, Type[Absent]] = Absent,
            item_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
            loc_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
            loc_grp_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
            loc_srq_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]] = Absent,
    ) -> eve_data.Buff:
        if data is Default:
            data = self.__default_data
        return data.mk_buff(
            id_=id_,
            aggregate_mode=aggr_mode,
            operation_name=op,
            item_modifiers=item_mods,
            location_modifiers=loc_mods,
            location_group_modifiers=loc_grp_mods,
            location_skillreq_modifiers=loc_srq_mods)

    def mk_eve_buff_mod(
            self,
            attr_id: Union[int, Type[Absent]] = Absent,
            group_id: Union[int, Type[Absent]] = Absent,
            skill_id: Union[int, Type[Absent]] = Absent,
    ) -> eve_data.BuffModifier:
        return eve_data.BuffModifier(
            attr_id=attr_id,
            group_id=group_id,
            skill_id=skill_id)

    # Data source-related methods
    def create_source_request(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
    ) -> Request:
        if data is Default:
            data = self.__default_data
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/source/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'http://localhost:{self.__data_server.port}/{data.alias}/'})

    def create_source(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
    ) -> None:
        if data is Default:
            data = self.__default_data
        # Set up server with local data
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
        # Get request and send it
        resp = self.create_source_request(data=data).send()
        assert resp.status_code == 201
        self.__created_data_aliases.add(data.alias)

    def remove_source_request(self, src_alias: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/source/{src_alias}')

    def remove_source(self, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def __setup_handler(self, url: str, data: str) -> None:
        self.__data_server.expect_request(url).respond_with_data(data)

    def create_sources(self) -> None:
        for data in self.__datas.values():
            self.create_source(data)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)

    # Solar system-related methods
    def create_ss_request(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
    ) -> Request:
        if data is Default:
            data = self.__default_data
        body = {}
        if data is not Absent:
            body['src_alias'] = data.alias
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/solar_system',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=body)

    def create_ss(
            self,
            data: Union[eve_data.TestObjects, Type[Default]] = Default,
    ) -> api_data.SolarSystem:
        if data is Default:
            data = self.__default_data
        resp = self.create_ss_request(data=data).send()
        assert resp.status_code == 201
        sol_sys = api_data.SolarSystem(client=self, data=resp.json())
        self.created_sss.add(sol_sys)
        return sol_sys

    def update_ss_request(self, ss_id: str) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'})

    def remove_ss_request(self, ss_id: str) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}')

    def cleanup_sss(self) -> None:
        for ss in self.created_sss.copy():
            ss.remove()

    # Fit-related methods
    def create_fit_request(
            self,
            ss_id: str
    ) -> Request:
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit',
            params={'fit': 'full', 'item': 'id'})

    def update_fit_request(
            self,
            ss_id: str,
            fit_id: str
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit/{fit_id}',
            params={'fit': 'full', 'item': 'id'})

    def remove_fit_request(
            self,
            ss_id: str,
            fit_id: str
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit/{fit_id}')

    # Item-related methods
    def get_item_request(
            self,
            ss_id: str,
            item_id: str
    ) -> Request:
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}/item/{item_id}',
            params={'item': 'full'})

    def remove_item_request(
            self,
            ss_id: str,
            item_id: str
    ) -> Request:
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}/item/{item_id}')

    def set_char_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int
    ) -> Request:
        payload = {'commands': [{'type': 'set_character', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=payload)

    def add_skill_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            level: int,
            state: Union[bool, Type[Absent]] = Absent
    ) -> Request:
        command = {
            'type': 'add_skill',
            'fit_id': fit_id,
            'type_id': type_id,
            'level': level}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def change_skill_request(
            self,
            ss_id: str,
            item_id: str,
            level: Union[int, Type[Absent]] = Absent,
            state: Union[bool, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'change_skill', 'item_id': item_id}
        conditional_insert(command, 'level', level)
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'effect_modes', effect_modes)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def add_implant_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_implant', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def set_ship_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
    ) -> Request:
        payload = {'commands': [{'type': 'set_ship', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=payload)

    def set_struct_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
    ) -> Request:
        payload = {'commands': [{'type': 'set_structure', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json=payload)

    def add_subsystem_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_subsystem', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def add_mod_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            rack: ApiRack = ApiRack.high,
            state: str = ApiState.offline,
            charge_type_id: Union[int, Type[Absent]] = Absent,
            mode: str = 'equip',
    ) -> Request:
        command = {
            'type': 'add_module',
            'fit_id': fit_id,
            'rack': rack,
            'add_mode': mode,
            'type_id': type_id,
            'state': state}
        conditional_insert(command, 'charge_type_id', charge_type_id)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def change_mod_request(
            self,
            ss_id: str,
            item_id: str,
            state: Union[ApiState, Type[Absent]] = Absent,
            charge: Union[int, None, Type[Absent]] = Absent,
            effect_modes: Union[dict[int, ApiEffMode], Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'change_module', 'item_id': item_id}
        conditional_insert(command, 'state', state)
        conditional_insert(command, 'charge', charge)
        conditional_insert(command, 'effect_modes', effect_modes)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def add_rig_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_rig', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def add_drone_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: str = ApiState.offline,
    ) -> Request:
        return self.__add_simple_item('add_drone', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def add_sw_effect_request(
            self,
            ss_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        command = {'type': 'add_sw_effect', 'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def change_sw_effect_request(
            self,
            ss_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__change_simple_item('change_sw_effect', ss_id=ss_id, item_id=item_id, state=state)

    def add_fw_effect_request(
            self,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__add_simple_item('add_fw_effect', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def change_fw_effect_request(
            self,
            ss_id: str,
            item_id: int,
            state: Union[bool, Type[Absent]] = Absent,
    ) -> Request:
        return self.__change_simple_item('change_fw_effect', ss_id=ss_id, item_id=item_id, state=state)

    def __add_simple_item(
            self,
            cmd_name: str,
            ss_id: str,
            fit_id: str,
            type_id: int,
            state: Union[bool, str, Type[Absent]],
    ) -> Request:
        command = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})

    def __change_simple_item(
            self,
            cmd_name: str,
            ss_id: str,
            item_id: int,
            state: Union[bool, str, Type[Absent]],
    ) -> Request:
        command = {'type': cmd_name, 'item_id': item_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'id'},
            json={'commands': [command]})
