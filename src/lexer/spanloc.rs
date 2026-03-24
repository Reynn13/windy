use std::{
    fmt::{Display, Formatter},
    ops::Add
};

#[derive(Clone, Copy)]
pub struct SpanLoc {
    pub start_idx: u32,
    pub end_idx: u32,
    pub start_col: u16,
    pub end_col: u16,
    pub start_line: u16,
    pub end_line: u16,
}


impl SpanLoc {
    pub fn new(
        si: usize, ei: usize,
        sc: usize, ec: usize,
        sl: usize, el: usize
    ) -> Self {
        Self {
            start_idx: si as u32,
            end_idx: ei as u32,
            start_col: sc as u16,
            end_col: ec as u16,
            start_line: sl as u16,
            end_line: el as u16,
        }
    }
    fn real_new(
        si: u32, ei: u32,
        sc: u16, ec: u16,
        sl: u16, el: u16
    ) -> Self {
        Self {
            start_idx: si,
            end_idx: ei,
            start_col: sc,
            end_col: ec,
            start_line: sl,
            end_line: el,
        }
    }
}

impl Display for SpanLoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "idx {}-{}, col {}-{}, ln {}-{}", self.start_idx, self.end_idx, self.start_col, self.end_col, self.start_line, self.end_line)
    }
}

impl Add for SpanLoc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::real_new(
            self.start_idx, rhs.end_idx,
            self.start_col, rhs.end_col,
            self.start_line, rhs.end_line
        )
    }
}
