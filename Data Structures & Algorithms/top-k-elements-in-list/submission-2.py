class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        a = [[] for _ in range(len(nums)+1)]
        for n,c in Counter(nums).items():
            a[c].append(n)
        return [n for x in a for n in x][-k:]