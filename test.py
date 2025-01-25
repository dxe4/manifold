from manifold_rs import miller_rabin_bool

import timeit
from sympy.ntheory.primetest import mr as miller_rabin


# MIN, MAX = 10**3, 10**5
MIN, MAX = 10**6, 10**7
def benchmark_miller_rabin(num_trials=5):

    def miller_rabin_test():
        for i in range(MIN, MAX):
            is_prime = miller_rabin(i, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 47, 53, 59, 67, 71, 73, 79, 97])

    def miller_rabin_test_rust():
        for i in range(MIN, MAX):
            is_prime = miller_rabin_bool(i)

    total_time_py = timeit.timeit(miller_rabin_test, number=num_trials)
    total_time_rust = timeit.timeit(miller_rabin_test_rust, number=num_trials)
    avg_time_rust = total_time_rust / num_trials
    avg_time_python = total_time_py / num_trials
    print(f"trials: {num_trials} sympy total:{total_time_py} avg: {avg_time_python:.6f} seconds -- rust total: {total_time_rust} avg {avg_time_rust}")

benchmark_miller_rabin()
