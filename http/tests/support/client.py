import requests

from tests.support.api_data import SolarSystem
from tests.support.consts import EffCat, ItemCat
from tests.support.eve_data import TestObjects, Modifier
from tests.support.request import Request
from tests.support.util import Absent, Default, conditional_insert, get_stack_key

data_id = 10000000  # pylint: disable=C0103


class TestClient:

    def __init__(self, data_server, port):
        self.__datas = {}
        self.__data_server = data_server
        self.__stack_alias_map = {}
        self.__session = requests.Session()
        self.__base_url = f'http://localhost:{port}'

    def send_prepared(self, req):
        return self.__session.send(req)

    # Data-related methods
    def mk_eve_data(self):
        global data_id  # pylint: disable=C0103,W0603
        alias = str(data_id)
        data = self.__datas[alias] = TestObjects(alias)
        data_id += 1
        return data

    @property
    def __default_data(self):
        key = get_stack_key()
        if key in self.__stack_alias_map:
            alias = self.__stack_alias_map[key]
            return self.__datas[alias]
        data = self.mk_eve_data()
        self.__stack_alias_map[key] = data.alias
        return data

    def mk_eve_item(
            self,
            data=Default,
            id_=Default,
            grp_id=Default,
            cat_id=ItemCat.module,
            attrs=Default,
            eff_ids=Default,
            defeff_id=None,
            srqs=Default,
            capacity=Default,
            mass=Default,
            radius=Default,
            volume=Default,
    ):
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

    def mk_eve_attr(
            self,
            data=Default,
            id_=Default,
            stackable=True,
            high_is_good=True,
            def_val=0.0,
            max_attr_id=Absent,
    ):
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
            data=Default,
            id_=Default,
            cat_id=EffCat.passive,
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

    def mk_eve_buff(
            self,
            data=Default,
            id_=Default,
            aggr_mode=Default,
            op=Default,
            item_mods=Default,
            loc_mods=Default,
            loc_grp_mods=Default,
            loc_srq_mods=Default,
    ):
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

    def mk_eve_mod(
            self,
            func=Absent,
            dom=Absent,
            src_attr_id=Absent,
            tgt_attr_id=Absent,
            op=Absent
    ):
        return Modifier(
            func=func,
            domain=dom,
            src_attr_id=src_attr_id,
            tgt_attr_id=tgt_attr_id,
            operation=op)

    # Data source-related methods
    def create_source_request(self, data=Default):
        if data is Default:
            data = self.__default_data
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/source/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'http://localhost:{self.__data_server.port}/{data.alias}/'})

    def create_source(self, data=Default):
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
        assert resp.status_code == 204

    def __setup_handler(self, url, data):
        self.__data_server.expect_request(url).respond_with_data(data)

    def create_sources(self):
        for data in self.__datas.values():
            self.create_source(data)

    # Solar system-related methods
    def create_ss_request(self, data=Default):
        if data is Default:
            data = self.__default_data
        body = {}
        if data is not Absent:
            body['src_alias'] = data.alias
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/solar_system',
            params={'ss': 'full', 'fit': 'full', 'item': 'full'},
            json=body)

    def create_ss(self, data=Default):
        if data is Default:
            data = self.__default_data
        resp = self.create_ss_request(data=data).send()
        assert resp.status_code == 201
        return SolarSystem(client=self, data=resp.json())

    def update_ss_request(self, ss_id):
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            params={'ss': 'full', 'fit': 'full', 'item': 'full'})

    # Fit-related methods
    def create_fit_request(self, ss_id):
        return Request(
            self,
            method='POST',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit',
            params={'fit': 'full', 'item': 'full'})

    def update_fit_request(self, ss_id, fit_id):
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}/fit/{fit_id}',
            params={'fit': 'full', 'item': 'full'})

    # Item-related methods
    def get_item_request(self, ss_id, item_id):
        return Request(
            self,
            method='GET',
            url=f'{self.__base_url}/solar_system/{ss_id}/item/{item_id}',
            params={'item': 'full'})

    def remove_item_request(self, ss_id, item_id):
        return Request(
            self,
            method='DELETE',
            url=f'{self.__base_url}/solar_system/{ss_id}/item/{item_id}')

    def add_implant_request(self, ss_id, fit_id, type_id, state=Absent):
        return self.__add_simple_item('add_implant', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def set_ship_request(self, ss_id, fit_id, type_id):
        payload = {'commands': [{'type': 'set_ship', 'fit_id': fit_id, 'type_id': type_id}]}
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            json=payload)

    def add_high_mod_request(self, ss_id, fit_id, module_type_id, state='offline', charge_type_id=Absent, mode='equip'):
        command = {
            'type': 'add_module_high',
            'fit_id': fit_id,
            'add_mode': mode,
            'module_type_id': module_type_id,
            'state': state}
        conditional_insert(command, 'charge_type_id', charge_type_id)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            json={'commands': [command]})

    def add_rig_request(self, ss_id, fit_id, type_id, state=Absent):
        return self.__add_simple_item('add_rig', ss_id=ss_id, fit_id=fit_id, type_id=type_id, state=state)

    def __add_simple_item(self, cmd_name, ss_id, fit_id, type_id, state):
        command = {
            'type': cmd_name,
            'fit_id': fit_id,
            'type_id': type_id}
        conditional_insert(command, 'state', state)
        return Request(
            self,
            method='PATCH',
            url=f'{self.__base_url}/solar_system/{ss_id}',
            json={'commands': [command]})
