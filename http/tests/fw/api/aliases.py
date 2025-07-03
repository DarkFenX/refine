type DpsProfile = tuple[float, float, float, float] | tuple[float, float, float, float, tuple[float, float] | None]
type ProjRange = float | str | None
type MutaAdd = int | tuple[int, dict[int, float | str]]
type MutaChange = dict[int, float | str | None]
