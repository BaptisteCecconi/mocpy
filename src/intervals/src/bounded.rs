use num::{Integer, One, PrimInt};

#[inline(always)]
const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

const TO_EVEN_MASK: u32 = !0x1;

pub trait Bounded<T>
where
    T: Integer + PrimInt,
{
    const MAXDEPTH: i8;
    const MAXPIX: T;

    #[inline(always)]
    fn get_msb(x: T) -> u32 {
        num_bits::<T>() as u32 - x.leading_zeros() - 1
    }

    #[inline(always)]
    fn pix_depth(u: T) -> (u32, T) {
        let msb = Self::get_msb(u) & TO_EVEN_MASK;

        let depth = (msb >> 1) - 1;
        let t: T = One::one();
        let pix = u - t.unsigned_shl(msb);

        (depth, pix)
    }
}

impl Bounded<u128> for u128 {
    const MAXDEPTH: i8 = 62;
    const MAXPIX: u128 = 3 << 126;
}

impl Bounded<u64> for u64 {
    const MAXDEPTH: i8 = 29;
    const MAXPIX: u64 = 3 << 60;
}
impl Bounded<i64> for i64 {
    const MAXDEPTH: i8 = 29;
    const MAXPIX: i64 = 3 << 60;
}

impl Bounded<u32> for u32 {
    const MAXDEPTH: i8 = 13;
    const MAXPIX: u32 = 3 << 28;
}
impl Bounded<u8> for u8 {
    const MAXDEPTH: i8 = 2;
    const MAXPIX: u8 = 3 << 6;
}