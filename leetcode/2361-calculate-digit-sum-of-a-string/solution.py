import itertools


def f(s, k):
    for ds in itertools.batched(s, n=k):
        yield str(sum(int(d) for d in ds))


class Solution:
    def digitSum(self, s: str, k: int) -> str:
        if len(s) <= k:
            return s
        return self.digitSum("".join(f(s, k)), k)

