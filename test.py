from manifold_rs import miller_rabin_bool

import timeit
from sympy.ntheory.primetest import mr as miller_rabin


def get_base(n):
    if n < 2047:
        a = [2]
    if n < 1373653:
        a = [2, 3]
    if n < 9080191:
        a = [31, 73]
    if n < 25326001:
        a = [2, 3, 5]
    if n < 3215031751:
        a = [2, 3, 5, 7]
    if n < 4759123141:
        a = [2, 7, 61]
    if n < 1122004669633:
        a = [2, 13, 23, 1662803]
    if n < 2152302898747:
        a = [2, 3, 5, 7, 11]
    if n < 3474749660383:
        a = [2, 3, 5, 7, 11, 13]
    if n < 341550071728321:
        a = [2, 3, 5, 7, 11, 13, 17]
    if n < 3825123056546413051:
        a = [2, 3, 5, 7, 11, 13, 17, 19, 23]
    else:
        a = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    return a


# MIN, MAX = 10**3, 10**5
MIN, MAX = 341550071728321, 341550071728321 + 10**6


def benchmark_miller_rabin(num_trials=5):

    def miller_rabin_test():
        for i in range(MIN, MAX):
            bases = get_base(i)
            is_prime = miller_rabin(i, bases)

    def miller_rabin_test_rust():
        for i in range(MIN, MAX):
            is_prime = miller_rabin_bool(i)

    total_time_py = timeit.timeit(miller_rabin_test, number=num_trials)
    total_time_rust = timeit.timeit(miller_rabin_test_rust, number=num_trials)
    avg_time_rust = total_time_rust / num_trials
    avg_time_python = total_time_py / num_trials
    print(
        f"trials: {num_trials} iteration MIN: {MIN} MAX: {MAX} \n"
        f"sympy total:{total_time_py} avg: {avg_time_python:.6f} seconds \n"
        f"rust total: {total_time_rust} avg {avg_time_rust} \n"
    )


benchmark_miller_rabin()
