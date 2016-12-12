use std::hash::{Hasher, BuildHasher};


// ***** ID hash *****
pub struct IdentityHash { hash: u64 }

impl IdentityHash {
    pub fn new() -> IdentityHash {
        IdentityHash { hash: 0 }
    }
}

impl Hasher for IdentityHash {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        unsafe {
            if msg.len() != 8 {
                panic!("IdentityHash is only valid for u64 keys! len: {:?}", msg.len());
            }
            self.hash = *(msg.as_ptr() as *mut u64);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

impl BuildHasher for IdentityHash {
    type Hasher = IdentityHash;
    fn build_hasher(&self) -> Self::Hasher {
        let mut murm = IdentityHash::new();
        murm.hash = self.hash;
        murm
    }
}

// ***** U64Hash *****
pub struct U64Hash { hash: u64 }

impl U64Hash {
    pub fn new() -> U64Hash {
        U64Hash { hash: 0 }
    }
}

impl Hasher for U64Hash {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        unsafe {
            if msg.len() != 8 {
                panic!("U64Hash is only valid for u64 keys! len: {:?}", msg.len());
            }
           
            let new_val: u64 =  *(msg.as_ptr() as *mut u64); 

            self.hash = hash_u64(new_val);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

impl BuildHasher for U64Hash {
    type Hasher = U64Hash;
    fn build_hasher(&self) -> Self::Hasher {
        let mut murm = U64Hash::new();
        murm.hash = self.hash;
        murm
    }
}

pub fn hash_u64(seed: u64) -> u64 {
    // let w: u32 = 64;
    let a = 11400714819323198549u64;

    // assert!(m <= w, "hash_u64(): m is too large! m: {:?} w: {:?}", m, w);

    // let hi_bits = w-m;

    let val = a.wrapping_mul(seed);
    // val = val.wrapping_shr(hi_bits);
    // val = val.rotate_right(hi_bits);
    // val
    val
}

// ***** Murmur2_64a ***** 

#[allow(non_camel_case_types)]
pub struct Murmur2_64a {
    seed: u64
}

impl Murmur2_64a {
    pub fn new() -> Murmur2_64a {
        Murmur2_64a{ seed: 0 }
    }
}


impl Hasher for Murmur2_64a {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.seed = murmur_hash64a(msg, self.seed);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.seed as u64
    }
}

impl BuildHasher for Murmur2_64a {
    type Hasher = Murmur2_64a;
    fn build_hasher(&self) -> Self::Hasher {
        let mut murm = Murmur2_64a::new();
        murm.seed = self.seed;
        murm
    }
}

pub fn murmur_hash64a(key: &[u8], seed: u64) -> u64 {
    let m : u64 = 0xc6a4a7935bd1e995;
    let r : u8 = 47;

    let len = key.len();
    let mut hash : u64 = seed ^ ((len as u64).wrapping_mul(m));

    // let end = len >> 3;
    let nblocks = (len >> 3) as isize;

    let mut k: u64;
    let blocks = key.as_ptr() as *mut u64;

    unsafe {
        for i in 0..nblocks {
            k = *blocks.offset(i);

            k = k.wrapping_mul(m);
            k ^= k >> r;            
            k = k.wrapping_mul(m);

            hash ^= k;
            hash = hash.wrapping_mul(m);
        }

        let tail = blocks.offset(nblocks) as *mut u8;

        match len & 7 {
            7 => {
                hash ^= (*tail.offset(6) as u64) << 48;
                hash ^= (*tail.offset(5) as u64) << 40;
                hash ^= (*tail.offset(4) as u64) << 32;
                hash ^= (*tail.offset(3) as u64) << 24;
                hash ^= (*tail.offset(2) as u64) << 16;
                hash ^= (*tail.offset(1) as u64) << 8;
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            6 => {
                hash ^= (*tail.offset(5) as u64) << 40;
                hash ^= (*tail.offset(4) as u64) << 32;
                hash ^= (*tail.offset(3) as u64) << 24;
                hash ^= (*tail.offset(2) as u64) << 16;
                hash ^= (*tail.offset(1) as u64) << 8;
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            5 => {
                hash ^= (*tail.offset(4) as u64) << 32;
                hash ^= (*tail.offset(3) as u64) << 24;
                hash ^= (*tail.offset(2) as u64) << 16;
                hash ^= (*tail.offset(1) as u64) << 8;
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            4 => {
                hash ^= (*tail.offset(3) as u64) << 24;
                hash ^= (*tail.offset(2) as u64) << 16;
                hash ^= (*tail.offset(1) as u64) << 8;
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            3 => {
                hash ^= (*tail.offset(2) as u64) << 16;
                hash ^= (*tail.offset(1) as u64) << 8;
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            2 => {
                hash ^= (*tail.offset(1) as u64) << 8;
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            1 => {
                hash ^= *tail as u64;
                hash = hash.wrapping_mul(m);
            },
            _ => {},
        }
    }

    hash ^= hash >> r;
    hash = hash.wrapping_mul(m);
    hash ^= hash >> r;
    hash
}
