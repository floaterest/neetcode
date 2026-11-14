class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        pr = [1]
        for n in nums[:-1]:
            pr.append(n * pr[-1])
        su = [1]
        for n in reversed(nums[1:]):
            su.append(n * su[-1])
        return [a*b for a,b in zip(pr,reversed(su))]

