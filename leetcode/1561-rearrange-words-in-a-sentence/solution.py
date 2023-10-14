class Solution:
    def arrangeWords(self, s: str) -> str:
        s = sorted(s.lower().split(' '), key=str.__len__)
        s[0] = s[0].capitalize()
        return ' '.join(s)
