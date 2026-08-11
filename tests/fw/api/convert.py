class Muta:

    @staticmethod
    def roll_to_api(*, val: float) -> str:
        return f'r{val}'

    @staticmethod
    def abs_to_api(*, val: float) -> str:
        return f'a{val}'


class Spool:

    @staticmethod
    def cycles_to_api(*, count: int) -> str:
        return f'c{count}'

    @staticmethod
    def time_to_api(*, time: float) -> str:
        return f't{time}'

    @staticmethod
    def spool_scale_to_api(*, val: float) -> str:
        return f'ss{val}'

    @staticmethod
    def cycle_scale_to_api(*, val: float) -> str:
        return f'cs{val}'


class Effect:

    @staticmethod
    def custom_to_api(*, custom_effect_id: int) -> str:
        return f'c{custom_effect_id}'

    @staticmethod
    def sw_to_api(*, type_id: int) -> str:
        return f'sw{type_id}'

    @staticmethod
    def se_to_api(*, type_id: int) -> str:
        return f'se{type_id}'

    @staticmethod
    def pe_to_api(*, type_id: int) -> str:
        return f'pe{type_id}'

    @staticmethod
    def pt_to_api(*, type_id: int) -> str:
        return f'pt{type_id}'

    @staticmethod
    def sl_to_api(*, type_id: int) -> str:
        return f'sl{type_id}'
