/*
 * encoding=utf-8
 * 无符号整数编解码操作接口。
 * 历史：
 *     2020-11-21，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub trait Code {
    //以大端序解码整数。
    fn decode_be(&mut self, pos: &[u8]);
    /*------------------------------------------------------------------------*/
    //以小端序解码整数。
    fn decode_le(&mut self, pos: &[u8]);
    /*------------------------------------------------------------------------*/
    //以大端序编码整数。
    fn encode_be(self, pos: &mut [u8]);
    /*------------------------------------------------------------------------*/
    //以小端序编码整数。
    fn encode_le(self, pos: &mut [u8]);
}
/*============================================================================*/
macro_rules! code_impl {
    ($($t:ty)*) => ($(
        impl Code for $t {
            #[inline]
            fn decode_be(&mut self, pos: &[u8]) {
                let mut val: Self = pos[0] as Self;

                for x in &pos[1..] {
                    val <<= 8;
                    val |= *x as Self;
                }

                *self = val;
            }
            /*----------------------------------------------------------------*/
            #[inline]
            fn decode_le(&mut self, pos: &[u8]) {
                let mut len = pos.len() - 1;
                let mut val: Self = pos[len] as Self;

                while len != 0 {
                    len -= 1;
                    val <<= 8;
                    val |= pos[len] as Self;
                }

                *self = val;
            }
            /*----------------------------------------------------------------*/
            #[inline]
            fn encode_be(self, pos: &mut [u8]) {
                let mut len = pos.len() - 1;
                let mut val = self;

                pos[len] = val as u8;
                while len != 0 {
                    len -= 1;
                    val >>= 8;
                    pos[len] = val as u8;
                }
            }
            /*----------------------------------------------------------------*/
            #[inline]
            fn encode_le(self, pos: &mut [u8]) {
                let mut val = self;
                pos[0] = val as u8;

                for x in &mut pos[1..] {
                    val >>= 8;
                    *x = val as u8;
                }
            }
        }
    )*)
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
code_impl! { usize u16 u32 u64 u128 }
/*----------------------------------------------------------------------------*/
impl Code for u8 {
    #[inline]
    fn decode_be(&mut self, pos: &[u8]) {
        *self = pos[pos.len() - 1];
    }
    /*------------------------------------------------------------------------*/
    #[inline]
    fn decode_le(&mut self, pos: &[u8]) {
        *self = pos[0];
    }
    /*------------------------------------------------------------------------*/
    #[inline]
    fn encode_be(self, pos: &mut [u8]) {
        let mut len = pos.len() - 1;

        pos[len] = self;
        while len != 0 {
            len -= 1;
            pos[len] = 0;
        }
    }
    /*------------------------------------------------------------------------*/
    #[inline]
    fn encode_le(self, pos: &mut [u8]) {
        pos[0] = self;

        for x in &mut pos[1..] {
            *x = 0;
        }
    }
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
