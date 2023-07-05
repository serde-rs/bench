use postcard::ser_flavors::Flavor;
use postcard::Result;

pub struct PreallocatedVec<'a> {
    vec: &'a mut Vec<u8>,
}

impl<'a> PreallocatedVec<'a> {
    pub fn new(vec: &'a mut Vec<u8>) -> Self {
        PreallocatedVec { vec }
    }
}

impl<'a> Flavor for PreallocatedVec<'a> {
    type Output = ();

    #[inline]
    fn try_extend(&mut self, data: &[u8]) -> Result<()> {
        self.vec.extend_from_slice(data);
        Ok(())
    }

    #[inline]
    fn try_push(&mut self, data: u8) -> Result<()> {
        self.vec.push(data);
        Ok(())
    }

    #[inline]
    fn finalize(self) -> Result<Self::Output> {
        Ok(())
    }
}
