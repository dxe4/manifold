from typing import List, Any

class Collatz:
    n: int
    seq: List[int]
    two_adic_distance: List[float]

    def get_seq_str(self) -> List[str]: ...
    def two_adic_disntace_str(self) -> List[str]: ...
    def seq_str(self) -> List[str]: ...
    def total_distance(self) -> str: ...
    def total_2adic_distance(self) -> str: ...


def to_rug_integer(obj: Any) -> int: ...
def miller_rabin_bool_multiple(a: Any, b: Any) -> List[bool]: ...
def miller_rabin_bool(a: Any) -> bool: ...
def collatz_sequence(a: Any) -> Collatz: ...
def power_of_two_exponent_10n_py(start: int, end: int) -> List[str]: ...
