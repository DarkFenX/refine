import inspect

import requests

from .data import TestObjects
from .data.objects import Modifier
from ..consts import EffCat, ItemCat
from ..util import Absent, Default

data_id = 10000000


def frame_to_primitive(frame, ignore_local_context=False):
    if ignore_local_context:
        return (
            frame.filename,
            frame.function)
    else:
        pos = frame.positions
        return (
            frame.filename,
            frame.lineno,
            frame.function,
            pos.lineno,
            pos.end_lineno,
            pos.col_offset,
            pos.end_col_offset)


def get_stack_key():
    stack = inspect.stack(context=0)
    # Filter out stack entries for entities from client file
    stack = [f for f in stack if f.filename != __file__]
    # For method which tried to retrieve data, ignore all its local context,
    # to refer to the same data on different calls
    key = [frame_to_primitive(stack[0], ignore_local_context=True)]
    key += [frame_to_primitive(f) for f in stack[1:]]
    return tuple(key)


class TestClient:

    def __init__(self, data_server):
        self.__datas = {}
        self.__data_server = data_server
        self.__stack_alias_map = {}
        self.__session = requests.Session()

    # Data-related methods
    def mk_data(self):
        global data_id
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
        data = self.mk_data()
        self.__stack_alias_map[key] = data.alias
        return data

    def mk_item(
            self,
            data=Default,
            id=Default,
            grp_id=Default,
            cat_id=ItemCat.module,
            attrs=Default,
            eff_ids=Default,
            defeff_id=None,
            srqs=Default,
    ):
        if data is Default:
            data = self.__default_data
        return data.mk_item(
            id=id,
            group_id=grp_id,
            category_id=cat_id,
            attributes={} if attrs is Default else attrs,
            effect_ids=[] if eff_ids is Default else eff_ids,
            default_effect_id=defeff_id,
            skill_reqs={} if srqs is Default else srqs)

    def mk_attr(
            self,
            data=Default,
            id=Default,
            stackable=True,
            high_is_good=True,
            def_val=0.0,
            max_attr_id=Absent,
    ):
        if data is Default:
            data = self.__default_data
        return data.mk_attr(
            id=id,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            max_attribute_id=max_attr_id)

    def mk_effect(
            self,
            data=Default,
            id=Default,
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

    def mk_buff(
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
        return data.mk_buff(
            id=id,
            aggregate_mode=aggr_mode,
            operation_name=op,
            item_modifiers=item_mods,
            location_modifiers=loc_mods,
            location_group_modifiers=loc_grp_mods,
            location_skillreq_modifiers=loc_srq_mods)

    def mk_mod(
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

    def create_source_request(self, data=Default):
        if data is Default:
            data = self.__default_data
        req = requests.Request('POST', f'http://localhost:8000/source/{data.alias}', json={
            'data_version': '1',
            'data_base_url': f'http://localhost:{self.__data_server.port}/{data.alias}/'})
        return req

    def create_source(self, data=Default):
        if data is Default:
            data = self.__default_data
        # Set up server with local data
        str_data = data.render()
        self.__setup_handler(f'/{data.alias}/fsd_binary/types.json', str_data.types)
        self.__setup_handler(f'/{data.alias}/fsd_binary/groups.json', str_data.groups)
        self.__setup_handler(f'/{data.alias}/fsd_binary/dogmaattributes.json', str_data.dogmaattributes)
        self.__setup_handler(f'/{data.alias}/fsd_binary/typedogma.json', str_data.typedogma)
        self.__setup_handler(f'/{data.alias}/fsd_binary/dogmaeffects.json', str_data.dogmaeffects)
        self.__setup_handler(f'/{data.alias}/fsd_lite/fighterabilities.json', str_data.fighterabilities)
        self.__setup_handler(f'/{data.alias}/fsd_lite/fighterabilitiesbytype.json', str_data.fighterabilitiesbytype)
        self.__setup_handler(f'/{data.alias}/fsd_lite/dbuffcollections.json', str_data.dbuffcollections)
        self.__setup_handler(f'/{data.alias}/fsd_binary/requiredskillsfortypes.json', str_data.requiredskillsfortypes)
        self.__setup_handler(f'/{data.alias}/fsd_binary/dynamicitemattributes.json', str_data.dynamicitemattributes)
        # Get request and send it
        req = self.create_source_request(data=data)
        resp = self.__session.send(req.prepare())
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
        payload = {}
        if data is not Absent:
            payload['src_alias'] = data.alias
        req = requests.Request('POST', 'http://localhost:8000/solar_system', json=payload)
        return req

    def create_ss(self, data=Default):
        if data is Default:
            data = self.__default_data
        req = self.create_ss_request(data=data)
        resp = self.__session.send(req.prepare())
        assert resp.status_code == 201
        return resp.json()['id']

    def create_fit_request(self, ss):
        payload = {}
        req = requests.Request('POST', f'http://localhost:8000/solar_system/{ss}/fit', json=payload)
        return req

    def create_fit(self, ss):
        req = self.create_fit_request(ss=ss)
        resp = self.__session.send(req.prepare())
        assert resp.status_code == 201
        return resp.json()['id']

    def set_ship_request(self, ss, fit_id, ship_id):
        payload = {'commands': [{'type': 'set_ship', 'ship_type_id': ship_id}]}
        req = requests.Request('PATCH', f'http://localhost:8000/solar_system/{ss}/fit/{fit_id}', json=payload)
        return req

    def set_ship(self, ss, fit_id, ship_id):
        req = self.set_ship_request(ss=ss, fit_id=fit_id, ship_id=ship_id)
        resp = self.__session.send(req.prepare())
        assert resp.status_code == 200
        return resp.json()['cmd_results'][0]['id']

    def add_high_mod_request(self, ss, fit_id, module_id, state, charge_id=None, mode='equip'):
        command = {'type': 'add_module_high', 'add_mode': mode, 'module_type_id': module_id, 'state': state}
        if charge_id is not None:
            command['charge_type_id'] = charge_id
        payload = {'commands': [command]}
        req = requests.Request('PATCH', f'http://localhost:8000/solar_system/{ss}/fit/{fit_id}', json=payload)
        return req

    def add_high_mod(self, ss, fit_id, module_id, state, charge_id=None, mode='equip'):
        req = self.add_high_mod_request(
            ss=ss, fit_id=fit_id, module_id=module_id,
            state=state, charge_id=charge_id, mode=mode)
        resp = self.__session.send(req.prepare())
        assert resp.status_code == 200
        return resp.json()['cmd_results'][0]['id']
