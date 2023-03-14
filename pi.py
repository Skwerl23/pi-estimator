import random
import time 
import math

start = time.time()
a = 0.0
B = 100_000_000.0

for _ in range(int(B)):
    x = random.random()
    y = random.random()
    if math.sqrt(x*x + y*y) < 1.0:
        a += 1.0

end = time.time()

elapsed = end-start
print(f"Estimated pi: {a/B*4.0}");
print(f"Python Elapsed time: {elapsed:.6f} seconds")
