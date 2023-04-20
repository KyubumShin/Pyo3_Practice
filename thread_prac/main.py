from multiprocessing import Process
from typing import Optional
import time

class ExampleProcess(Process):
    def __init__(self, seconds: int, name: str) -> None:
        super().__init__()
        self.seconds:int = seconds
        self.name = name
        self._return = None
    
    def run(self) -> None:
        pass
    
    def join(self) -> int:
        Process.join(self)
        return self._return

def recur_fibo(n: int) -> int:
    if n <= 1:
        return n
    else:
        return (recur_fibo(n-1) + recur_fibo(n-2))

def main():
    start = time.time()

    recur_fibo(n=8)
    recur_fibo(n=12)
    recur_fibo(n=12)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=36)
    finish = time.time()
    print(f"{finish - start} has elapsed")


if __name__ == "__main__":
    main()
    from multiprocessing import Pool
    start = time.time()

    with Pool(4) as p:
        print(p.starmap(recur_fibo, [(8,), (12,), (12,), (20,), (20,), (20,), (20,), (28,), (28,), (28,), (28,), (36,)]))
    finish = time.time()
    print(f"{finish - start} has elapsed")