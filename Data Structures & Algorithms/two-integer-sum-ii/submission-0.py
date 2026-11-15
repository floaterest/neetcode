class Solution:
    def twoSum(self, xs: List[int], t: int) -> List[int]:
        f=lambda i,j:xs[i]+xs[j]
        i,j=0,len(xs)-1
        while (s:=xs[i]+xs[j])!=t:
            if s<t:
                i+=1
            else: # s>t
                j-=1
        return [i+1,j+1]