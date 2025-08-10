pub(super) struct StrIt<'a> {
    pub(super) s: &'a [u8],
    pub(super) pos: usize,
}

impl<'a> StrIt<'a> {
    #[inline(always)]
    pub fn peek(&self) -> Option<&u8> {
        self.s.get(self.pos)
    }

    pub fn pop(&mut self) -> Option<u8> {
        let c = self.peek().copied()?;
        self.pos += 1;
        Some(c)
    }

    #[inline(always)]
    pub fn starts_with(&self, cs: &[u8]) -> bool {
        self.s[self.pos..].starts_with(cs)
    }

    #[inline(always)]
    pub fn shift(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn peek_n(&self, n: usize) -> String {
        let chars = self.s[self.pos..self.pos + n]
            .iter()
            .copied()
            .collect::<Vec<_>>();
        String::from_utf8(chars).unwrap()
    }
}
