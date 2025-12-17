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
    def dogma_to_api(*, dogma_effect_id: int) -> str:
        return f'd{dogma_effect_id}'

    @staticmethod
    def custom_to_api(*, custom_effect_id: int) -> str:
        return f'c{custom_effect_id}'

    @staticmethod
    def scsw_to_api(*, type_id: int) -> str:
        return f'scsw{type_id}'

    @staticmethod
    def scse_to_api(*, type_id: int) -> str:
        return f'scse{type_id}'

    @staticmethod
    def scpe_to_api(*, type_id: int) -> str:
        return f'scpe{type_id}'

    @staticmethod
    def scpt_to_api(*, type_id: int) -> str:
        return f'scpt{type_id}'

    @staticmethod
    def scsl_to_api(*, type_id: int) -> str:
        return f'scsl{type_id}'
