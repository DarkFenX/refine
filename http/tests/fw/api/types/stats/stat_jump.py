from fw.util import AttrDict, AttrHookDef, NttList


class StatJump(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'jump_bridge': AttrHookDef(func=lambda d: NttList(d) if d is not None else None)})
