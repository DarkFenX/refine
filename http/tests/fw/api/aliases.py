type DpsProfile = tuple[float, float, float, float] | tuple[float, float, float, float, tuple[float, float] | None]
type MutaAdd = int | tuple[int, dict[int | str, float | str]]
type MutaChange = dict[int | str, float | str | None]
