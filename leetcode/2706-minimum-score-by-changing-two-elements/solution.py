class Solution:
    def minimizeSum(self, nums: List[int]) -> int:
        nums.sort()
        a = nums[-2] - nums[1]  # change first and last
        b = nums[-1] - nums[2]  # change first two
        c = nums[-3] - nums[0]
        return min(a,b,c)
