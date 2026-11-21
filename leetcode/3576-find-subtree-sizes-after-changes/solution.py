from collections import *

class Solution:
    def findSubtreeSizes(self, parent: List[int], s: str) -> List[int]:
        # build adj list
        adj = defaultdict(list)
        for i,v in enumerate(parent):
            adj[v].append(i)
        # find new parent
        def dfs(v, anc={}):
            if s[v] in anc:
                parent[v] = anc[s[v]]
            for u in adj[v]:
                dfs(u, anc | { s[v]: v })
        dfs(0)
        # build adj list
        adj = defaultdict(list)
        for i,v in enumerate(parent):
            adj[v].append(i)
        print(adj)
        # calculate size
        answer = [0] * len(parent)
        def f(v):
            if not answer[v]:
                answer[v] = 1 + sum(f(u) for u in adj[v])
            return answer[v]
        print(f(0))
        return answer
            

