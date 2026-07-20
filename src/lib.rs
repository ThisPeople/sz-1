use std::convert::TryInto;

// ========== КОНСТАНТЫ ==========
const H0: u64 = 0x6a09e66993bcc908;
const H1: u64 = 0xbbeeae899ccaa73b;
const H2: u64 = 0x3c6ef372fe94f82b;
const H3: u64 = 0xa54ff53a5f1d36f1;
const H4: u64 = 0x510e527fade682d1;
const H5: u64 = 0x9b05688c2b3e6c1f;
const H6: u64 = 0x1f83d9abfb41bd6b;
const H7: u64 = 0x5be0cd19137e2179;

// Генерируем K-константы (те же, что в Python)
const K: [u64; 80] = [
    0x270d6865e0f2f385, 0x9effbede62b46a29, 0x6c65b4977e453753,
    0x8e8d464b143987b4, 0xb447404b4bce1dd0, 0x197d39ac175e7cd6,
    0xe25b7b4a6c53711a, 0x3de7991e6f70ecf4, 0x66eb289169acdf86,
    0x6bf92f8a4857f58a, 0x1f62cdc068ba87a6, 0x4f1c3b7f23ae200d,
    0x850db3f3ccb3bc4a, 0x41fd5bcc551095f0, 0x7a94cd9097e87664,
    0xf9d78db9ed93d1b7, 0xcf6631649c3cb5dc, 0x9c6b456685e1c328,
    0x2dd06754b7b40e68, 0xee095a54ed13708b, 0x2a9d0cdfc5c4a4cb,
    0x06f2dc4032534b13, 0x37862c06a69e6666, 0x2b835454137276c2,
    0x46f5dc21ae241d56, 0x6424de1f0edda258, 0x4a9ccb03a615029f,
    0x6128787cfc61e53e, 0x14ef4da5e9025f6b, 0x8fe8a1de19bcf2df,
    0xb1866054e85bd4ce, 0x951ed27e4c18cd28, 0x57e55d1e88dde60e,
    0x98aaeea2b4e29ceb, 0x9097ffbed35ebf27, 0x1b8aa8fcde6e7444,
    0x4835d8075511d295, 0x6d2b8f854d77c6f2, 0x67c2a905b2b075cb,
    0x2628d7837b5b56fc, 0xb6a63373899bd2f6, 0x6cbd10cf985731d7,
    0xf60a507fab7c2c4e, 0xb8df77dc252bce7a, 0x38c05e9fdeea6be8,
    0xfe2232a8eb8e1036, 0x6c4b406675a3d5fb, 0x0b119d2614bc5b40,
    0x9eedf253c2429cc1, 0xcf768141f434ff19, 0x0b46827b40dd9c6a,
    0xd23e7cf6c22ff253, 0xa7a8e011a1d30a94, 0xb48b6be528196134,
    0xab7b525ed9c4bb25, 0x899a4ff5249b592e, 0xa74bda84c575fb76,
    0x8087a863a31680c7, 0x0ee479b9baa3e2c7, 0xf2809fd9ad43baed,
    0x282b4941ab92a255, 0x79285b7d16aa1502, 0x282c99a559641316,
    0xfb12d705fbe7c0a2, 0x759a697916cfa17a, 0x89ae51f17b78cfdb,
    0xbd95a4208896dcab, 0x76230ef0fdd835fc, 0x1a1965a5a7fd0fad,
    0x11c14f50c0ac7099, 0x25c873f24a75bc44, 0x0a23b68508238c62,
    0xa598b5f88636abfe, 0x4c0cd348c8e92a80, 0xc68f42cb9ad2dc13,
    0x22b546aaa43bb410, 0xc339aa399ae7e942, 0x7e29b5519997a9c4,
    0x862a49a4bcf125aa, 0x209a72342a1df584,
];

// ========== ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ==========
#[inline]
fn rotr(x: u64, n: u32) -> u64 {
    (x >> n) | (x << (64 - n))
}

#[inline]
fn rotl(x: u64, n: u32) -> u64 {
    (x << n) | (x >> (64 - n))
}

#[inline]
fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ ((!x) & z)
}

#[inline]
fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline]
fn sigma0(x: u64) -> u64 {
    rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
}

#[inline]
fn sigma1(x: u64) -> u64 {
    rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
}

// ========== PADDING ==========
fn pad_data(data: &[u8]) -> Vec<u8> {
    let original_len = data.len();
    let mut padded = data.to_vec();
    
    // Добавляем 0x80
    padded.push(0x80);
    
    // Добавляем нули до 112 байт в блоке
    while (padded.len() % 128) != 112 {
        padded.push(0x00);
    }
    
    // Добавляем оригинальную длину в битах (big-endian)
    let bit_len = (original_len * 8) as u128;
    padded.extend_from_slice(&bit_len.to_be_bytes());
    
    padded
}

// ========== РАСШИРЕНИЕ БЛОКА ==========
fn expand_block(block: &[u8]) -> [u64; 80] {
    let mut w = [0u64; 80];
    
    // Первые 16 слов из блока
    for i in 0..16 {
        let start = i * 8;
        w[i] = u64::from_be_bytes(block[start..start+8].try_into().unwrap());
    }
    
    // Расширяем до 80 слов
    for i in 16..80 {
        let s0 = rotr(w[i-15], 1) ^ rotr(w[i-15], 8) ^ (w[i-15] >> 7);
        let s1 = rotr(w[i-2], 19) ^ rotr(w[i-2], 61) ^ (w[i-2] >> 6);
        w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
    }
    
    w
}

