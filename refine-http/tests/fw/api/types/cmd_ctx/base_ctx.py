import dataclasses
import enum
import typing

from fw.api.types.item import Item

if typing.TYPE_CHECKING:
    from fw.api import ApiClient
    from fw.api.aliases import ReqHook
    from fw.api.commands import BaseCommand
    from fw.request import Request
    from fw.response import Response


@enum.unique
class IdFillKind(enum.StrEnum):
    regular = 'regular'
    charge = 'charge'


@dataclasses.dataclass(kw_only=True)
class EntityData:
    kind: IdFillKind
    data: dict


class BaseCmdCtx:

    def __init__(
            self, *,
            client: ApiClient,
            sol_id: str,
            hook_req: ReqHook | None,
            status_code: int,
            json_predicate: dict | None,
    ) -> None:
        self._client = client
        self._sol_id = sol_id
        self._hook_req = hook_req
        self._status_code = status_code
        self._json_predicate = json_predicate
        self._commands: list[BaseCommand] = []
        self._ret_datas: dict[int, EntityData] = {}

    # Entity making methods are supposed to be called after command has been added
    def _make_item(self) -> Item:
        index = len(self._commands) - 1
        data = {'id': f'#{index}', 'charge': {'id': f'#{index}c'}}
        self._ret_datas[index] = EntityData(kind=IdFillKind.regular, data=data)
        return Item(client=self._client, data=data, sol_id=self._sol_id)

    def _make_item_charge(self) -> Item:
        index = len(self._commands) - 1
        data = {'id': f'#{index}c'}
        self._ret_datas[index] = EntityData(kind=IdFillKind.charge, data=data)
        return Item(client=self._client, data=data, sol_id=self._sol_id)

    def _clear_ret_datas(self) -> None:
        for entity_data in self._ret_datas.values():
            entity_data.data.clear()

    def _process_request(self, *, req: Request) -> Response:
        if self._hook_req is not None:
            self._hook_req(req)
        resp = req.send()
        self._client.check_sol(sol_id=self._sol_id)
        resp.check(status_code=self._status_code, json_predicate=self._json_predicate)
        return resp

    def _fill_entity_ids(self, *, resp_data: dict) -> None:
        # Update IDs in all the entities which were created by the commands
        for i, cmd_result in enumerate(resp_data['cmd_results']):
            if i not in self._ret_datas:
                continue
            entity_data = self._ret_datas[i]
            match entity_data.kind:
                case IdFillKind.regular:
                    self.__fill_entity_ids_regular(entity_data=entity_data, cmd_result=cmd_result)
                case IdFillKind.charge:
                    self.__fill_entity_ids_charge(entity_data=entity_data, cmd_result=cmd_result)

    @staticmethod
    def __fill_entity_ids_regular(*, entity_data: EntityData, cmd_result: dict) -> None:
        if 'fleet_id' in cmd_result:
            entity_data.data['id'] = cmd_result['fleet_id']
        if 'fit_id' in cmd_result:
            entity_data.data['id'] = cmd_result['fit_id']
        if 'item_id' in cmd_result:
            entity_data.data['id'] = cmd_result['item_id']
        if 'charge_item_id' in cmd_result:
            entity_data.data['charge'] = {'id': cmd_result['charge_item_id']}

    @staticmethod
    def __fill_entity_ids_charge(*, entity_data: EntityData, cmd_result: dict) -> None:
        if 'charge_item_id' in cmd_result:
            entity_data.data['id'] = cmd_result['charge_item_id']
