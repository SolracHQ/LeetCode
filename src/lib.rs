trait Sortable<T: Ord> {
    fn sorted(&mut self) -> &mut Self;
}

impl <T: Ord> Sortable<T> for Vec<T> {
    fn sorted(&mut self) -> &mut Self {
        self.sort_unstable();
        self
    }
}

#[cfg(test)]
mod s0001;
#[cfg(test)]
mod s0002;
#[cfg(test)]
mod s0003;
#[cfg(test)]
mod s0005;
#[cfg(test)]
mod s0006;
#[cfg(test)]
mod s0007;
#[cfg(test)]
mod s0008;
#[cfg(test)]
mod s0009;
#[cfg(test)]
mod s0010;
#[cfg(test)]
mod s0011;
#[cfg(test)]
mod s0012;
#[cfg(test)]
mod s0013;
#[cfg(test)]
mod s0014;
#[cfg(test)]
mod s0015;
#[cfg(test)]
mod s0017;
#[cfg(test)]
mod s0019;
#[cfg(test)]
mod s0020;
#[cfg(test)]
mod s0021;
#[cfg(test)]
mod s0022;
#[cfg(test)]
mod s0023;
#[cfg(test)]
mod s0026;
#[cfg(test)]
mod s0027;
#[cfg(test)]
mod s0028;
#[cfg(test)]
mod s0029;
#[cfg(test)]
mod s0032;