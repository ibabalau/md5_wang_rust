#![allow(arithmetic_overflow)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::result::Result;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use std::io::{self, Write};
use std::{process, thread, iter};

macro_rules! md5_F
{
    ($x: expr, $y: expr, $z: expr)=> ($z ^ ($x & ($y ^ $z)))
}

macro_rules! md5_G
{
    ($x: expr, $y: expr, $z: expr)=> {md5_F!($z, $x, $y)}
}

macro_rules! md5_H
{
    ($x: expr, $y: expr, $z: expr)=> ($x ^ $y ^ $z)
}

macro_rules! md5_I
{
    ($x: expr, $y: expr, $z: expr)=> ($y ^ ($x | !$z))
}

macro_rules! RL {
    ($x: expr, $y: expr) => {
        ((($x) << ($y)) | (($x) >> (32 - ($y))))
    };
}

macro_rules! RR {
    ($x: expr, $y: expr) => {
        ((($x) >> ($y)) | (($x) << (32 - ($y))))
    };
}

const LOOP_11: u32 = 300;
const LOOP_12: u32 = 0x20000000;
const LOOP_21: u32 = 1000;
const LOOP_22: u32 = 0x40000000;

struct StateS {
    a0: u32,
    b0: u32,
    c0: u32,
    d0: u32,
    a1: u32,
    b1: u32,
    c1: u32,
    d1: u32,
    q0: [u32; 65],
    q1: [u32; 65],
    x0: [u32; 32],
    x1: [u32; 32],
    ct1: i32,
    ct2: u32,
} 

static IV_DEFAULT: [u32; 4] =  [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];


