class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        a = sorted(((v,k) for k,v in Counter(nums).items()), reverse=True)
        return [k for _,k in a][:k]