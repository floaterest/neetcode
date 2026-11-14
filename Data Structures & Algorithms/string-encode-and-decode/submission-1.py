class Solution:

    def encode(self, strs: List[str]) -> str:
        return f'{len(strs)}あ%s' % "あ".join(strs)

    def decode(self, s: str) -> List[str]:
        n, *rest = s.split('あ')
        return rest if int(n) else []