fn block1(iv: [u32; 4], mut ct1: i32, s: &mut StateS) -> i32
{
    println!("BLOCK 1 START");
    let mut rng = rand::thread_rng();
    let mut cnt:i32;
    let mut i:i32;
    println!("{:?} {:?}", rng.gen::<u32>(), rng.gen::<u32>());
    let rand_arr: [u32; 16] = [624373811,
                                2088533051,
                                16313804,
                                1906178774,
                                1444401697,
                                538793990,
                                1513199138,
                                1003925344,
                                1324987654,
                                209079516,
                                1694804360,
                                532383565,
                                599986987,
                                1435564660,
                                492230460,
                                1314485900];

    /* instead of goto block1_again */
    'block1_again: loop
    {
        'loop_1: loop
        {
            /* c1 */
            s.q0[3] = rand_arr[0] & !0x00800040;
            s.q1[3] = s.q0[3];

            /* b1 */
            s.q0[4] = (rand_arr[1] | 0x80080800) & !(0x00800040 | 0x0077f780);
            s.q0[4] |= (s.q0[3] & 0x0077f780);
            s.q1[4] = s.q0[4];

            /* A2 */
            s.q0[5] = (rand_arr[2] | 0x88400025) & !0x02bfffc0;
            s.q1[5] = s.q0[5] - 0x00000040;

            /* D2 */
            s.q0[6] = (rand_arr[3] | 0x027fbc41) & !(0x888043a4 | 0x7500001a);
            s.q0[6] |= (s.q0[5] & 0x7500001a);
            s.q1[6] = s.q0[6] - 0x7f800040;

            /* C2 */
            s.q0[7] = (rand_arr[4] | 0x03fef820) & !0xfc0107df;
            s.q1[7] = s.q0[7] - 0x07800041;

            s.x0[6] = RR!(s.q0[7] - s.q0[6], 17) - md5_F!(s.q0[6], s.q0[5], s.q0[4])
                - s.q0[3] - 0xa8304613;
            s.x1[6] = RR!(s.q1[7] - s.q1[6], 17) - md5_F!(s.q1[6], s.q1[5], s.q1[4])
                - s.q1[3] - 0xa8304613;
            if(s.x0[6] != s.x1[6])
            {
                continue 'loop_1;
            }

            /* B2 */
            s.q0[8] = (rand_arr[5] | 0x01910540) & !0xfe0eaabf;
            s.q1[8] = s.q0[8] - 0x00827fff;
            
            s.x0[7] = RR!(s.q0[8] - s.q0[7], 22) - md5_F!(s.q0[7], s.q0[6], s.q0[5])
                - s.q0[4] - 0xfd469501;
            s.x1[7] = RR!(s.q1[8] - s.q1[7], 22) - md5_F!(s.q1[7], s.q1[6], s.q1[5])
                - s.q1[4] - 0xfd469501;
            if(s.x0[7] != s.x1[7])
            {
                continue 'loop_1;
            }

            /* A3 */
            s.q0[9] = (rand_arr[6] | 0xfb102f3d) & !(0x040f80c2 | 0x00001000);
            s.q0[9] |= (s.q0[8] & 0x00001000);
            s.q1[9] = s.q0[9] - 0x8000003f;

            s.x0[8] = RR!(s.q0[9] - s.q0[8],  7) - md5_F!(s.q0[8], s.q0[7], s.q0[6])
                - s.q0[5] - 0x698098d8;
            s.x1[8] = RR!(s.q1[9] - s.q1[8],  7) - md5_F!(s.q1[8], s.q1[7], s.q1[6])
                - s.q1[5] - 0x698098d8;
            if(s.x0[8] != s.x1[8])
            {
                continue 'loop_1;
            }

            /* D3 */
            s.q0[10] = (rand_arr[7] | 0x401f9040) & !0x80802183;
            s.q1[10] = s.q0[10] - 0x7ffff000;

            s.x0[9] = RR!(s.q0[10] - s.q0[9], 12) - md5_F!(s.q0[9], s.q0[8], s.q0[7])
                - s.q0[6] - 0x8b44f7af;
            s.x1[9] = RR!(s.q1[10] - s.q1[9], 12) - md5_F!(s.q1[9], s.q1[8], s.q1[7])
                - s.q1[6] - 0x8b44f7af;
            if(s.x0[9] != s.x1[9])
            {
                continue 'loop_1;
            }

            /* C3 */
            s.q0[11] = (rand_arr[8] | 0x000180c2) & !(0xc00e3101 | 0x00004000);
            s.q0[11] |= (s.q0[10] & 0x00004000);
            s.q1[11] = s.q0[11] - 0x40000000;

            s.x0[10] = RR!(s.q0[11] - s.q0[10], 17) - md5_F!(s.q0[10], s.q0[9], s.q0[8])
                - s.q0[7] - 0xffff5bb1;
            s.x1[10] = RR!(s.q1[11] - s.q1[10], 17) - md5_F!(s.q1[10], s.q1[9], s.q1[8])
                - s.q1[7] - 0xffff5bb1;
            if(s.x0[10] != s.x1[10])
            {
                continue 'loop_1;
            }

            /* B3 */
            s.q0[12] = (rand_arr[9] | 0x00081100) & !(0xc007e080 | 0x03000000);
            s.q0[12] |= (s.q0[11] & 0x03000000);
            s.q1[12] = s.q0[12] - 0x80002080;
            
            s.x0[11] = RR!(s.q0[12] - s.q0[11], 22) - md5_F!(s.q0[11], s.q0[10], s.q0[9])
                - s.q0[8] - 0x895cd7be;
            s.x1[11] = RR!(s.q1[12] - s.q1[11], 22) - md5_F!(s.q1[11], s.q1[10], s.q1[9])
                - s.q1[8] - 0x895cd7be;
            if((s.x0[11] ^ s.x1[11]) != 0x00008000)
            {
                continue 'loop_1;
            }

            /* A4 */
            s.q0[13] = (rand_arr[10] | 0x410fe008) & !0x82000180;
            s.q1[13] = s.q0[13] - 0x7f000000;

            s.x0[12] = RR!(s.q0[13] - s.q0[12],  7) - md5_F!(s.q0[12], s.q0[11], s.q0[10])
                - s.q0[9] - 0x6b901122;
            s.x1[12] = RR!(s.q1[13] - s.q1[12],  7) - md5_F!(s.q1[12], s.q1[11], s.q1[10])
                - s.q1[9] - 0x6b901122;
            if(s.x0[12] != s.x1[12])
            {
                continue 'loop_1;
            }

            /* D4 */
            s.q0[14] = (rand_arr[11] | 0x000be188) & !0xa3040000;
            s.q1[14] = s.q0[14] - 0x80000000;

            s.x0[13] = RR!(s.q0[14] - s.q0[13], 12) - md5_F!(s.q0[13], s.q0[12], s.q0[11])
                - s.q0[10] - 0xfd987193;
            s.x1[13] = RR!(s.q1[14] - s.q1[13], 12) - md5_F!(s.q1[13], s.q1[12], s.q1[11])
                - s.q1[10] - 0xfd987193;
            if(s.x0[13] != s.x1[13])
            {
                continue 'loop_1;
            }

            /* C4 */
            s.q0[15] = (rand_arr[12] | 0x21008000) & !0x82000008;
            s.q1[15] = s.q0[15] - 0x80007ff8;

            s.x0[14] = RR!(s.q0[15] - s.q0[14], 17) - md5_F!(s.q0[14], s.q0[13], s.q0[12])
                - s.q0[11] - 0xa679438e;
            s.x1[14] = RR!(s.q1[15] - s.q1[14], 17) - md5_F!(s.q1[14], s.q1[13], s.q1[12])
                - s.q1[11] - 0xa679438e;
            if((s.x0[14] ^ s.x1[14]) != 0x80000000)
            {
                continue 'loop_1;
            }


            /* B4 */
            s.q0[16] = (rand_arr[13] | 0x20000000) & !0x80000000;
            s.q1[16] = s.q0[16] - 0xa0000000;

            s.x0[15] = RR!(s.q0[16] - s.q0[15], 22) - md5_F!(s.q0[15], s.q0[14], s.q0[13])
                - s.q0[12] - 0x49b40821;
            s.x1[15] = RR!(s.q1[16] - s.q1[15], 22) - md5_F!(s.q1[15], s.q1[14], s.q1[13])
                - s.q1[12] - 0x49b40821;
            if(s.x0[15] != s.x1[15])
            {
                continue 'loop_1;
            }
            break;
        }
        i = -1;
        'loop_11: loop
        {
            i += 1;
            if i as u32 == LOOP_11
            {
                continue 'block1_again;
            }
            /* A5 */
            s.q0[17] = rand_arr[14] & !(0x80020000 | 0x00008008);
            s.q0[17] |= (s.q0[16] & 0x00008008);
            s.q1[17] = s.q0[17] - 0x80000000;

            s.x0[1] = RR!(s.q0[17] - s.q0[16],  5) - md5_G!(s.q0[16], s.q0[15], s.q0[14])
                - s.q0[13] - 0xf61e2562;
            s.x1[1] = RR!(s.q1[17] - s.q1[16],  5) - md5_G!(s.q1[16], s.q1[15], s.q1[14])
                - s.q1[13] - 0xf61e2562;
            if(s.x0[1] != s.x1[1])
            {
                continue 'loop_11;
            }

            /* D5 */
            s.q0[18] = RL!(md5_G!(s.q0[17], s.q0[16], s.q0[15]) + s.q0[14]
                + s.x0[6] + 0xc040b340,  9) + s.q0[17];
            if((s.q0[18] & 0xa0020000)
                != (0x00020000 | (s.q0[17] & 0x20000000)))
            {
                continue 'loop_11;
            }
            s.q1[18] = RL!(md5_G!(s.q1[17], s.q1[16], s.q1[15]) + s.q1[14]
                + s.x1[6] + 0xc040b340,  9) + s.q1[17];
            if((s.q0[18] ^ s.q1[18]) != 0x80000000)
            {
                continue 'loop_11;
            }
            /* C5 */
            s.q0[19] = RL!(md5_G!(s.q0[18], s.q0[17], s.q0[16]) + s.q0[15]
                + s.x0[11] + 0x265e5a51, 14) + s.q0[18];
            if(s.q0[19] & 0x80020000 != 0)
            {
                continue 'loop_11;
            }
            s.q1[19] = RL!(md5_G!(s.q1[18], s.q1[17], s.q1[16]) + s.q1[15]
                + s.x1[11] + 0x265e5a51, 14) + s.q1[18];
            if(s.q0[19] - s.q1[19] != 0x7ffe0000)
            {
                continue 'loop_11;
            }

            /* B5 */
            s.q0[20] = rand_arr[15] & !0x80000000;
            s.q1[20] = s.q0[20] - 0x80000000;

            s.x0[0] = RR!(s.q0[20] - s.q0[19], 20) - md5_G!(s.q0[19], s.q0[18], s.q0[17])
                - s.q0[16] - 0xe9b6c7aa;
            s.x1[0] = RR!(s.q1[20] - s.q1[19], 20) - md5_G!(s.q1[19], s.q1[18], s.q1[17])
                - s.q1[16] - 0xe9b6c7aa;
            if(s.x0[0] != s.x1[0])
            {
                continue 'loop_11;
            }

            s.q0[1] = RL!(md5_F!(iv[1], iv[2], iv[3]) + iv[0]
                + s.x0[0] + 0xd76aa478,  7) + iv[1];
            s.q1[1] = s.q0[1];

            s.q0[2] = RL!(md5_F!(s.q0[1], iv[1], iv[2]) + iv[3]
                + s.x0[1] + 0xe8c7b756, 12) + s.q0[1];
            s.q1[2] = s.q0[2];

            s.x0[2] = RR!(s.q0[3] - s.q0[2], 17) - md5_F!(s.q0[2], s.q0[1], iv[1])
                - iv[2] - 0x242070db;
            s.x1[2] = s.x0[2];

            s.x0[3] = RR!(s.q0[4] - s.q0[3], 22) - md5_F!(s.q0[3], s.q0[2], s.q0[1])
                - iv[1] - 0xc1bdceee;
            s.x1[3] = s.x0[3];

            s.x0[4] = RR!(s.q0[5] - s.q0[4],  7) - md5_F!(s.q0[4], s.q0[3], s.q0[2])
                - s.q0[1] - 0xf57c0faf;
            s.x1[4] = RR!(s.q1[5] - s.q1[4],  7) - md5_F!(s.q1[4], s.q1[3], s.q1[2])
                - s.q1[1] - 0xf57c0faf;
            if((s.x0[4] ^ s.x1[4]) != 0x80000000)
            {
                continue 'loop_11;
            }

            s.x0[5] = RR!(s.q0[6] - s.q0[5], 12) - md5_F!(s.q0[5], s.q0[4], s.q0[3])
                - s.q0[2] - 0x4787c62a;
            s.x1[5] = RR!(s.q1[6] - s.q1[5], 12) - md5_F!(s.q1[5], s.q1[4], s.q1[3])
                - s.q1[2] - 0x4787c62a;
            if(s.x0[5] != s.x1[5])
            {
                continue 'loop_11;
            }

            /* A6 */
            s.q0[21] = RL!(md5_G!(s.q0[20], s.q0[19], s.q0[18]) + s.q0[17]
                + s.x0[5] + 0xd62f105d,  5) + s.q0[20];
            if((s.q0[21] & 0x80020000) != (s.q0[20] & 0x00020000))
            {
                continue 'loop_11;
            }
            s.q1[21] = RL!(md5_G!(s.q1[20], s.q1[19], s.q1[18]) + s.q1[17]
                + s.x1[5] + 0xd62f105d,  5) + s.q1[20];
            if((s.q0[21] ^ s.q1[21]) != 0x80000000)
            {
                continue 'loop_11;
            }

            /* D6 */
            s.q0[22] = RL!(md5_G!(s.q0[21], s.q0[20], s.q0[19]) + s.q0[18]
                + s.x0[10] + 0x02441453,  9) + s.q0[21];
            if(s.q0[22] & 0x80000000 != 0)
            {
                continue 'loop_11;
            }
            s.q1[22] = RL!(md5_G!(s.q1[21], s.q1[20], s.q1[19]) + s.q1[18]
                + s.x1[10] + 0x02441453,  9) + s.q1[21];
            if((s.q0[22] ^ s.q1[22]) != 0x80000000)
            {
                continue 'loop_11;
            }

            /* C6 */
            s.q0[23] = RL!(md5_G!(s.q0[22], s.q0[21], s.q0[20]) + s.q0[19]
                + s.x0[15] + 0xd8a1e681, 14) + s.q0[22];
            if(s.q0[23] & 0x80000000 != 0)
            {
                continue 'loop_11;
            }
            s.q1[23] = RL!(md5_G!(s.q1[22], s.q1[21], s.q1[20]) + s.q1[19]
                + s.x1[15] + 0xd8a1e681, 14) + s.q1[22];
            if(s.q0[23] != s.q1[23])
            {
                continue 'loop_11;
            }

            /* B6 */
            s.q0[24] = RL!(md5_G!(s.q0[23], s.q0[22], s.q0[21]) + s.q0[20]
                + s.x0[4] + 0xe7d3fbc8, 20) + s.q0[23];
            s.q1[24] = RL!(md5_G!(s.q1[23], s.q1[22], s.q1[21]) + s.q1[20]
                + s.x1[4] + 0xe7d3fbc8, 20) + s.q1[23];
            if(s.q0[24] != s.q1[24])
            {
                continue 'loop_11;
            }

            /* A7 */
            s.q0[25] = RL!(md5_G!(s.q0[24], s.q0[23], s.q0[22]) + s.q0[21]
                + s.x0[9] + 0x21e1cde6,  5) + s.q0[24];
            s.q1[25] = RL!(md5_G!(s.q1[24], s.q1[23], s.q1[22]) + s.q1[21]
                + s.x1[9] + 0x21e1cde6,  5) + s.q1[24];
            if(s.q0[25] != s.q1[25])
            {
                continue 'loop_11;
            }

            /* D7 */
            s.q0[26] = RL!(md5_G!(s.q0[25], s.q0[24], s.q0[23]) + s.q0[22]
                    + s.x0[14] + 0xc33707d6,  9) + s.q0[25];
            s.q1[26] = RL!(md5_G!(s.q1[25], s.q1[24], s.q1[23]) + s.q1[22]
                    + s.x1[14] + 0xc33707d6,  9) + s.q1[25];
            if(s.q0[26] != s.q1[26])
            {
                continue 'loop_11;
            }

            /* C7 */
            s.q0[27] = RL!(md5_G!(s.q0[26], s.q0[25], s.q0[24]) + s.q0[23]
                + s.x0[3] + 0xf4d50d87, 14) + s.q0[26];
            s.q1[27] = RL!(md5_G!(s.q1[26], s.q1[25], s.q1[24]) + s.q1[23]
                + s.x1[3] + 0xf4d50d87, 14) + s.q1[26];
            if(s.q0[27] != s.q1[27])
            {
                continue 'loop_11;
            }

            break;
        }
        println!("DONE");
        ct1 += 1;
        cnt = 0;
        i = -1;
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        let mut cnt3 = 0;
        let mut cnt4 = 0;
        let mut cnt5 = 0;
        let mut cnt6 = 0;
        'loop_12: loop
        {
            i += 1;
            if i as u32 == LOOP_12
            {
                println!("RESTART 1 LOOP");
                continue 'block1_again;
            }
            if (i & 0xfffff) == 0
            {
                //callback2(ct1, (i>>20) as u32);
                println!("{:?}", i);
                //  println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", cnt, cnt1, cnt2, cnt3, cnt4, cnt5, cnt6);
            }
            /* B5 */
            //s.q0[20] ^= (1 << (rng.gen::<u32>() % 31));
            s.q0[20] = 886608102;
            s.q1[20] = s.q0[20] - 0x80000000;

            s.x0[0] = RR!(s.q0[20] - s.q0[19], 20) - md5_G!(s.q0[19], s.q0[18], s.q0[17])
                - s.q0[16] - 0xe9b6c7aa;
            s.x1[0] = RR!(s.q1[20] - s.q1[19], 20) - md5_G!(s.q1[19], s.q1[18], s.q1[17])
                - s.q1[16] - 0xe9b6c7aa;
            if(s.x0[0] != s.x1[0])
            {
                continue 'loop_12;
            }

            s.q0[1] = RL!(md5_F!(iv[1], iv[2], iv[3]) + iv[0]
                + s.x0[0] + 0xd76aa478,  7) + iv[1];
            s.q1[1] = s.q0[1];

            s.q0[2] = RL!(md5_F!(s.q0[1], iv[1], iv[2]) + iv[3]
                + s.x0[1] + 0xe8c7b756, 12) + s.q0[1];
            s.q1[2] = s.q0[2];
            s.x0[2] = RR!(s.q0[3] - s.q0[2], 17) - md5_F!(s.q0[2], s.q0[1], iv[1])
                - iv[2] - 0x242070db;
            s.x1[2] = s.x0[2];

            s.x0[3] = RR!(s.q0[4] - s.q0[3], 22) - md5_F!(s.q0[3], s.q0[2], s.q0[1])
                - iv[1] - 0xc1bdceee;
            s.x1[3] = s.x0[3];

            s.x0[4] = RR!(s.q0[5] - s.q0[4],  7) - md5_F!(s.q0[4], s.q0[3], s.q0[2])
                - s.q0[1] - 0xf57c0faf;
            s.x1[4] = RR!(s.q1[5] - s.q1[4],  7) - md5_F!(s.q1[4], s.q1[3], s.q1[2])
                - s.q1[1] - 0xf57c0faf;
            if((s.x0[4] ^ s.x1[4]) != 0x80000000)
            {
                continue 'loop_12;
            }

            s.x0[5] = RR!(s.q0[6] - s.q0[5], 12) - md5_F!(s.q0[5], s.q0[4], s.q0[3])
                - s.q0[2] - 0x4787c62a;
            s.x1[5] = RR!(s.q1[6] - s.q1[5], 12) - md5_F!(s.q1[5], s.q1[4], s.q1[3])
                - s.q1[2] - 0x4787c62a;
            if(s.x0[5] != s.x1[5])
            {
                continue 'loop_12;
            }

            /* A6 */
            s.q0[21] = RL!(md5_G!(s.q0[20], s.q0[19], s.q0[18]) + s.q0[17]
                + s.x0[5] + 0xd62f105d,  5) + s.q0[20];
            if((s.q0[21] & 0x80020000) != (s.q0[20] & 0x00020000))
            {
                continue 'loop_12;
            }
            s.q1[21] = RL!(md5_G!(s.q1[20], s.q1[19], s.q1[18]) + s.q1[17]
                + s.x1[5] + 0xd62f105d,  5) + s.q1[20];
            if((s.q0[21] ^ s.q1[21]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D6 */
            s.q0[22] = RL!(md5_G!(s.q0[21], s.q0[20], s.q0[19]) + s.q0[18]
                + s.x0[10] + 0x02441453,  9) + s.q0[21];
            if(s.q0[22] & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[22] = RL!(md5_G!(s.q1[21], s.q1[20], s.q1[19]) + s.q1[18]
                + s.x1[10] + 0x02441453,  9) + s.q1[21];
            if((s.q0[22] ^ s.q1[22]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C6 */
            s.q0[23] = RL!(md5_G!(s.q0[22], s.q0[21], s.q0[20]) + s.q0[19]
                + s.x0[15] + 0xd8a1e681, 14) + s.q0[22];
            if(s.q0[23] & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[23] = RL!(md5_G!(s.q1[22], s.q1[21], s.q1[20]) + s.q1[19]
                + s.x1[15] + 0xd8a1e681, 14) + s.q1[22];
            if(s.q0[23] != s.q1[23])
            {
                continue 'loop_12;
            }

            /* B6 */
            s.q0[24] = RL!(md5_G!(s.q0[23], s.q0[22], s.q0[21]) + s.q0[20]
                + s.x0[4] + 0xe7d3fbc8, 20) + s.q0[23];
            s.q1[24] = RL!(md5_G!(s.q1[23], s.q1[22], s.q1[21]) + s.q1[20]
                + s.x1[4] + 0xe7d3fbc8, 20) + s.q1[23];
            if(s.q0[24] != s.q1[24])
            {
                continue 'loop_12;
            }

            /* A7 */
            s.q0[25] = RL!(md5_G!(s.q0[24], s.q0[23], s.q0[22]) + s.q0[21]
                + s.x0[9] + 0x21e1cde6,  5) + s.q0[24];
            s.q1[25] = RL!(md5_G!(s.q1[24], s.q1[23], s.q1[22]) + s.q1[21]
                + s.x1[9] + 0x21e1cde6,  5) + s.q1[24];
            if(s.q0[25] != s.q1[25])
            {
                continue 'loop_12;
            }

            /* D7 */
            s.q0[26] = RL!(md5_G!(s.q0[25], s.q0[24], s.q0[23]) + s.q0[22]
                + s.x0[14] + 0xc33707d6,  9) + s.q0[25];
            s.q1[26] = RL!(md5_G!(s.q1[25], s.q1[24], s.q1[23]) + s.q1[22]
                + s.x1[14] + 0xc33707d6,  9) + s.q1[25];
            if(s.q0[26] != s.q1[26])
            {
                continue 'loop_12;
            }

            /* C7 */
            s.q0[27] = RL!(md5_G!(s.q0[26], s.q0[25], s.q0[24]) + s.q0[23]
                + s.x0[3] + 0xf4d50d87, 14) + s.q0[26];
            s.q1[27] = RL!(md5_G!(s.q1[26], s.q1[25], s.q1[24]) + s.q1[23]
                + s.x1[3] + 0xf4d50d87, 14) + s.q1[26];
            if(s.q0[27] != s.q1[27])
            {
                continue 'loop_12;
            }

            /* B7 */
            s.q0[28] = RL!(md5_G!(s.q0[27], s.q0[26], s.q0[25]) + s.q0[24]
                + s.x0[8] + 0x455a14ed, 20) + s.q0[27];
            s.q1[28] = RL!(md5_G!(s.q1[27], s.q1[26], s.q1[25]) + s.q1[24]
                + s.x1[8] + 0x455a14ed, 20) + s.q1[27];
            if(s.q0[28] != s.q1[28])
            {
                continue 'loop_12;
            }

            /* A8 */
            s.q0[29] = RL!(md5_G!(s.q0[28], s.q0[27], s.q0[26]) + s.q0[25]
                + s.x0[13] + 0xa9e3e905,  5) + s.q0[28];
            s.q1[29] = RL!(md5_G!(s.q1[28], s.q1[27], s.q1[26]) + s.q1[25]
                + s.x1[13] + 0xa9e3e905,  5) + s.q1[28];
            if(s.q0[29] != s.q1[29])
            {
                continue 'loop_12;
            }

            /* D8 */
            s.q0[30] = RL!(md5_G!(s.q0[29], s.q0[28], s.q0[27]) + s.q0[26]
                + s.x0[2] + 0xfcefa3f8,  9) + s.q0[29];
            s.q1[30] = RL!(md5_G!(s.q1[29], s.q1[28], s.q1[27]) + s.q1[26]
                + s.x1[2] + 0xfcefa3f8,  9) + s.q1[29];
            if(s.q0[30] != s.q1[30])
            {
                continue 'loop_12;
            }

            /* C8 */
            s.q0[31] = RL!(md5_G!(s.q0[30], s.q0[29], s.q0[28]) + s.q0[27]
                + s.x0[7] + 0x676f02d9, 14) + s.q0[30];
            s.q1[31] = RL!(md5_G!(s.q1[30], s.q1[29], s.q1[28]) + s.q1[27]
                + s.x1[7] + 0x676f02d9, 14) + s.q1[30];
            if(s.q0[31] != s.q1[31])
            {
                continue 'loop_12;
            }

            /* B8 */
            s.q0[32] = RL!(md5_G!(s.q0[31], s.q0[30], s.q0[29]) + s.q0[28]
                + s.x0[12] + 0x8d2a4c8a, 20) + s.q0[31];
            s.q1[32] = RL!(md5_G!(s.q1[31], s.q1[30], s.q1[29]) + s.q1[28]
                + s.x1[12] + 0x8d2a4c8a, 20) + s.q1[31];
            if(s.q0[32] != s.q1[32])
            {
                continue 'loop_12;
            }

            /* A9 */
            s.q0[33] = RL!(md5_H!(s.q0[32], s.q0[31], s.q0[30]) + s.q0[29]
                + s.x0[5] + 0xfffa3942,  4) + s.q0[32];
            s.q1[33] = RL!(md5_H!(s.q1[32], s.q1[31], s.q1[30]) + s.q1[29]
                + s.x1[5] + 0xfffa3942,  4) + s.q1[32];
            if(s.q0[33] != s.q1[33])
            {
                continue 'loop_12;
            }

            /* D9 */
            s.q0[34] = RL!(md5_H!(s.q0[33], s.q0[32], s.q0[31]) + s.q0[30]
                + s.x0[8] + 0x8771f681, 11) + s.q0[33];
            s.q1[34] = RL!(md5_H!(s.q1[33], s.q1[32], s.q1[31]) + s.q1[30]
                + s.x1[8] + 0x8771f681, 11) + s.q1[33];
            if(s.q0[34] != s.q1[34])
            {
                continue 'loop_12;
            }

            /* C9 */
            s.q0[35] = RL!(md5_H!(s.q0[34], s.q0[33], s.q0[32]) + s.q0[31]
                + s.x0[11] + 0x6d9d6122, 16) + s.q0[34];
            s.q1[35] = RL!(md5_H!(s.q1[34], s.q1[33], s.q1[32]) + s.q1[31]
                + s.x1[11] + 0x6d9d6122, 16) + s.q1[34];
            if((s.q0[35] ^ s.q1[35]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B9 */
            s.q0[36] = RL!(md5_H!(s.q0[35], s.q0[34], s.q0[33]) + s.q0[32]
                + s.x0[14] + 0xfde5380c, 23) + s.q0[35];
            s.q1[36] = RL!(md5_H!(s.q1[35], s.q1[34], s.q1[33]) + s.q1[32]
                + s.x1[14] + 0xfde5380c, 23) + s.q1[35];
            if((s.q0[36] ^ s.q1[36]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A10 */
            s.q0[37] = RL!(md5_H!(s.q0[36], s.q0[35], s.q0[34]) + s.q0[33]
                + s.x0[1] + 0xa4beea44,  4) + s.q0[36];
            s.q1[37] = RL!(md5_H!(s.q1[36], s.q1[35], s.q1[34]) + s.q1[33]
                + s.x1[1] + 0xa4beea44,  4) + s.q1[36];
            if((s.q0[37] ^ s.q1[37]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D10 */
            s.q0[38] = RL!(md5_H!(s.q0[37], s.q0[36], s.q0[35]) + s.q0[34]
                + s.x0[4] + 0x4bdecfa9, 11) + s.q0[37];
            s.q1[38] = RL!(md5_H!(s.q1[37], s.q1[36], s.q1[35]) + s.q1[34]
                + s.x1[4] + 0x4bdecfa9, 11) + s.q1[37];
            if((s.q0[38] ^ s.q1[38]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C10 */
            s.q0[39] = RL!(md5_H!(s.q0[38], s.q0[37], s.q0[36]) + s.q0[35]
                + s.x0[7] + 0xf6bb4b60, 16) + s.q0[38];
            s.q1[39] = RL!(md5_H!(s.q1[38], s.q1[37], s.q1[36]) + s.q1[35]
                + s.x1[7] + 0xf6bb4b60, 16) + s.q1[38];
            if((s.q0[39] ^ s.q1[39]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B10 */
            s.q0[40] = RL!(md5_H!(s.q0[39], s.q0[38], s.q0[37]) + s.q0[36]
                + s.x0[10] + 0xbebfbc70, 23) + s.q0[39];
            s.q1[40] = RL!(md5_H!(s.q1[39], s.q1[38], s.q1[37]) + s.q1[36]
                + s.x1[10] + 0xbebfbc70, 23) + s.q1[39];
            if((s.q0[40] ^ s.q1[40]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A11 */
            s.q0[41] = RL!(md5_H!(s.q0[40], s.q0[39], s.q0[38]) + s.q0[37]
                + s.x0[13] + 0x289b7ec6,  4) + s.q0[40];
            s.q1[41] = RL!(md5_H!(s.q1[40], s.q1[39], s.q1[38]) + s.q1[37]
                + s.x1[13] + 0x289b7ec6,  4) + s.q1[40];
            if((s.q0[41] ^ s.q1[41]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D11 */
            s.q0[42] = RL!(md5_H!(s.q0[41], s.q0[40], s.q0[39]) + s.q0[38]
                + s.x0[0] + 0xeaa127fa, 11) + s.q0[41];
            s.q1[42] = RL!(md5_H!(s.q1[41], s.q1[40], s.q1[39]) + s.q1[38]
                + s.x1[0] + 0xeaa127fa, 11) + s.q1[41];
            if((s.q0[42] ^ s.q1[42]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C11 */
            s.q0[43] = RL!(md5_H!(s.q0[42], s.q0[41], s.q0[40]) + s.q0[39]
                + s.x0[3] + 0xd4ef3085, 16) + s.q0[42];
            s.q1[43] = RL!(md5_H!(s.q1[42], s.q1[41], s.q1[40]) + s.q1[39]
                + s.x1[3] + 0xd4ef3085, 16) + s.q1[42];
            if((s.q0[43] ^ s.q1[43]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B11 */
            s.q0[44] = RL!(md5_H!(s.q0[43], s.q0[42], s.q0[41]) + s.q0[40]
                + s.x0[6] + 0x04881d05, 23) + s.q0[43];
            s.q1[44] = RL!(md5_H!(s.q1[43], s.q1[42], s.q1[41]) + s.q1[40]
                + s.x1[6] + 0x04881d05, 23) + s.q1[43];
            if((s.q0[44] ^ s.q1[44]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A12 */
            s.q0[45] = RL!(md5_H!(s.q0[44], s.q0[43], s.q0[42]) + s.q0[41]
                + s.x0[9] + 0xd9d4d039,  4) + s.q0[44];
            s.q1[45] = RL!(md5_H!(s.q1[44], s.q1[43], s.q1[42]) + s.q1[41]
                + s.x1[9] + 0xd9d4d039,  4) + s.q1[44];
            if((s.q0[45] ^ s.q1[45]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D12 */
            s.q0[46] = RL!(md5_H!(s.q0[45], s.q0[44], s.q0[43]) + s.q0[42]
                + s.x0[12] + 0xe6db99e5, 11) + s.q0[45];
            s.q1[46] = RL!(md5_H!(s.q1[45], s.q1[44], s.q1[43]) + s.q1[42]
                + s.x1[12] + 0xe6db99e5, 11) + s.q1[45];
            if((s.q0[46] ^ s.q1[46]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C12 */
            s.q0[47] = RL!(md5_H!(s.q0[46], s.q0[45], s.q0[44]) + s.q0[43]
                + s.x0[15] + 0x1fa27cf8, 16) + s.q0[46];
            s.q1[47] = RL!(md5_H!(s.q1[46], s.q1[45], s.q1[44]) + s.q1[43]
                + s.x1[15] + 0x1fa27cf8, 16) + s.q1[46];
            if((s.q0[47] ^ s.q1[47]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B12 */
            s.q0[48] = RL!(md5_H!(s.q0[47], s.q0[46], s.q0[45]) + s.q0[44]
                + s.x0[2] + 0xc4ac5665, 23) + s.q0[47];
            if((s.q0[48] ^ s.q0[46]) & 0x80000000  != 0)
            {
                continue 'loop_12;
            }
            s.q1[48] = RL!(md5_H!(s.q1[47], s.q1[46], s.q1[45]) + s.q1[44]
                + s.x1[2] + 0xc4ac5665, 23) + s.q1[47];
            if((s.q0[48] ^ s.q1[48]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A13 */
            s.q0[49] = RL!(md5_I!(s.q0[48], s.q0[47], s.q0[46]) + s.q0[45]
                + s.x0[0] + 0xf4292244,  6) + s.q0[48];
            if((s.q0[49] ^ s.q0[47]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[49] = RL!(md5_I!(s.q1[48], s.q1[47], s.q1[46]) + s.q1[45]
                + s.x1[0] + 0xf4292244,  6) + s.q1[48];
            if((s.q0[49] ^ s.q1[49]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D13 */
            s.q0[50] = RL!(md5_I!(s.q0[49], s.q0[48], s.q0[47]) + s.q0[46]
                + s.x0[7] + 0x432aff97, 10) + s.q0[49];
            if(!((s.q0[50] ^ s.q0[48]) & 0x80000000) != 0)
            {
                continue 'loop_12;
            }
            s.q1[50] = RL!(md5_I!(s.q1[49], s.q1[48], s.q1[47]) + s.q1[46]
                + s.x1[7] + 0x432aff97, 10) + s.q1[49];
            if((s.q0[50] ^ s.q1[50]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C13 */
            s.q0[51] = RL!(md5_I!(s.q0[50], s.q0[49], s.q0[48]) + s.q0[47]
                + s.x0[14] + 0xab9423a7, 15) + s.q0[50];
            if((s.q0[51] ^ s.q0[49]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[51] = RL!(md5_I!(s.q1[50], s.q1[49], s.q1[48]) + s.q1[47]
                + s.x1[14] + 0xab9423a7, 15) + s.q1[50];
            if((s.q0[51] ^ s.q1[51]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B13 */
            s.q0[52] = RL!(md5_I!(s.q0[51], s.q0[50], s.q0[49]) + s.q0[48]
                + s.x0[5] + 0xfc93a039, 21) + s.q0[51];
            if((s.q0[52] ^ s.q0[50]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[52] = RL!(md5_I!(s.q1[51], s.q1[50], s.q1[49]) + s.q1[48]
                + s.x1[5] + 0xfc93a039, 21) + s.q1[51];
            if((s.q0[52] ^ s.q1[52]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A14 */
            s.q0[53] = RL!(md5_I!(s.q0[52], s.q0[51], s.q0[50]) + s.q0[49]
                + s.x0[12] + 0x655b59c3,  6) + s.q0[52];
            if((s.q0[53] ^ s.q0[51]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[53] = RL!(md5_I!(s.q1[52], s.q1[51], s.q1[50]) + s.q1[49]
                + s.x1[12] + 0x655b59c3,  6) + s.q1[52];
            if((s.q0[53] ^ s.q1[53]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D14 */
            s.q0[54] = RL!(md5_I!(s.q0[53], s.q0[52], s.q0[51]) + s.q0[50]
                + s.x0[3] + 0x8f0ccc92, 10) + s.q0[53];
            if((s.q0[54] ^ s.q0[52]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[54] = RL!(md5_I!(s.q1[53], s.q1[52], s.q1[51]) + s.q1[50]
                + s.x1[3] + 0x8f0ccc92, 10) + s.q1[53];
            if((s.q0[54] ^ s.q1[54]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C14 */
            s.q0[55] = RL!(md5_I!(s.q0[54], s.q0[53], s.q0[52]) + s.q0[51]
                + s.x0[10] + 0xffeff47d, 15) + s.q0[54];
            if((s.q0[55] ^ s.q0[53]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[55] = RL!(md5_I!(s.q1[54], s.q1[53], s.q1[52]) + s.q1[51]
                + s.x1[10] + 0xffeff47d, 15) + s.q1[54];
            if((s.q0[55] ^ s.q1[55]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B14 */
            s.q0[56] = RL!(md5_I!(s.q0[55], s.q0[54], s.q0[53]) + s.q0[52]
                + s.x0[1] + 0x85845dd1, 21) + s.q0[55];
            if((s.q0[56] ^ s.q0[54]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[56] = RL!(md5_I!(s.q1[55], s.q1[54], s.q1[53]) + s.q1[52]
                + s.x1[1] + 0x85845dd1, 21) + s.q1[55];
            if((s.q0[56] ^ s.q1[56]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A15 */
            s.q0[57] = RL!(md5_I!(s.q0[56], s.q0[55], s.q0[54]) + s.q0[53]
                + s.x0[8] + 0x6fa87e4f,  6) + s.q0[56];
            if((s.q0[57] ^ s.q0[55]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[57] = RL!(md5_I!(s.q1[56], s.q1[55], s.q1[54]) + s.q1[53]
                + s.x1[8] + 0x6fa87e4f,  6) + s.q1[56];
            if((s.q0[57] ^ s.q1[57]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D15 */
            s.q0[58] = RL!(md5_I!(s.q0[57], s.q0[56], s.q0[55]) + s.q0[54]
                + s.x0[15] + 0xfe2ce6e0, 10) + s.q0[57];
            if((s.q0[58] ^ s.q0[56]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[58] = RL!(md5_I!(s.q1[57], s.q1[56], s.q1[55]) + s.q1[54]
                + s.x1[15] + 0xfe2ce6e0, 10) + s.q1[57];
            if((s.q0[58] ^ s.q1[58]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* C15 */
            s.q0[59] = RL!(md5_I!(s.q0[58], s.q0[57], s.q0[56]) + s.q0[55]
                + s.x0[6] + 0xa3014314, 15) + s.q0[58];
            if((s.q0[59] ^ s.q0[57]) & 0x80000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[59] = RL!(md5_I!(s.q1[58], s.q1[57], s.q1[56]) + s.q1[55]
                + s.x1[6] + 0xa3014314, 15) + s.q1[58];
            if((s.q0[59] ^ s.q1[59]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* B15 */
            s.q0[60] = RL!(md5_I!(s.q0[59], s.q0[58], s.q0[57]) + s.q0[56]
                + s.x0[13] + 0x4e0811a1, 21) + s.q0[59];
            if(s.q0[60] & 0x02000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[60] = RL!(md5_I!(s.q1[59], s.q1[58], s.q1[57]) + s.q1[56]
                + s.x1[13] + 0x4e0811a1, 21) + s.q1[59];
            if((s.q0[60] ^ s.q1[60]) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* A16 */
            s.q0[61] = RL!(md5_I!(s.q0[60], s.q0[59], s.q0[58]) + s.q0[57]
                + s.x0[4] + 0xf7537e82,  6) + s.q0[60];
            s.a0 = iv[0] + s.q0[61];
            s.q1[61] = RL!(md5_I!(s.q1[60], s.q1[59], s.q1[58]) + s.q1[57]
                + s.x1[4] + 0xf7537e82,  6) + s.q1[60];
            s.a1 = iv[0] + s.q1[61];
            if((s.a0 ^ s.a1) != 0x80000000)
            {
                continue 'loop_12;
            }

            /* D16 */
            s.q0[62] = RL!(md5_I!(s.q0[61], s.q0[60], s.q0[59]) + s.q0[58]
                + s.x0[11] + 0xbd3af235, 10) + s.q0[61];
            s.d0 = iv[3] + s.q0[62];
            if(s.d0 & 0x02000000 != 0)
            {
                continue 'loop_12;
            }
            s.q1[62] = RL!(md5_I!(s.q1[61], s.q1[60], s.q1[59]) + s.q1[58]
                + s.x1[11] + 0xbd3af235, 10) + s.q1[61];
            s.d1 = iv[3] + s.q1[62];
            if((s.d0 - s.d1) != 0x7e000000)
            {
                continue 'loop_12;
            }

            /* C16 */
            s.q0[63] = RL!(md5_I!(s.q0[62], s.q0[61], s.q0[60]) + s.q0[59]
                + s.x0[2] + 0x2ad7d2bb, 15) + s.q0[62];
            s.c0 = iv[2] + s.q0[63];
            if((s.c0 & 0x86000000) != ((s.d0 & 0x80000000) | 0x02000000))
            {
                continue 'loop_12;
            }
            s.q1[63] = RL!(md5_I!(s.q1[62], s.q1[61], s.q1[60]) + s.q1[59]
                + s.x1[2] + 0x2ad7d2bb, 15) + s.q1[62];
            s.c1 = iv[2] + s.q1[63];
            if((s.c0 - s.c1) != 0x7e000000)
            {
                continue 'loop_12;
            }

            /* B16 */
            s.q0[64] = RL!(md5_I!(s.q0[63], s.q0[62], s.q0[61]) + s.q0[60]
                + s.x0[9] + 0xeb86d391, 21) + s.q0[63];
            s.b0 = iv[1] + s.q0[64];
            if((s.b0 & 0x86000020) != (s.c0 & 0x80000000))
            {
                continue 'loop_12;
            }
            s.q1[64] = RL!(md5_I!(s.q1[63], s.q1[62], s.q1[61]) + s.q1[60]
                + s.x1[9] + 0xeb86d391, 21) + s.q1[63];
            s.b1 = iv[1] + s.q1[64];
            if((s.b0 - s.b1) != 0x7e000000)
            {
                continue 'loop_12;
            }
        
            break;
        }
        s.ct1 = ct1;
        s.ct2 = (cnt >> 20) as u32;
        println!("BLOCK 1 FINISH");
        /*
        for i in 0..65
        {
            println!("q0 {:?} {:?}", i, s.q0[i]);
            println!("q1 {:?} {:?}", i, s.q1[i]);
        }
        for i in 0..32
        {
            println!("x0 {:?} {:?}", i, s.x0[i]);
            println!("x1 {:?} {:?}", i, s.x1[i]);
        }
        println!("{:?} {:?} {:?} {:?}", s.a0, s.b0, s.c0, s.d0);
        println!("{:?} {:?} {:?} {:?}", s.a1, s.b1, s.c1, s.d1);
        process::exit(0);
        */
        return 0; 
    }
}

const mask22:[u32; 30] = [
    0x00000001, 0x00000002, 0x00000004, 0x00000008,
    0x00000010, 0x00000020, 0x00000040, 0x00000080,
    0x00000100, 0x00000200, 0x00000400, 0x00000800, 
    0x00001000, 0x00002000, 0x00004000, 0x00008000, 
    0x00010000, 0x00020000, 0x00040000, 0x00080000,
    0x00100000, 0x00200000, 0x00400000, 0x00800000,
    0x01000000, 0x02000000, 0x04000000, 0x08000000,
    0x10000000, 0x40000000
];

fn block2(s: &mut StateS) -> i32
{
    println!("BLOCK 2 START");
    let mut rng = rand::thread_rng();
    let mut ct3: i32 = 0;
    let mut it: i32 = 0;
    let mut i: i32;

    /* block2_again */
    'block2_again: loop
    {
        loop
        {
            /* a1 */
            s.q0[1] = (rng.gen::<u32>() | 0x84200000) & !0x0a000820;
            s.q1[1] = s.q0[1] - 0x7e000000;
            s.x0[16] = RR!(s.q0[1] - s.b0,  7) - md5_F!(s.b0, s.c0, s.d0) - s.a0 - 0xd76aa478;
            s.x1[16] = RR!(s.q1[1] - s.b1,  7) - md5_F!(s.b1, s.c1, s.d1) - s.a1 - 0xd76aa478;
            if s.x0[16] != s.x1[16]
            {
                continue;
            }
            break;
        }

        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* d1 */
            s.q0[2] = (rng.gen::<u32>() | 0x8c000800) & !(0x02208026 | 0x701f10c0);
            s.q0[2] |= s.q0[1] & 0x701f10c0;
            s.q1[2] = s.q0[2] - 0x7dffffe0;
    
            s.x0[17] = RR!(s.q0[2] - s.q0[1], 12) - md5_F!(s.q0[1], s.b0, s.c0)
                    - s.d0 - 0xe8c7b756;
            s.x1[17] = RR!(s.q1[2] - s.q1[1], 12) - md5_F!(s.q1[1], s.b1, s.c1)
                    - s.d1 - 0xe8c7b756;
            if s.x0[17] != s.x1[17]
            {
                continue;
            }
            break;
        }
        
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                if ct3 == 0
                {
                    /* sometimes block1() returns a state that
                    never gets past this point, causing
                    block2() to hang forever. Try to detect
                    this and fail (emergency exit). One example
                    where this happens is the initial vector
                    0x874587a2 0xf09dfbdf 0x17732fb1 0x9299e527
                    with random seed 2. */
                    it += 1;
                    if it >= 10000
                    {
                        return -1;
                    }
                }
                continue 'block2_again;
            }
            /* c1 */
            s.q0[3] = (rng.gen::<u32>() | 0xbe1f0966) & !(0x40201080 | 0x00000018);
            s.q0[3] |= s.q0[2] & 0x00000018;
            s.q1[3] = s.q0[3] - 0x7dfef7e0;
    
            s.x0[18] = RR!(s.q0[3] - s.q0[2], 17) - md5_F!(s.q0[2], s.q0[1], s.b0)
                    - s.c0 - 0x242070db;
            s.x1[18] = RR!(s.q1[3] - s.q1[2], 17) - md5_F!(s.q1[2], s.q1[1], s.b1)
                    - s.c1 - 0x242070db;
            if s.x0[18] != s.x1[18]
            {
                continue;
            }
            break;
        }
        
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* b1 */
            s.q0[4] = (rng.gen::<u32>() | 0xba040010) & !(0x443b19ee | 0x00000601);
            s.q0[4] |= s.q0[3] & 0x00000601;
            s.q1[4] = s.q0[4] - 0x7dffffe2;
    
            s.x0[19] = RR!(s.q0[4] - s.q0[3], 22) - md5_F!(s.q0[3], s.q0[2], s.q0[1])
                    - s.b0 - 0xc1bdceee;
            s.x1[19] = RR!(s.q1[4] - s.q1[3], 22) - md5_F!(s.q1[3], s.q1[2], s.q1[1])
                    - s.b1 - 0xc1bdceee;
            if s.x0[19] != s.x1[19]
            {
                continue;
            }
            break;
        }
        
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* A2 */
            s.q0[5] = (rng.gen::<u32>() | 0x482f0e50) & !0xb41011af;
            s.q1[5] = s.q0[5] - 0x7ffffcbf;
    
            s.x0[20] = RR!(s.q0[5] - s.q0[4],  7) - md5_F!(s.q0[4], s.q0[3], s.q0[2])
                    - s.q0[1] - 0xf57c0faf;
            s.x1[20] = RR!(s.q1[5] - s.q1[4],  7) - md5_F!(s.q1[4], s.q1[3], s.q1[2])
                    - s.q1[1] - 0xf57c0faf;
            if (s.x0[20] ^ s.x1[20]) != 0x80000000
            {
                continue;
            }
            break;
        }
    
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* D2 */
            s.q0[6] = (rng.gen::<u32>() | 0x04220c56) & !0x9a1113a9;
            s.q1[6] = s.q0[6] - 0x80110000;
    
            s.x0[21] = RR!(s.q0[6] - s.q0[5], 12) - md5_F!(s.q0[5], s.q0[4], s.q0[3])
                    - s.q0[2] - 0x4787c62a;
            s.x1[21] = RR!(s.q1[6] - s.q1[5], 12) - md5_F!(s.q1[5], s.q1[4], s.q1[3])
                    - s.q1[2] - 0x4787c62a;
            if s.x0[21] != s.x1[21]
            {
                continue;
            }
            break;
        }

        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* C2 */
            s.q0[7] = (rng.gen::<u32>() | 0x96011e01) & !(0x083201c0 | 0x01808000);
            s.q0[7] |= s.q0[6] & 0x01808000;
            s.q1[7] = s.q0[7] - 0x88000040;
    
            s.x0[22] = RR!(s.q0[7] - s.q0[6], 17) - md5_F!(s.q0[6], s.q0[5], s.q0[4])
                - s.q0[3] - 0xa8304613;
            s.x1[22] = RR!(s.q1[7] - s.q1[6], 17) - md5_F!(s.q1[6], s.q1[5], s.q1[4])
                - s.q1[3] - 0xa8304613;
            if s.x0[22] != s.x1[22]
            {
                continue;
            }
            break;
        }
    
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* B2 */
            s.q0[8] = (rng.gen::<u32>() | 0x843283c0) & !(0x1b810001 | 0x00000002);
            s.q0[8] |= s.q0[7] & 0x00000002;
            s.q1[8] = s.q0[8] - 0x80818000;
    
            s.x0[23] = RR!(s.q0[8] - s.q0[7], 22) - md5_F!(s.q0[7], s.q0[6], s.q0[5])
                - s.q0[4] - 0xfd469501;
            s.x1[23] = RR!(s.q1[8] - s.q1[7], 22) - md5_F!(s.q1[7], s.q1[6], s.q1[5])
                - s.q1[4] - 0xfd469501;
            if s.x0[23] != s.x1[23]
            {
                continue;
            }
            break;
        }
        
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* A3 */
            s.q0[9] = (rng.gen::<u32>() | 0x9c0101c1) & !(0x03828202 | 0x00001000);
            s.q0[9] |= s.q0[8] & 0x00001000;
            s.q1[9] = s.q0[9] - 0x7fffffbf;
    
            s.x0[24] = RR!(s.q0[9] - s.q0[8],  7) - md5_F!(s.q0[8], s.q0[7], s.q0[6])
                - s.q0[5] - 0x698098d8;
            s.x1[24] = RR!(s.q1[9] - s.q1[8],  7) - md5_F!(s.q1[8], s.q1[7], s.q1[6])
                - s.q1[5] - 0x698098d8;
            if s.x0[24] != s.x1[24]
            {
                continue;
            }
            break;
        }
    
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* D3 */
            s.q0[10] = (rng.gen::<u32>() | 0x878383c0) & !0x00041003;
            s.q1[10] = s.q0[10] - 0x7ffff000;
    
            s.x0[25] = RR!(s.q0[10] - s.q0[9], 12) - md5_F!(s.q0[9], s.q0[8], s.q0[7])
                - s.q0[6] - 0x8b44f7af;
            s.x1[25] = RR!(s.q1[10] - s.q1[9], 12) - md5_F!(s.q1[9], s.q1[8], s.q1[7])
                - s.q1[6] - 0x8b44f7af;
            if s.x0[25] != s.x1[25]
            {
                continue;
            }
            break;
        }
    
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* C3 */
            s.q0[11] = (rng.gen::<u32>() | 0x800583c3) & !(0x00021000 | 0x00086000);
            s.q0[11] |= s.q0[10] & 0x00086000;
            s.q1[11] = s.q0[11] - 0x80000000;
    
            s.x0[26] = RR!(s.q0[11] - s.q0[10], 17) - md5_F!(s.q0[10], s.q0[9], s.q0[8])
                - s.q0[7] - 0xffff5bb1;
            s.x1[26] = RR!(s.q1[11] - s.q1[10], 17) - md5_F!(s.q1[10], s.q1[9], s.q1[8])
                - s.q1[7] - 0xffff5bb1;
            if s.x0[26] != s.x1[26]
            {
                continue;
            }
            break;
        }
    
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* B3 */
            s.q0[12] = (rng.gen::<u32>() | 0x80081080) & !(0x0007e000 | 0x7f000000);
            s.q0[12] |= s.q0[11] & 0x7f000000;
            s.q1[12] = s.q0[12] - 0x80002080;
    
            s.x0[27] = RR!(s.q0[12] - s.q0[11], 22) - md5_F!(s.q0[11], s.q0[10], s.q0[9])
                - s.q0[8] - 0x895cd7be;
            s.x1[27] = RR!(s.q1[12] - s.q1[11], 22) - md5_F!(s.q1[11], s.q1[10], s.q1[9])
                - s.q1[8] - 0x895cd7be;
            if (s.x0[27] ^ s.x1[27]) != 0x00008000
            {
                continue;
            }
            break;
        }
        
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* A4 */
            s.q0[13] = (rng.gen::<u32>() | 0x3f0fe008) & !0x80000080;
            s.q1[13] = s.q0[13] - 0x7f000000;
    
            s.x0[28] = RR!(s.q0[13] - s.q0[12],  7) - md5_F!(s.q0[12], s.q0[11], s.q0[10])
                - s.q0[9] - 0x6b901122;
            s.x1[28] = RR!(s.q1[13] - s.q1[12],  7) - md5_F!(s.q1[12], s.q1[11], s.q1[10])
                - s.q1[9] - 0x6b901122;
            if s.x0[28] != s.x1[28]
            {
                continue;
            }
            break;
        }

        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* D4 */
            s.q0[14] = (rng.gen::<u32>() | 0x400be088) & !0xbf040000;
            s.q1[14] = s.q0[14] - 0x80000000;
    
            s.x0[29] = RR!(s.q0[14] - s.q0[13], 12) - md5_F!(s.q0[13], s.q0[12], s.q0[11])
                - s.q0[10] - 0xfd987193;
            s.x1[29] = RR!(s.q1[14] - s.q1[13], 12) - md5_F!(s.q1[13], s.q1[12], s.q1[11])
                - s.q1[10] - 0xfd987193;
            if s.x0[29] != s.x1[29]
            {
                continue;
            }
            break;
        }
    
        i = -1;
        loop
        {
            i += 1;
            if i == 10
            {
                continue 'block2_again;
            }
            /* C4 */
            s.q0[15] = (rng.gen::<u32>() | 0x7d000000) & !0x82008008;
            s.q1[15] = s.q0[15] - 0x7fff7ff8;
    
            s.x0[30] = RR!(s.q0[15] - s.q0[14], 17) - md5_F!(s.q0[14], s.q0[13], s.q0[12])
                - s.q0[11] - 0xa679438e;
            s.x1[30] = RR!(s.q1[15] - s.q1[14], 17) - md5_F!(s.q1[14], s.q1[13], s.q1[12])
                - s.q1[11] - 0xa679438e;
            if (s.x0[30] ^ s.x1[30]) != 0x80000000
            {
                continue;
            }
            break;
        }
    
        i = -1;
        'loop_21: loop
        {
            i += 1;
            if i as u32 == LOOP_21
            {
                continue 'block2_again;
            }
            /* B4 */
            s.q0[16] = (rng.gen::<u32>() | 0x20000000) & !0x80000000;
            s.q1[16] = s.q0[16] - 0xa0000000;
    
            s.x0[31] = RR!(s.q0[16] - s.q0[15], 22) - md5_F!(s.q0[15], s.q0[14], s.q0[13])
                - s.q0[12] - 0x49b40821;
            s.x1[31] = RR!(s.q1[16] - s.q1[15], 22) - md5_F!(s.q1[15], s.q1[14], s.q1[13])
                - s.q1[12] - 0x49b40821;
            if s.x0[31] != s.x1[31]
            {
                continue 'loop_21;
            }
    
            /* A5 */
            s.q0[17] = RL!(md5_G!(s.q0[16], s.q0[15], s.q0[14]) + s.q0[13]
                    + s.x0[17] + 0xf61e2562,  5) + s.q0[16];
            if (s.q0[17] & 0x80028008) != (s.q0[16] & 0x00008008)
            {
                continue 'loop_21;
            }
            s.q1[17] = RL!(md5_G!(s.q1[16], s.q1[15], s.q1[14]) + s.q1[13]
                    + s.x1[17] + 0xf61e2562,  5) + s.q1[16];
            if (s.q0[17] ^ s.q1[17]) != 0x80000000
            {
                continue 'loop_21;
            }
    
            /* D5 */
            s.q0[18] = RL!(md5_G!(s.q0[17], s.q0[16], s.q0[15]) + s.q0[14]
                    + s.x0[22] + 0xc040b340,  9) + s.q0[17];
            if (s.q0[18] & 0xa0020000)
                    != ((s.q0[17] & 0x20000000) | 0x00020000)
            {
                continue 'loop_21;
            }
            s.q1[18] = RL!(md5_G!(s.q1[17], s.q1[16], s.q1[15]) + s.q1[14]
                    + s.x1[22] + 0xc040b340,  9) + s.q1[17];
            if (s.q0[18] ^ s.q1[18]) != 0x80000000
            {
                continue 'loop_21;
            }
    
            /* C5 */
            s.q0[19] = RL!(md5_G!(s.q0[18], s.q0[17], s.q0[16]) + s.q0[15]
                    + s.x0[27] + 0x265e5a51, 14) + s.q0[18];
            if s.q0[19] & 0x80020000 != 0
            {
                continue 'loop_21;
            }
            s.q1[19] = RL!(md5_G!(s.q1[18], s.q1[17], s.q1[16]) + s.q1[15]
                    + s.x1[27] + 0x265e5a51, 14) + s.q1[18];
            if (s.q0[19] - s.q1[19]) != 0x7ffe0000
            {
                continue 'loop_21;
            }
    
            /* B5 */
            s.q0[20] = RL!(md5_G!(s.q0[19], s.q0[18], s.q0[17]) + s.q0[16]
                    + s.x0[16] + 0xe9b6c7aa, 20) + s.q0[19];
            if s.q0[20] & 0x80000000 != 0
            {
                continue 'loop_21;
            }
            s.q1[20] = RL!(md5_G!(s.q1[19], s.q1[18], s.q1[17]) + s.q1[16]
                    + s.x1[16] + 0xe9b6c7aa, 20) + s.q1[19];
            if (s.q0[20] ^ s.q1[20]) != 0x80000000
            {
                continue 'loop_21;
            }
    
            /* A6 */
            s.q0[21] = RL!(md5_G!(s.q0[20], s.q0[19], s.q0[18]) + s.q0[17]
                    + s.x0[21] + 0xd62f105d,  5) + s.q0[20];
            if (s.q0[21] & 0x80020000) != (s.q0[20] & 0x00020000)
            {
                continue 'loop_21;
            }
            s.q1[21] = RL!(md5_G!(s.q1[20], s.q1[19], s.q1[18]) + s.q1[17]
                    + s.x1[21] + 0xd62f105d,  5) + s.q1[20];
            if (s.q0[21] ^ s.q1[21]) != 0x80000000
            {
                continue 'loop_21;
            }
            break;
        }

        ct3 += 1;
        i = -1;
        'loop_22: loop
        {
            i += 1;
            if i as u32 == LOOP_22
            {
                continue 'block2_again;
            }
            if (i & 0xfffff) == 0 
            {
                //callback4(s.ct1, s.ct2, ct3, (i>>20) as u32);
            }
    
            /* B4 */
            s.q0[16] ^= mask22[rng.gen::<usize>() % 30];
            s.q1[16] = s.q0[16] - 0xa0000000;

            s.x0[31] = RR!(s.q0[16] - s.q0[15], 22) - md5_F!(s.q0[15], s.q0[14], s.q0[13])
                    - s.q0[12] - 0x49b40821;
            s.x1[31] = RR!(s.q1[16] - s.q1[15], 22) - md5_F!(s.q1[15], s.q1[14], s.q1[13])
                    - s.q1[12] - 0x49b40821;
            if s.x0[31] != s.x1[31]
            {
                continue 'loop_22;
            }
    
            /* A5 */
            s.q0[17] = RL!(md5_G!(s.q0[16], s.q0[15], s.q0[14]) + s.q0[13]
                + s.x0[17] + 0xf61e2562,  5) + s.q0[16];
            if (s.q0[17] & 0x80028008) != (s.q0[16] & 0x00008008)
            {
                continue 'loop_22;
            }
            s.q1[17] = RL!(md5_G!(s.q1[16], s.q1[15], s.q1[14]) + s.q1[13]
                + s.x1[17] + 0xf61e2562,  5) + s.q1[16];
            if (s.q0[17] ^ s.q1[17]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D5 */
            s.q0[18] = RL!(md5_G!(s.q0[17], s.q0[16], s.q0[15]) + s.q0[14]
                + s.x0[22] + 0xc040b340,  9) + s.q0[17];
            if (s.q0[18] & 0xa0020000)
                != (s.q0[17] & 0x20000000) | 0x00020000
            {
                continue 'loop_22;
            }
            s.q1[18] = RL!(md5_G!(s.q1[17], s.q1[16], s.q1[15]) + s.q1[14]
                + s.x1[22] + 0xc040b340,  9) + s.q1[17];
            if (s.q0[18] ^ s.q1[18]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C5 */
            s.q0[19] = RL!(md5_G!(s.q0[18], s.q0[17], s.q0[16]) + s.q0[15]
                + s.x0[27] + 0x265e5a51, 14) + s.q0[18];
            if s.q0[19] & 0x80020000 != 0
            {
                continue 'loop_22;
            }
            s.q1[19] = RL!(md5_G!(s.q1[18], s.q1[17], s.q1[16]) + s.q1[15]
                + s.x1[27] + 0x265e5a51, 14) + s.q1[18];
            if (s.q0[19] - s.q1[19]) != 0x7ffe0000
            {
                continue 'loop_22;
            }
    
            /* B5 */
            s.q0[20] = RL!(md5_G!(s.q0[19], s.q0[18], s.q0[17]) + s.q0[16]
                + s.x0[16] + 0xe9b6c7aa, 20) + s.q0[19];
            if s.q0[20] & 0x80000000 != 0
            {
                continue 'loop_22;
            }
            s.q1[20] = RL!(md5_G!(s.q1[19], s.q1[18], s.q1[17]) + s.q1[16]
                + s.x1[16] + 0xe9b6c7aa, 20) + s.q1[19];
            if (s.q0[20] ^ s.q1[20]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A6 */
            s.q0[21] = RL!(md5_G!(s.q0[20], s.q0[19], s.q0[18]) + s.q0[17]
                + s.x0[21] + 0xd62f105d,  5) + s.q0[20];
            if (s.q0[21] & 0x80020000) != (s.q0[20] & 0x00020000)
            {
                continue 'loop_22;
            }
            s.q1[21] = RL!(md5_G!(s.q1[20], s.q1[19], s.q1[18]) + s.q1[17]
                + s.x1[21] + 0xd62f105d,  5) + s.q1[20];
            if (s.q0[21] ^ s.q1[21]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D6 */
            s.q0[22] = RL!(md5_G!(s.q0[21], s.q0[20], s.q0[19]) + s.q0[18]
                + s.x0[26] + 0x02441453,  9) + s.q0[21];
            if s.q0[22] & 0x80000000 != 0
            {
                continue 'loop_22;
            }
            s.q1[22] = RL!(md5_G!(s.q1[21], s.q1[20], s.q1[19]) + s.q1[18]
                + s.x1[26] + 0x02441453,  9) + s.q1[21];
            if (s.q0[22] ^ s.q1[22]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C6 */
            s.q0[23] = RL!(md5_G!(s.q0[22], s.q0[21], s.q0[20]) + s.q0[19]
                + s.x0[31] + 0xd8a1e681, 14) + s.q0[22];
            if s.q0[23] & 0x80000000 != 0
            {
                continue 'loop_22;
            }
            s.q1[23] = RL!(md5_G!(s.q1[22], s.q1[21], s.q1[20]) + s.q1[19]
                + s.x1[31] + 0xd8a1e681, 14) + s.q1[22];
            if s.q0[23] != s.q1[23]
            {
                continue 'loop_22;
            }
    
            /* B6 */
            s.q0[24] = RL!(md5_G!(s.q0[23], s.q0[22], s.q0[21]) + s.q0[20]
                + s.x0[20] + 0xe7d3fbc8, 20) + s.q0[23];
            s.q1[24] = RL!(md5_G!(s.q1[23], s.q1[22], s.q1[21]) + s.q1[20]
                + s.x1[20] + 0xe7d3fbc8, 20) + s.q1[23];
            if s.q0[24] != s.q1[24]
            {
                continue 'loop_22;
            }
    
            /* A7 */
            s.q0[25] = RL!(md5_G!(s.q0[24], s.q0[23], s.q0[22]) + s.q0[21]
                + s.x0[25] + 0x21e1cde6,  5) + s.q0[24];
            s.q1[25] = RL!(md5_G!(s.q1[24], s.q1[23], s.q1[22]) + s.q1[21]
                + s.x1[25] + 0x21e1cde6,  5) + s.q1[24];
            if s.q0[25] != s.q1[25]
            {
                continue 'loop_22;
            }
    
            /* D7 */
            s.q0[26] = RL!(md5_G!(s.q0[25], s.q0[24], s.q0[23]) + s.q0[22]
                + s.x0[30] + 0xc33707d6,  9) + s.q0[25];
            s.q1[26] = RL!(md5_G!(s.q1[25], s.q1[24], s.q1[23]) + s.q1[22]
                + s.x1[30] + 0xc33707d6,  9) + s.q1[25];
            if s.q0[26] != s.q1[26]
            {
                continue 'loop_22;
            }
    
            /* C7 */
            s.q0[27] = RL!(md5_G!(s.q0[26], s.q0[25], s.q0[24]) + s.q0[23]
                + s.x0[19] + 0xf4d50d87, 14) + s.q0[26];
            s.q1[27] = RL!(md5_G!(s.q1[26], s.q1[25], s.q1[24]) + s.q1[23]
                + s.x1[19] + 0xf4d50d87, 14) + s.q1[26];
            if s.q0[27] != s.q1[27]
            {
                continue 'loop_22;
            }
    
            /* B7 */
            s.q0[28] = RL!(md5_G!(s.q0[27], s.q0[26], s.q0[25]) + s.q0[24]
                + s.x0[24] + 0x455a14ed, 20) + s.q0[27];
            s.q1[28] = RL!(md5_G!(s.q1[27], s.q1[26], s.q1[25]) + s.q1[24]
                + s.x1[24] + 0x455a14ed, 20) + s.q1[27];
            if s.q0[28] != s.q1[28]
            {
                continue 'loop_22;
            }
    
            /* A8 */
            s.q0[29] = RL!(md5_G!(s.q0[28], s.q0[27], s.q0[26]) + s.q0[25]
                + s.x0[29] + 0xa9e3e905,  5) + s.q0[28];
            s.q1[29] = RL!(md5_G!(s.q1[28], s.q1[27], s.q1[26]) + s.q1[25]
                + s.x1[29] + 0xa9e3e905,  5) + s.q1[28];
            if s.q0[29] != s.q1[29]
            {
                continue 'loop_22;
            }
    
            /* D8 */
            s.q0[30] = RL!(md5_G!(s.q0[29], s.q0[28], s.q0[27]) + s.q0[26]
                + s.x0[18] + 0xfcefa3f8,  9) + s.q0[29];
            s.q1[30] = RL!(md5_G!(s.q1[29], s.q1[28], s.q1[27]) + s.q1[26]
                + s.x1[18] + 0xfcefa3f8,  9) + s.q1[29];
            if s.q0[30] != s.q1[30]
            {
                continue 'loop_22;
            }
    
            /* C8 */
            s.q0[31] = RL!(md5_G!(s.q0[30], s.q0[29], s.q0[28]) + s.q0[27]
                + s.x0[23] + 0x676f02d9, 14) + s.q0[30];
            s.q1[31] = RL!(md5_G!(s.q1[30], s.q1[29], s.q1[28]) + s.q1[27]
                + s.x1[23] + 0x676f02d9, 14) + s.q1[30];
            if s.q0[31] != s.q1[31]
            {
                continue 'loop_22;
            }
    
            /* B8 */
            s.q0[32] = RL!(md5_G!(s.q0[31], s.q0[30], s.q0[29]) + s.q0[28]
                + s.x0[28] + 0x8d2a4c8a, 20) + s.q0[31];
            s.q1[32] = RL!(md5_G!(s.q1[31], s.q1[30], s.q1[29]) + s.q1[28]
                + s.x1[28] + 0x8d2a4c8a, 20) + s.q1[31];
            if s.q0[32] != s.q1[32]
            {
                continue 'loop_22;
            }
    
            /* A9 */
            s.q0[33] = RL!(md5_H!(s.q0[32], s.q0[31], s.q0[30]) + s.q0[29]
                + s.x0[21] + 0xfffa3942,  4) + s.q0[32];
            s.q1[33] = RL!(md5_H!(s.q1[32], s.q1[31], s.q1[30]) + s.q1[29]
                + s.x1[21] + 0xfffa3942,  4) + s.q1[32];
            if s.q0[33] != s.q1[33]
            {
                continue 'loop_22;
            }
    
            /* D9 */
            s.q0[34] = RL!(md5_H!(s.q0[33], s.q0[32], s.q0[31]) + s.q0[30]
                + s.x0[24] + 0x8771f681, 11) + s.q0[33];
            s.q1[34] = RL!(md5_H!(s.q1[33], s.q1[32], s.q1[31]) + s.q1[30]
                + s.x1[24] + 0x8771f681, 11) + s.q1[33];
            if s.q0[34] != s.q1[34]
            {
                continue 'loop_22;
            }
    
            /* C9 */
            s.q0[35] = RL!(md5_H!(s.q0[34], s.q0[33], s.q0[32]) + s.q0[31]
                + s.x0[27] + 0x6d9d6122, 16) + s.q0[34];
            s.q1[35] = RL!(md5_H!(s.q1[34], s.q1[33], s.q1[32]) + s.q1[31]
                + s.x1[27] + 0x6d9d6122, 16) + s.q1[34];
            if (s.q0[35] ^ s.q1[35]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B9 */
            s.q0[36] = RL!(md5_H!(s.q0[35], s.q0[34], s.q0[33]) + s.q0[32]
                + s.x0[30] + 0xfde5380c, 23) + s.q0[35];
            s.q1[36] = RL!(md5_H!(s.q1[35], s.q1[34], s.q1[33]) + s.q1[32]
                + s.x1[30] + 0xfde5380c, 23) + s.q1[35];
            if (s.q0[36] ^ s.q1[36]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A10 */
            s.q0[37] = RL!(md5_H!(s.q0[36], s.q0[35], s.q0[34]) + s.q0[33]
                + s.x0[17] + 0xa4beea44,  4) + s.q0[36];
            s.q1[37] = RL!(md5_H!(s.q1[36], s.q1[35], s.q1[34]) + s.q1[33]
                + s.x1[17] + 0xa4beea44,  4) + s.q1[36];
            if (s.q0[37] ^ s.q1[37]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D10 */
            s.q0[38] = RL!(md5_H!(s.q0[37], s.q0[36], s.q0[35]) + s.q0[34]
                + s.x0[20] + 0x4bdecfa9, 11) + s.q0[37];
            s.q1[38] = RL!(md5_H!(s.q1[37], s.q1[36], s.q1[35]) + s.q1[34]
                + s.x1[20] + 0x4bdecfa9, 11) + s.q1[37];
            if (s.q0[38] ^ s.q1[38]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C10 */
            s.q0[39] = RL!(md5_H!(s.q0[38], s.q0[37], s.q0[36]) + s.q0[35]
                + s.x0[23] + 0xf6bb4b60, 16) + s.q0[38];
            s.q1[39] = RL!(md5_H!(s.q1[38], s.q1[37], s.q1[36]) + s.q1[35]
                + s.x1[23] + 0xf6bb4b60, 16) + s.q1[38];
            if (s.q0[39] ^ s.q1[39]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B10 */
            s.q0[40] = RL!(md5_H!(s.q0[39], s.q0[38], s.q0[37]) + s.q0[36]
                + s.x0[26] + 0xbebfbc70, 23) + s.q0[39];
            s.q1[40] = RL!(md5_H!(s.q1[39], s.q1[38], s.q1[37]) + s.q1[36]
                + s.x1[26] + 0xbebfbc70, 23) + s.q1[39];
            if (s.q0[40] ^ s.q1[40]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A11 */
            s.q0[41] = RL!(md5_H!(s.q0[40], s.q0[39], s.q0[38]) + s.q0[37]
                + s.x0[29] + 0x289b7ec6,  4) + s.q0[40];
            s.q1[41] = RL!(md5_H!(s.q1[40], s.q1[39], s.q1[38]) + s.q1[37]
                + s.x1[29] + 0x289b7ec6,  4) + s.q1[40];
            if (s.q0[41] ^ s.q1[41]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D11 */
            s.q0[42] = RL!(md5_H!(s.q0[41], s.q0[40], s.q0[39]) + s.q0[38]
                + s.x0[16] + 0xeaa127fa, 11) + s.q0[41];
            s.q1[42] = RL!(md5_H!(s.q1[41], s.q1[40], s.q1[39]) + s.q1[38]
                + s.x1[16] + 0xeaa127fa, 11) + s.q1[41];
            if (s.q0[42] ^ s.q1[42]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C11 */
            s.q0[43] = RL!(md5_H!(s.q0[42], s.q0[41], s.q0[40]) + s.q0[39]
                + s.x0[19] + 0xd4ef3085, 16) + s.q0[42];
            s.q1[43] = RL!(md5_H!(s.q1[42], s.q1[41], s.q1[40]) + s.q1[39]
                + s.x1[19] + 0xd4ef3085, 16) + s.q1[42];
            if (s.q0[43] ^ s.q1[43]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B11 */
            s.q0[44] = RL!(md5_H!(s.q0[43], s.q0[42], s.q0[41]) + s.q0[40]
                + s.x0[22] + 0x04881d05, 23) + s.q0[43];
            s.q1[44] = RL!(md5_H!(s.q1[43], s.q1[42], s.q1[41]) + s.q1[40]
                + s.x1[22] + 0x04881d05, 23) + s.q1[43];
            if (s.q0[44] ^ s.q1[44]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A12 */
            s.q0[45] = RL!(md5_H!(s.q0[44], s.q0[43], s.q0[42]) + s.q0[41]
                + s.x0[25] + 0xd9d4d039,  4) + s.q0[44];
            s.q1[45] = RL!(md5_H!(s.q1[44], s.q1[43], s.q1[42]) + s.q1[41]
                + s.x1[25] + 0xd9d4d039,  4) + s.q1[44];
            if (s.q0[45] ^ s.q1[45]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D12 */
            s.q0[46] = RL!(md5_H!(s.q0[45], s.q0[44], s.q0[43]) + s.q0[42]
                + s.x0[28] + 0xe6db99e5, 11) + s.q0[45];
            s.q1[46] = RL!(md5_H!(s.q1[45], s.q1[44], s.q1[43]) + s.q1[42]
                + s.x1[28] + 0xe6db99e5, 11) + s.q1[45];
            if (s.q0[46] ^ s.q1[46]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C12 */
            s.q0[47] = RL!(md5_H!(s.q0[46], s.q0[45], s.q0[44]) + s.q0[43]
                + s.x0[31] + 0x1fa27cf8, 16) + s.q0[46];
            s.q1[47] = RL!(md5_H!(s.q1[46], s.q1[45], s.q1[44]) + s.q1[43]
                + s.x1[31] + 0x1fa27cf8, 16) + s.q1[46];
            if (s.q0[47] ^ s.q1[47]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B12 */
            s.q0[48] = RL!(md5_H!(s.q0[47], s.q0[46], s.q0[45]) + s.q0[44]
                + s.x0[18] + 0xc4ac5665, 23) + s.q0[47];
            if (s.q0[48] & 0x80000000) != (s.q0[46] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[48] = RL!(md5_H!(s.q1[47], s.q1[46], s.q1[45]) + s.q1[44]
                + s.x1[18] + 0xc4ac5665, 23) + s.q1[47];
            if (s.q0[48] ^ s.q1[48]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A13 */
            s.q0[49] = RL!(md5_I!(s.q0[48], s.q0[47], s.q0[46]) + s.q0[45]
                + s.x0[16] + 0xf4292244,  6) + s.q0[48];
            if (s.q0[49] & 0x80000000) != (s.q0[47] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[49] = RL!(md5_I!(s.q1[48], s.q1[47], s.q1[46]) + s.q1[45]
                + s.x1[16] + 0xf4292244,  6) + s.q1[48];
            if (s.q0[49] ^ s.q1[49]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D13 */
            s.q0[50] = RL!(md5_I!(s.q0[49], s.q0[48], s.q0[47]) + s.q0[46]
                + s.x0[23] + 0x432aff97, 10) + s.q0[49];
            s.q1[50] = RL!(md5_I!(s.q1[49], s.q1[48], s.q1[47]) + s.q1[46]
                + s.x1[23] + 0x432aff97, 10) + s.q1[49];
            if (s.q0[50] ^ s.q1[50]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C13 */
            s.q0[51] = RL!(md5_I!(s.q0[50], s.q0[49], s.q0[48]) + s.q0[47]
                + s.x0[30] + 0xab9423a7, 15) + s.q0[50];
            if (s.q0[51] & 0x80000000) != (s.q0[49] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[51] = RL!(md5_I!(s.q1[50], s.q1[49], s.q1[48]) + s.q1[47]
                + s.x1[30] + 0xab9423a7, 15) + s.q1[50];
            if (s.q0[51] ^ s.q1[51]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B13 */
            s.q0[52] = RL!(md5_I!(s.q0[51], s.q0[50], s.q0[49]) + s.q0[48]
                + s.x0[21] + 0xfc93a039, 21) + s.q0[51];
            if (s.q0[52] & 0x80000000) != (s.q0[50] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[52] = RL!(md5_I!(s.q1[51], s.q1[50], s.q1[49]) + s.q1[48]
                + s.x1[21] + 0xfc93a039, 21) + s.q1[51];
            if (s.q0[52] ^ s.q1[52]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A14 */
            s.q0[53] = RL!(md5_I!(s.q0[52], s.q0[51], s.q0[50]) + s.q0[49]
                + s.x0[28] + 0x655b59c3,  6) + s.q0[52];
            if (s.q0[53] & 0x80000000) != (s.q0[51] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[53] = RL!(md5_I!(s.q1[52], s.q1[51], s.q1[50]) + s.q1[49]
                + s.x1[28] + 0x655b59c3,  6) + s.q1[52];
            if (s.q0[53] ^ s.q1[53]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D14 */
            s.q0[54] = RL!(md5_I!(s.q0[53], s.q0[52], s.q0[51]) + s.q0[50]
                + s.x0[19] + 0x8f0ccc92, 10) + s.q0[53];
            if (s.q0[54] & 0x80000000) != (s.q0[52] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[54] = RL!(md5_I!(s.q1[53], s.q1[52], s.q1[51]) + s.q1[50]
                + s.x1[19] + 0x8f0ccc92, 10) + s.q1[53];
            if (s.q0[54] ^ s.q1[54]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C14 */
            s.q0[55] = RL!(md5_I!(s.q0[54], s.q0[53], s.q0[52]) + s.q0[51]
                + s.x0[26] + 0xffeff47d, 15) + s.q0[54];
            if (s.q0[55] & 0x80000000) != (s.q0[53] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[55] = RL!(md5_I!(s.q1[54], s.q1[53], s.q1[52]) + s.q1[51]
                + s.x1[26] + 0xffeff47d, 15) + s.q1[54];
            if (s.q0[55] ^ s.q1[55]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B14 */
            s.q0[56] = RL!(md5_I!(s.q0[55], s.q0[54], s.q0[53]) + s.q0[52]
                + s.x0[17] + 0x85845dd1, 21) + s.q0[55];
            if (s.q0[56] & 0x80000000) != (s.q0[54] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[56] = RL!(md5_I!(s.q1[55], s.q1[54], s.q1[53]) + s.q1[52]
                + s.x1[17] + 0x85845dd1, 21) + s.q1[55];
            if (s.q0[56] ^ s.q1[56]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A15 */
            s.q0[57] = RL!(md5_I!(s.q0[56], s.q0[55], s.q0[54]) + s.q0[53]
                + s.x0[24] + 0x6fa87e4f,  6) + s.q0[56];
            if (s.q0[57] & 0x80000000) != (s.q0[55] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[57] = RL!(md5_I!(s.q1[56], s.q1[55], s.q1[54]) + s.q1[53]
                + s.x1[24] + 0x6fa87e4f,  6) + s.q1[56];
            if (s.q0[57] ^ s.q1[57]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* D15 */
            s.q0[58] = RL!(md5_I!(s.q0[57], s.q0[56], s.q0[55]) + s.q0[54]
                + s.x0[31] + 0xfe2ce6e0, 10) + s.q0[57];
            if (s.q0[58] & 0x80000000) != (s.q0[56] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[58] = RL!(md5_I!(s.q1[57], s.q1[56], s.q1[55]) + s.q1[54]
                + s.x1[31] + 0xfe2ce6e0, 10) + s.q1[57];
            if (s.q0[58] ^ s.q1[58]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* C15 */
            s.q0[59] = RL!(md5_I!(s.q0[58], s.q0[57], s.q0[56]) + s.q0[55]
                + s.x0[22] + 0xa3014314, 15) + s.q0[58];
            if (s.q0[59] & 0x80000000) != (s.q0[57] & 0x80000000)
            {
                continue 'loop_22;
            }
            s.q1[59] = RL!(md5_I!(s.q1[58], s.q1[57], s.q1[56]) + s.q1[55]
                + s.x1[22] + 0xa3014314, 15) + s.q1[58];
            if (s.q0[59] ^ s.q1[59]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* B15 */
            s.q0[60] = RL!(md5_I!(s.q0[59], s.q0[58], s.q0[57]) + s.q0[56]
                + s.x0[29] + 0x4e0811a1, 21) + s.q0[59];
            s.q1[60] = RL!(md5_I!(s.q1[59], s.q1[58], s.q1[57]) + s.q1[56]
                + s.x1[29] + 0x4e0811a1, 21) + s.q1[59];
            if (s.q0[60] ^ s.q1[60]) != 0x80000000
            {
                continue 'loop_22;
            }
    
            /* A16 */
            s.q0[61] = RL!(md5_I!(s.q0[60], s.q0[59], s.q0[58]) + s.q0[57]
                + s.x0[20] + 0xf7537e82,  6) + s.q0[60];
            s.q1[61] = RL!(md5_I!(s.q1[60], s.q1[59], s.q1[58]) + s.q1[57]
                + s.x1[20] + 0xf7537e82,  6) + s.q1[60];
            if (s.q0[61] ^ s.q1[61]) != 0x80000000
            {
                continue 'loop_22;
            }
            if (s.a0 + s.q0[61]) != (s.a1 + s.q1[61])
            {
                continue 'loop_22;
            }
    
            /* D16 */
            s.q0[62] = RL!(md5_I!(s.q0[61], s.q0[60], s.q0[59]) + s.q0[58]
                + s.x0[27] + 0xbd3af235, 10) + s.q0[61];
            s.q1[62] = RL!(md5_I!(s.q1[61], s.q1[60], s.q1[59]) + s.q1[58]
                + s.x1[27] + 0xbd3af235, 10) + s.q1[61];
            if (s.d0 + s.q0[62]) != (s.d1 + s.q1[62])
            {
                continue 'loop_22;
            }
    
            /* C16 */
            s.q0[63] = RL!(md5_I!(s.q0[62], s.q0[61], s.q0[60]) + s.q0[59]
                + s.x0[18] + 0x2ad7d2bb, 15) + s.q0[62];
            s.q1[63] = RL!(md5_I!(s.q1[62], s.q1[61], s.q1[60]) + s.q1[59]
                + s.x1[18] + 0x2ad7d2bb, 15) + s.q1[62];
            if (s.c0 + s.q0[63]) != (s.c1 + s.q1[63])
            {
                continue 'loop_22;
            }
    
            /* B16 */
            s.q0[64] = RL!(md5_I!(s.q0[63], s.q0[62], s.q0[61]) + s.q0[60]
                + s.x0[25] + 0xeb86d391, 21) + s.q0[63];
            s.q1[64] = RL!(md5_I!(s.q1[63], s.q1[62], s.q1[61]) + s.q1[60]
                + s.x1[25] + 0xeb86d391, 21) + s.q1[63];
            if (s.b0 + s.q0[64]) != (s.b1 + s.q1[64])
            {
                continue 'loop_22;
            }
            break;
        }
        return 0;
    }
}

/* return 0 on success, 1 if interrupt requested */
fn md5coll_with_iv(iv: [u32; 4], m0: [u32; 32], m1: [u32; 32]) -> i32
{
    let mut r: i32;
    let mut ct1: i32 = 0;
    let mut s: StateS = StateS{ 
        a0: 0,
        b0: 0,
        c0: 0,
        d0: 0,
        a1: 0,
        b1: 0,
        c1: 0,
        d1: 0,
        q0: [0; 65],
        q1: [0; 65],
        x0: [0; 32],
        x1: [0; 32],
        ct1: 0,
        ct2: 0,
    };

    loop
    {
        r = block1(iv, ct1, &mut s);
        if r == 1
        {
            return 1;
        }

        r = block2(&mut s);
        if r==1
        {
            return 1;
        }
        else if r == -1
        {
            unsafe
            {
                ct1 = s.ct1;
            }
            continue;
        }
        else
        {
            break;
        }
    }
    //memcpy(m0, s.x0, 128);
    //memcpy(m1, s.x1, 128);
    unsafe
    {
        println!("x0");
        for i in 0..32
        {
            println!("0x{:X}", s.x0[i]);
        }
        println!("x1");
        for i in 0..32
        {
            println!("0x{:X}", s.x1[i]);
        }
    }
    return 0;
}

fn main() -> std::io::Result<()>
{
    println!("MD5 Collision in Rust Program Started");
    let args: Vec<String> = env::args().collect();

    //let option = &args[1];
    //let filename = &args[2];
    //let file = File::open(filename)?;
    let mut a: u32 = 5;
    let mut iv : [u32; 4] = [0; 4];
    //find_iv(filename, &mut iv);
    let items: Vec<_> = iter::repeat(0).take(1).collect();

    let threads: Vec<_> = items
        .into_iter()
        .map(|_| {
            thread::spawn(move || {
                println!("Started!");
                let m0: [u32; 32] = [0; 32];
                let m1: [u32; 32] = [0; 32];
                md5coll_with_iv(IV_DEFAULT, m0, m1);
                println!("Finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap()
    }

    Ok(())
}