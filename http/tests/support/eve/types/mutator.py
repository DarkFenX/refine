from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import Absent, conditional_insert, make_repr_str

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.eve.containers.primitives import EvePrimitives


def convert_items(
        items: list[tuple[Union[list[int], Type[Absent]], Union[int, Type[Absent]]]]
) -> list[dict[str, Union[list[int], int]]]:
    converted = []
    for in_items, out_item in items:
        entry_data = {}
        conditional_insert(container=entry_data, key='applicableTypes', value=in_items)
        conditional_insert(container=entry_data, key='resultingType', value=out_item)
        converted.append(entry_data)
    return converted


def convert_attributes(
        attributes: dict[int, tuple[Union[float, Type[Absent]], Union[float, Type[Absent]]]]
) -> dict[int: dict[str, float]]:
    converted = {}
    for attr_id, (min_mult, max_mult) in attributes.items():
        attr_data = {}
        conditional_insert(container=attr_data, key='min', value=min_mult)
        conditional_insert(container=attr_data, key='max', value=max_mult)
        converted[attr_id] = attr_data
    return converted



class Mutator:

    def __init__(
            self, *,
            id_: int,
            items: Union[list[tuple[list[int], int]], Type[Absent]],
            attributes: Union[dict[int, tuple[float, float]], Type[Absent]],
    ):
        self.id = id_
        self.items = items
        self.attributes = attributes

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        mutator_entry = {}
        conditional_insert(
            container=mutator_entry,
            key='inputOutputMapping',
            value=self.items,
            cast_to=convert_items)
        conditional_insert(
            container=mutator_entry,
            key='attributeIDs',
            value=self.attributes,
            cast_to=convert_attributes)
        primitive_data.dynamicitemattributes[self.id] = mutator_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
