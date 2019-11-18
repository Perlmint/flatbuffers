use super::builder::FlatBufferBuilder;
use follow::Follow;
use std::ops::{Deref, DerefMut};

/*
 * Self owned buffer
 */
pub trait Message: Sized {
    fn data(&self) -> &[u8];

    fn len(&self) -> usize {
        self.data().len()
    }

    fn encode<'a, B: Extend<&'a u8>>(&'a self, buffer: &'a mut B) {
        buffer.extend(self.data());
    }

    fn decode(buffer: &[u8]) -> (Self, usize);

    fn get_root_impl<'a, T>(&'a self) -> T::Inner
    where
        T: Follow<'a>,
    {
        let buf_ref: &[u8] = unsafe { &*(self.data().as_ref() as *const [u8]) };
        // read root table offset
        let offset = super::read_scalar_at::<u32>(buf_ref, 0);
        T::follow(buf_ref, offset as usize)
    }
}

pub struct MessageBuilder<'fbb>(FlatBufferBuilder<'fbb>);

impl<'fbb> MessageBuilder<'fbb> {
    pub fn release_message<T: Message>(&mut self) -> T {
        let message_buf = self.0.owned_buf.split_off(self.0.head);
        self.0.reset();

        T::decode(&message_buf).0
    }
}

impl<'fbb> Deref for MessageBuilder<'fbb> {
    type Target = FlatBufferBuilder<'fbb>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'fbb> DerefMut for MessageBuilder<'fbb> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
