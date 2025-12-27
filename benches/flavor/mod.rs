use std::convert::Infallible;

pub struct PreallocatedVec<'a> {
    vec: &'a mut Vec<u8>,
}

impl<'a> PreallocatedVec<'a> {
    pub fn new(vec: &'a mut Vec<u8>) -> Self {
        PreallocatedVec { vec }
    }
}

impl<'a> postcard::ser_flavors::Flavor for PreallocatedVec<'a> {
    type Output = ();

    #[inline]
    fn try_extend(&mut self, data: &[u8]) -> postcard::Result<()> {
        self.vec.extend_from_slice(data);
        Ok(())
    }

    #[inline]
    fn try_push(&mut self, data: u8) -> postcard::Result<()> {
        self.vec.push(data);
        Ok(())
    }

    #[inline]
    fn finalize(self) -> postcard::Result<Self::Output> {
        Ok(())
    }
}

impl<'a> postcard2::ser_flavors::Flavor for PreallocatedVec<'a> {
    type Output = ();
    type PushError = Infallible;
    type FinalizeError = Infallible;

    #[inline]
    fn try_extend(&mut self, data: &[u8]) -> Result<(), Infallible> {
        self.vec.extend_from_slice(data);
        Ok(())
    }

    #[inline]
    fn try_push(&mut self, data: u8) -> Result<(), Infallible> {
        self.vec.push(data);
        Ok(())
    }

    #[inline]
    fn finalize(self) -> Result<Self::Output, Infallible> {
        Ok(())
    }
}
