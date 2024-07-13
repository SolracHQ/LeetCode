#[cfg(test)]
trait Sortable<T: Ord> {
    fn sorted(&mut self) -> &mut Self;
}

#[cfg(test)]
impl<T: Ord> Sortable<T> for Vec<T> {
    fn sorted(&mut self) -> &mut Self {
        self.sort_unstable();
        self
    }
}
#[cfg(test)]
mod solutions;
