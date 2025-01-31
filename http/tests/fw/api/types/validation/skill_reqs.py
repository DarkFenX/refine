from dataclasses import dataclass


class ValSrqDetails(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({
            item_id: {int(skill_id): ValSrqSkill(data=skill_data) for skill_id, skill_data in item_data.items()}
            for item_id, item_data in data.items()})


@dataclass
class ValSrqSkill:

    skill_lvl: int | None
    req_lvl: int

    def __init__(self, *, data: tuple) -> None:
        self.skill_lvl = data[0]
        self.req_lvl = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.skill_lvl, self.req_lvl) == (other[0], other[1])
