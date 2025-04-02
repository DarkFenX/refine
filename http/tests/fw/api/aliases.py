type DpsProfile = tuple[float, float, float, float] | tuple[float, float, float, float, tuple[float, float] | None]
type MutaAdd = int | tuple[int, dict[int, dict[str, float]]]
type MutaChange = dict[int, dict[str, float] | None]
