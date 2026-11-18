class Solution:
    def minChanges(self, s: str) -> int:
        x = ''
        a = []
        for c in s:
            if c == x:
                a[-1] += 1
            else:
                x = c
                a.append(1)
        print(a)
        c = 0
        for i in range(len(a) - 1):
            if a[i] & 1:
                c += 1
                a[i+1] += 1
        return c