// ========== СУПЕР-АГРЕССИВНЫЙ РАУНД ==========
#[inline]
fn mix_state_super(
    a: u64, b: u64, c: u64, d: u64,
    e: u64, f: u64, g: u64, h: u64,
    w: &[u64; 80], round_num: usize
) -> (u64, u64, u64, u64, u64, u64, u64, u64) {
    
    let idx = round_num % 80;
    
    let mut t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(K[idx])
        .wrapping_add(w[idx]);
    
    let mut t2 = sigma0(a)
        .wrapping_add(maj(a, b, c));
    
    // Дополнительный хаос
    t1 ^= rotl(d, 13) ^ rotr(a, 7) ^ b;
    t2 ^= rotl(h, 11) ^ rotr(e, 17) ^ f;
    
    t1 = rotl(t1, 23);
    t2 = rotl(t2, 19);
    
    let old_a = a;
    let old_b = b;
    let old_c = c;
    let old_d = d;
    let old_e = e;
    let old_f = f;
    let old_g = g;
    let old_h = h;
    
    let new_a = t1
        .wrapping_add(t2)
        .wrapping_add(rotl(old_e, 3))
        .wrapping_add(rotr(old_h, 11));
    
    let new_b = rotl(old_a, 5)
        .wrapping_add(rotr(old_f, 7))
        .wrapping_add(t1);
    
    let new_c = rotl(old_b, 9)
        .wrapping_add(rotr(old_g, 13))
        .wrapping_add(t2);
    
    let new_d = rotl(old_c, 13)
        .wrapping_add(rotr(old_h, 17))
        .wrapping_add(t1)
        .wrapping_add(t2);
    
    let new_e = rotl(old_d, 17)
        .wrapping_add(rotr(old_a, 19))
        .wrapping_add(new_a ^ new_b);
    
    let new_f = rotl(old_e, 21)
        .wrapping_add(rotr(old_b, 23))
        .wrapping_add(new_c ^ new_d);
    
    let new_g = rotl(old_f, 25)
        .wrapping_add(rotr(old_c, 27))
        .wrapping_add(new_e ^ new_f);
    
    let new_h = rotl(old_g, 29)
        .wrapping_add(rotr(old_d, 31))
        .wrapping_add(new_g ^ new_a);
    
    // Финальный микс (убрали mut, т.к. не нужны)
    let final_a = new_a ^ rotl(new_b, 7) ^ rotr(new_c, 11);
    let final_e = new_e ^ rotl(new_f, 13) ^ rotr(new_g, 17);
    
    (final_a, new_b, new_c, new_d, final_e, new_f, new_g, new_h)
}

// ========== SZ-1 EXTREME ==========
pub fn sz1_extreme(data: &[u8]) -> [u8; 64] {
    let mut h0 = H0;
    let mut h1 = H1;
    let mut h2 = H2;
    let mut h3 = H3;
    let mut h4 = H4;
    let mut h5 = H5;
    let mut h6 = H6;
    let mut h7 = H7;
    
    let padded = pad_data(data);
    
    for chunk in padded.chunks(128) {
        let w = expand_block(chunk);
        
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;
        
        // 100 раундов
        for round in 0..100 {
            let (new_a, new_b, new_c, new_d, new_e, new_f, new_g, new_h) =
                mix_state_super(a, b, c, d, e, f, g, h, &w, round);
            
            a = new_a;
            b = new_b;
            c = new_c;
            d = new_d;
            e = new_e;
            f = new_f;
            g = new_g;
            h = new_h;
            
            // Каждый раунд - дополнительный взрыв хаоса
            a ^= rotl(b, 19) ^ rotr(c, 13);
            e ^= rotl(f, 23) ^ rotr(g, 7);
        }
        
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);
    }
    
    let mut result = [0u8; 64];
    result[0..8].copy_from_slice(&h0.to_be_bytes());
    result[8..16].copy_from_slice(&h1.to_be_bytes());
    result[16..24].copy_from_slice(&h2.to_be_bytes());
    result[24..32].copy_from_slice(&h3.to_be_bytes());
    result[32..40].copy_from_slice(&h4.to_be_bytes());
    result[40..48].copy_from_slice(&h5.to_be_bytes());
    result[48..56].copy_from_slice(&h6.to_be_bytes());
    result[56..64].copy_from_slice(&h7.to_be_bytes());
    
    result
}

// ========== FFI ДЛЯ ДИНАМИЧЕСКОЙ БИБЛИОТЕКИ ==========
#[unsafe(no_mangle)]
pub extern "C" fn sz1_hash(
    ptr: *const u8,
    len: usize,
    output: *mut u8
) {
    if ptr.is_null() || output.is_null() {
        return;
    }
    
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    let hash = sz1_extreme(data);
    
    unsafe {
        std::ptr::copy_nonoverlapping(hash.as_ptr(), output, 64);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sz1_hash_hex(
    ptr: *const u8,
    len: usize,
    output: *mut u8,
    output_len: usize
) {
    if ptr.is_null() || output.is_null() || output_len < 129 {
        return;
    }
    
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    let hash = sz1_extreme(data);
    
    let hex_str = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let bytes = hex_str.as_bytes();
    
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, 128);
        *output.add(128) = 0; // null-terminator
    }
}

// ========== ТЕСТ ==========
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sz1() {
        let data = b"Hello, World!";
        let hash = sz1_extreme(data);
        
        // Проверяем что хеш не нулевой
        assert_ne!(hash, [0u8; 64]);
        
        // Проверяем лавинный эффект
        let data2 = b"Hello, World?";
        let hash2 = sz1_extreme(data2);
        
        let mut diff = 0;
        for i in 0..64 {
            diff += (hash[i] ^ hash2[i]).count_ones();
        }
        
        println!("Rust SZ-1 разница: {} бит из 512", diff);
        assert!(diff > 200); // Хотя бы 200 бит разницы
    }
}