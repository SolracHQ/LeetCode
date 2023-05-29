trait Sortable<T: Ord> {
    fn sorted(&mut self) -> &mut Self;
}

impl <T: Ord> Sortable<T> for Vec<T> {
    fn sorted(&mut self) -> &mut Self {
        self.sort_unstable();
        self
    }
}

mod s0001;
mod s0002;
mod s0003;
mod s0005;
mod s0006;
mod s0007;
mod s0008;
mod s0009;
mod s0010;
mod s0011;
mod s0012;
mod s0013;
mod s0014;
mod s0015;
mod s0017;
mod s0019;
mod s0020;
mod s0022;