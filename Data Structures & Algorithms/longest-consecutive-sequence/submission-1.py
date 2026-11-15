class Solution:
    def longestConsecutive(self, a: List[int]) -> int:
        if not a:
            return 0
        # when n not in d
        #   d[n-1] is longest seq that ends at n-1
        #   d[n+1] is longest seq that starts at n+1
        d = defaultdict(int)
        for x in a:
            if d[x] > 0:
                continue
            d[x] = d[x-1] + d[x+1] + 1
            # update info at boundaries
            d[x - d[x-1]] = d[x]
            d[x + d[x+1]] = d[x]
        return max(d.values())


        