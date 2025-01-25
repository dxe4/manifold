number theory playground
port some logic from sympy to rust and run benchmakrs

miller rabin benchmark for numbers 1,200
the speedup hypothetically is higher for larger numbers
but this version crashes above 255
this is likey caused by the logic in SMALL_TRAILING
needs trouble shooting

trials: 100000 sympy total:123.5343531339895 avg: 0.001235 seconds -- rust total: 34.366217914037406 avg 0.0003436621791403741
