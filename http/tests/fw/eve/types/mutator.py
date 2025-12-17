import typing
from dataclasses import dataclass

from fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from fw.eve.containers.primitives import EvePrimitives


def convert_items(
        items: list[tuple[list[int] | type[Absent], int | type[Absent]]],
) -> list[dict[str, list[int] | int]]:
    converted = []
    for in_items, out_item in items:
        entry_data = {}
        conditional_insert(container=entry_data, path=['applicableTypes'], value=in_items)
        conditional_insert(container=entry_data, path=['resultingType'], value=out_item)
        converted.append(entry_data)
    return converted


def convert_attributes(
        attributes: dict[int, tuple[float | type[Absent], float | type[Absent]]],
) -> dict[int: dict[str, float]]:
    converted = {}
    for attr_id, (min_mult, max_mult) in attributes.items():
        attr_data = {}
        conditional_insert(container=attr_data, path=['min'], value=min_mult)
        conditional_insert(container=attr_data, path=['max'], value=max_mult)
        converted[attr_id] = attr_data
    return converted


@dataclass(kw_only=True)
class Mutator:

    id: int
    items: list[tuple[list[int], int]] | type[Absent]
    attributes: dict[int, tuple[float, float]] | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        mutator_entry = {}
        conditional_insert(
            container=mutator_entry,
            path=['inputOutputMapping'],
            value=self.items,
            cast_to=convert_items)
        conditional_insert(
            container=mutator_entry,
            path=['attributeIDs'],
            value=self.attributes,
            cast_to=convert_attributes)
        primitive_data.dynamicitemattributes[self.id] = mutator_entry
