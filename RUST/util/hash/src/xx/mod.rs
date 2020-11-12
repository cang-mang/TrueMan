/*
 * encoding=utf-8
 * XX-HASH散列操作接口。
 * 历史：
 *     2020-11-10，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use std::convert::TryInto;
/*============================================================================*/
const H32: [u32; 5] = [
    2654435761_u32, 2246822519_u32, 3266489917_u32, 668265263_u32, 374761393_u32
];
/*----------------------------------------------------------------------------*/
fn round32(seed: u32, val: u32) -> u32 {
    seed.wrapping_add(val.wrapping_mul(H32[1])).rotate_left(13)
        .wrapping_mul(H32[0])
}
/*----------------------------------------------------------------------------*/
pub fn x32(key: &[u8], seed: u32) -> u32 {
    let mut k = key;
    let mut h = if key.len() >= 16 {
        let mut v = [
            seed.wrapping_add(H32[0]).wrapping_add(H32[1]),
            seed.wrapping_add(H32[1]),
            seed,
            seed.wrapping_sub(H32[0])
        ];
        loop {
            let (x, a) = k.split_at(4);
            v[0] = round32(v[0], u32::from_le_bytes(x.try_into().unwrap()));

            let (x, b) = a.split_at(4);
            v[1] = round32(v[1], u32::from_le_bytes(x.try_into().unwrap()));

            let (x, a) = b.split_at(4);
            v[2] = round32(v[2], u32::from_le_bytes(x.try_into().unwrap()));

            let (x, b) = a.split_at(4);
            v[3] = round32(v[3], u32::from_le_bytes(x.try_into().unwrap()));

            k = b;
            if k.len() < 16 {
                break;
            }
        }

        v[0].rotate_left(1).wrapping_add(v[1].rotate_left( 7))
                           .wrapping_add(v[2].rotate_left(12))
                           .wrapping_add(v[3].rotate_left(18))
    } else {
        seed.wrapping_add(H32[4])
    };

    h = h.wrapping_add(key.len() as u32);

    while k.len() >= 4 {
        let (x, a) = k.split_at(4);
        h = h.wrapping_add(
            u32::from_le_bytes(x.try_into().unwrap()).wrapping_mul(H32[2])
        );
        h = h.rotate_left(17).wrapping_mul(H32[3]);
        k = a;
    }

    if k.len() != 0 {
        for o in k {
            h = h.wrapping_add((*o as u32).wrapping_mul(H32[4]));
            h = h.rotate_left(11).wrapping_mul(H32[0]);
        }
    }

    h ^= h >> 15; h = h.wrapping_mul(H32[1]);
    h ^= h >> 13; h = h.wrapping_mul(H32[2]);

    h ^ h >> 16
}
/*----------------------------------------------------------------------------*/
const H64: [u64; 5] = [
    11400714785074694791_u64,
    14029467366897019727_u64,
     1609587929392839161_u64,
     9650029242287828579_u64,
     2870177450012600261_u64
];
/*----------------------------------------------------------------------------*/
fn round64(seed: u64, val: u64) -> u64 {
    seed.wrapping_add(val.wrapping_mul(H64[1])).rotate_left(31)
        .wrapping_mul(H64[0])
}
/*----------------------------------------------------------------------------*/
fn merge64(seed: u64, val: u64) -> u64 {
    (seed ^ round64(0, val)).wrapping_mul(H64[0]).wrapping_add(H64[3])
}
/*----------------------------------------------------------------------------*/
pub fn x64(key: &[u8], seed: u64) -> u64 {
    let mut k = key;
    let mut h = if key.len() >= 32 {
        let mut v = [
            seed.wrapping_add(H64[0]).wrapping_add(H64[1]),
            seed.wrapping_add(H64[1]),
            seed,
            seed.wrapping_sub(H64[0])
        ];
        loop {
            let (x, a) = k.split_at(8);
            v[0] = round64(v[0], u64::from_le_bytes(x.try_into().unwrap()));

            let (x, b) = a.split_at(8);
            v[1] = round64(v[1], u64::from_le_bytes(x.try_into().unwrap()));

            let (x, a) = b.split_at(8);
            v[2] = round64(v[2], u64::from_le_bytes(x.try_into().unwrap()));

            let (x, b) = a.split_at(8);
            v[3] = round64(v[3], u64::from_le_bytes(x.try_into().unwrap()));

            k = b;
            if k.len() < 32 {
                break;
            }
        }

        merge64(
            merge64(
                merge64(
                    merge64(
                        v[0].rotate_left(1).wrapping_add(v[1].rotate_left( 7))
                                           .wrapping_add(v[2].rotate_left(12))
                                           .wrapping_add(v[3].rotate_left(18)),
                        v[0]
                    ),
                    v[1]
                ),
                v[2]
            ),
            v[3]
        )
    } else {
        seed.wrapping_add(H64[4])
    };

    h = h.wrapping_add(key.len() as u64);

    while k.len() >= 8 {
        let (x, a) = k.split_at(8);
        h ^= round64(0, u64::from_le_bytes(x.try_into().unwrap()));
        h = h.rotate_left(27).wrapping_mul(H64[0]).wrapping_add(H64[3]);
        k = a;
    }

    if k.len() >= 4 {
        let (x, a) = k.split_at(4);
        h ^= (
            u32::from_le_bytes(x.try_into().unwrap()) as u64
        ).wrapping_mul(H64[0]);
        h = h.rotate_left(23).wrapping_mul(H64[1]).wrapping_add(H64[2]);
        k = a;
    }

    if k.len() != 0 {
        for o in k {
            h ^= (*o as u64).wrapping_mul(H64[4]);
            h = h.rotate_left(11).wrapping_mul(H64[0]);
        }
    }

    h ^= h >> 33; h = h.wrapping_mul(H64[1]);
    h ^= h >> 29; h = h.wrapping_mul(H64[2]);

    h ^ h >> 32
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
