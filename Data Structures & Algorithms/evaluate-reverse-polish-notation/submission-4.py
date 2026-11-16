from operator import *

OPS = {"+": add, "-": sub, "*": mul, "/": truediv}


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        st = []
        for t in tokens:
            if t in OPS:
                b, a = st.pop(), st.pop()
                st.append(int(OPS[t](a, b)))
            else:
                st.append(int(t))
        return st.pop()
