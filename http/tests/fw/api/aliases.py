type DpsProfile = tuple[float, float, float, float] | tuple[float, float, float, float, tuple[float, float] | None]
type MutaAdd = int | tuple[int, dict[int, int | float | str]]
type MutaChange = dict[int, int | float | str | None]
