class Solution:
    def minimumChanges(self, s: str, k: int) -> int:
        n = len(s)
        
        # Precompute proper divisors for every possible substring length
        divisors = [[] for _ in range(n + 1)]
        for length in range(2, n + 1):
            for d in range(1, length):
                if length % d == 0:
                    divisors[length].append(d)
        
        # Precompute minimum changes for every substring s[i..j]
        INF = float('inf')
        cost = [[INF] * n for _ in range(n)]
        
        for i in range(n):
            for j in range(i + 1, n):
                length = j - i + 1
                min_changes = INF
                for d in divisors[length]:
                    changes = 0
                    for rem in range(d):
                        left = i + rem
                        right = j - d + 1 + rem
                        while left < right:
                            if s[left] != s[right]:
                                changes += 1
                            left += d
                            right -= d
                    if changes < min_changes:
                        min_changes = changes
                cost[i][j] = min_changes
                
        # DP to partition prefix of length i into p semi-palindromes
        dp = [[INF] * (k + 1) for _ in range(n + 1)]
        dp[0][0] = 0
        
        for p in range(1, k + 1):
            for i in range(2 * p, n + 1):
                for m in range(2 * (p - 1), i - 1):
                    if dp[m][p - 1] + cost[m][i - 1] < dp[i][p]:
                        dp[i][p] = dp[m][p - 1] + cost[m][i - 1]
                        
        return dp[n][k]

