from operator import *
from collections import deque

OPS = {"+": add, "-": sub, "*": mul, "/": lambda a,b: int(a/b)}


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        st = deque()

        for t in tokens:
            if t in OPS:
                b, a = st.pop(), st.pop()
                st.append(OPS[t](a, b))
            else:
                st.append(int(t))
        return st.pop()
