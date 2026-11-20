class Solution:
    def maxSelectedElements(self, nums: List[int]) -> int:
        nums.sort()
        dp0 = [1] * len(nums)
        dp1 = [1] * len(nums)
        for i in range(1, len(nums)):
            d = nums[i] - nums[i-1]
            x0, x1 = dp0[i-1], dp1[i-1]
            if d == 0: # 2, 2
                dp0[i] = x0
                dp1[i] = x0 + 1
            elif d == 1: # 2, 3
                dp0[i] = max(x0+1, x1)
                dp1[i] = x1+1
            elif d == 2: # 2, 4
                dp0[i] = x1 + 1
        return max(*dp0, *dp1)
