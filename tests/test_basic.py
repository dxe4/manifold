import pytest
from manifold_rs import (
    miller_rabin_bool,
    miller_rabin_bool_multiple,
    power_of_two_exponent_10n_py
)


def test_miller_rabin_bool():
    assert miller_rabin_bool(5) is True
    assert miller_rabin_bool(6) is False


def test_miller_rabin_bool_multiple():
    x = miller_rabin_bool_multiple(5, 6)
    assert x == [True, False]

@pytest.mark.xfail
def test_miller_rabin_bool_multiple_2():
    # TODO fix this, this probably proge when multi threading was added
    # add tests in the rust version
    x = miller_rabin_bool_multiple(10, 13)
    assert x == [False, True, False, True]

@pytest.mark.xfail
def test_miller_rabin_bool_2():
    # TODO fix this, this probably proge when multi threading was added
    # add tests in the rust version    
    assert miller_rabin_bool(10) is False
    assert miller_rabin_bool(11) is True
    assert miller_rabin_bool(12) is False
    assert miller_rabin_bool(13) is True


def test_power_of_two_exponent_10n_py():
    res = power_of_two_exponent_10n_py(1, 100)
    assert res[-2] == '4573536648073116712502659212705508520418524335224287996399854941049366806437310995625742046215396793754803629590173942305493553167642242743069098817056046992680891830197061490109937833490419136188999442576576769103890995893380022607743740081787109376'
    assert res[-1] == '7228380315618644500754615641066207634474587451600209197559409389648307032691781901149685493801300001392571910673782855101147912247921594402023866545696046992680891830197061490109937833490419136188999442576576769103890995893380022607743740081787109376'
