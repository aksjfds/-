
import shutil
import os
import numpy as np
import polars as pl


def update():
    pyd = r'C:\Users\AKSJF\PycharmProjects\机器学习\分类\rust_python\src\rust_python.pyd'
    dll = r'C:\Users\AKSJF\PycharmProjects\机器学习\分类\rust_python\target\debug\rust_python.dll'
    with open(dll, mode='rb') as f:
        content = f.read()
        with open(pyd, mode='wb') as f2:
            f2.write(content)

# update()


A = np.ones((3,2,2))
A[0,1,1] = 0
A[0,0,1] = 0
A[0,1,0] = 100
print(A)
print()
print(A.all(0))

