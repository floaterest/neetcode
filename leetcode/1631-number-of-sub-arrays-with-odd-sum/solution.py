class Solution:
    def numOfSubarrays(self, xs: list[int]) -> int:
        MOD = 10 ** 9+7
        # dp[i] is number of odd-sum subarr ending at xs[i]
        dp = [0] * len(xs)
        for i, x in enumerate(xs):
            if i == 0:
                dp[i] = x & 1
                continue
            odd = dp[i-1]
            if x & 1: # x odd
                even = i - odd
                dp[i] = (even + 1) % MOD
            else: # x even
                dp[i] = odd
        return sum(dp) % MOD

                
