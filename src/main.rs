#![allow(arithmetic_overflow)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::result::Result;
use rand::Rng;

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
    ($x: expr, $y: expr, $z: expr)=> ($y ^ ($x | (!$z)))
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
const LOOP_22: u32 = 0x4000000;

struct state_s {
    A0: u32,
    B0: u32,
    C0: u32,
    D0: u32,
    A1: u32,
    B1: u32,
    C1: u32,
    D1: u32,
    Q0: [u32; 65],
    Q1: [u32; 65],
    X0: [u32; 32],
    X1: [u32; 32],
    ct1: i32,
    ct2: u32,
}

static mut s: state_s = state_s{ 
    A0: 0,
    B0: 0,
    C0: 0,
    D0: 0,
    A1: 0,
    B1: 0,
    C1: 0,
    D1: 0,
    Q0: [0; 65],
    Q1: [0; 65],
    X0: [0; 32],
    X1: [0; 32],
    ct1: 0,
    ct2: 0,
};

static IV_default: [u32; 4] =  [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

fn block1(IV: [u32; 4], mut ct1: i32) -> u8
{
    let mut rng = rand::thread_rng();
    let mut goto_flag = 0;
    let mut cnt:u32 = 0;

    unsafe {
        /* instead of goto block1_again */
        loop
        {
            loop
            {
                /* C1 */
                s.Q0[ 3] = rng.gen::<u32>() & !0x00800040;
                s.Q1[ 3] = s.Q0[ 3];

                /* B1 */
                s.Q0[ 4] = (rng.gen::<u32>() | 0x80080800) & !(0x00800040 | 0x0077f780);
                s.Q0[ 4] |= (s.Q0[ 3] & 0x0077f780);
                s.Q1[ 4] = s.Q0[ 4];

                /* A2 */
                s.Q0[ 5] = (rng.gen::<u32>() | 0x88400025) & !0x02bfffc0;
                s.Q1[ 5] = s.Q0[ 5] - 0x00000040;

                /* D2 */
                s.Q0[ 6] = (rng.gen::<u32>() | 0x027fbc41) & !(0x888043a4 | 0x7500001a);
                s.Q0[ 6] |= (s.Q0[ 5] & 0x7500001a);
                s.Q1[ 6] = s.Q0[ 6] - 0x7f800040;

                /* C2 */
                s.Q0[ 7] = (rng.gen::<u32>() | 0x03fef820) & !0xfc0107df;
                s.Q1[ 7] = s.Q0[ 7] - 0x07800041;

                s.X0[ 6] = RR!(s.Q0[ 7] - s.Q0[ 6], 17) - md5_F!(s.Q0[ 6], s.Q0[ 5], s.Q0[ 4])
                - s.Q0[ 3] - 0xa8304613;
                s.X1[ 6] = RR!(s.Q1[ 7] - s.Q1[ 6], 17) - md5_F!(s.Q1[ 6], s.Q1[ 5], s.Q1[ 4])
                    - s.Q1[ 3] - 0xa8304613;
                if s.X0[ 6] != s.X1[ 6]
                    { continue; }
                /* B2 */
                s.Q0[ 8] = (rng.gen::<u32>() | 0x01910540) & !0xfe0eaabf;
                s.Q1[ 8] = s.Q0[ 8] - 0x00827fff;
                
                s.X0[ 7] = RR!(s.Q0[ 8] - s.Q0[ 7], 22) - md5_F!(s.Q0[ 7], s.Q0[ 6], s.Q0[ 5])
                    - s.Q0[ 4] - 0xfd469501;
                s.X1[ 7] = RR!(s.Q1[ 8] - s.Q1[ 7], 22) - md5_F!(s.Q1[ 7], s.Q1[ 6], s.Q1[ 5])
                    - s.Q1[ 4] - 0xfd469501;
                if s.X0[ 7] != s.X1[ 7]
                    { continue; }

                /* A3 */
                s.Q0[ 9] = (rng.gen::<u32>() | 0xfb102f3d) & !(0x040f80c2 | 0x00001000);
                s.Q0[ 9] |= (s.Q0[ 8] & 0x00001000);
                s.Q1[ 9] = s.Q0[ 9] - 0x8000003f;

                s.X0[ 8] = RR!(s.Q0[ 9] - s.Q0[ 8],  7) - md5_F!(s.Q0[ 8], s.Q0[ 7], s.Q0[ 6])
                    - s.Q0[ 5] - 0x698098d8;
                s.X1[ 8] = RR!(s.Q1[ 9] - s.Q1[ 8],  7) - md5_F!(s.Q1[ 8], s.Q1[ 7], s.Q1[ 6])
                    - s.Q1[ 5] - 0x698098d8;
                if s.X0[ 8] != s.X1[ 8]
                    { continue; }

                /* D3 */
                s.Q0[10] = (rng.gen::<u32>() | 0x401f9040) & !0x80802183;
                s.Q1[10] = s.Q0[10] - 0x7ffff000;

                s.X0[ 9] = RR!(s.Q0[10] - s.Q0[ 9], 12) - md5_F!(s.Q0[ 9], s.Q0[ 8], s.Q0[ 7])
                    - s.Q0[ 6] - 0x8b44f7af;
                s.X1[ 9] = RR!(s.Q1[10] - s.Q1[ 9], 12) - md5_F!(s.Q1[ 9], s.Q1[ 8], s.Q1[ 7])
                    - s.Q1[ 6] - 0x8b44f7af;
                if s.X0[ 9] != s.X1[ 9]
                    { continue; }

                /* C3 */
                s.Q0[11] = (rng.gen::<u32>() | 0x000180c2) & !(0xc00e3101 | 0x00004000);
                s.Q0[11] |= (s.Q0[10] & 0x00004000);
                s.Q1[11] = s.Q0[11] - 0x40000000;

                s.X0[10] = RR!(s.Q0[11] - s.Q0[10], 17) - md5_F!(s.Q0[10], s.Q0[ 9], s.Q0[ 8])
                    - s.Q0[ 7] - 0xffff5bb1;
                s.X1[10] = RR!(s.Q1[11] - s.Q1[10], 17) - md5_F!(s.Q1[10], s.Q1[ 9], s.Q1[ 8])
                    - s.Q1[ 7] - 0xffff5bb1;
                if s.X0[10] != s.X1[10]
                    { continue; }

                /* B3 */
                s.Q0[12] = (rng.gen::<u32>() | 0x00081100) & !(0xc007e080 | 0x03000000);
                s.Q0[12] |= (s.Q0[11] & 0x03000000);
                s.Q1[12] = s.Q0[12] - 0x80002080;
                
                s.X0[11] = RR!(s.Q0[12] - s.Q0[11], 22) - md5_F!(s.Q0[11], s.Q0[10], s.Q0[ 9])
                    - s.Q0[ 8] - 0x895cd7be;
                s.X1[11] = RR!(s.Q1[12] - s.Q1[11], 22) - md5_F!(s.Q1[11], s.Q1[10], s.Q1[ 9])
                    - s.Q1[ 8] - 0x895cd7be;
                if (s.X0[11] ^ s.X1[11]) != 0x00008000
                    { continue; }

                /* A4 */
                s.Q0[13] = (rng.gen::<u32>() | 0x410fe008) & !0x82000180;
                s.Q1[13] = s.Q0[13] - 0x7f000000;

                s.X0[12] = RR!(s.Q0[13] - s.Q0[12],  7) - md5_F!(s.Q0[12], s.Q0[11], s.Q0[10])
                    - s.Q0[ 9] - 0x6b901122;
                s.X1[12] = RR!(s.Q1[13] - s.Q1[12],  7) - md5_F!(s.Q1[12], s.Q1[11], s.Q1[10])
                    - s.Q1[ 9] - 0x6b901122;
                if s.X0[12] != s.X1[12]
                    { continue; }

                /* D4 */
                s.Q0[14] = (rng.gen::<u32>() | 0x000be188) & !0xa3040000;
                s.Q1[14] = s.Q0[14] - 0x80000000;

                s.X0[13] = RR!(s.Q0[14] - s.Q0[13], 12) - md5_F!(s.Q0[13], s.Q0[12], s.Q0[11])
                    - s.Q0[10] - 0xfd987193;
                s.X1[13] = RR!(s.Q1[14] - s.Q1[13], 12) - md5_F!(s.Q1[13], s.Q1[12], s.Q1[11])
                    - s.Q1[10] - 0xfd987193;
                if s.X0[13] != s.X1[13]
                    { continue; }

                /* C4 */
                s.Q0[15] = (rng.gen::<u32>() | 0x21008000) & !0x82000008;
                s.Q1[15] = s.Q0[15] - 0x80007ff8;

                s.X0[14] = RR!(s.Q0[15] - s.Q0[14], 17) - md5_F!(s.Q0[14], s.Q0[13], s.Q0[12])
                    - s.Q0[11] - 0xa679438e;
                s.X1[14] = RR!(s.Q1[15] - s.Q1[14], 17) - md5_F!(s.Q1[14], s.Q1[13], s.Q1[12])
                    - s.Q1[11] - 0xa679438e;
                if (s.X0[14] ^ s.X1[14]) != 0x80000000
                    { continue; }


                /* B4 */
                s.Q0[16] = (rng.gen::<u32>() | 0x20000000) & !0x80000000;
                s.Q1[16] = s.Q0[16] - 0xa0000000;

                s.X0[15] = RR!(s.Q0[16] - s.Q0[15], 22) - md5_F!(s.Q0[15], s.Q0[14], s.Q0[13])
                    - s.Q0[12] - 0x49b40821;
                s.X1[15] = RR!(s.Q1[16] - s.Q1[15], 22) - md5_F!(s.Q1[15], s.Q1[14], s.Q1[13])
                    - s.Q1[12] - 0x49b40821;
                if s.X0[15] != s.X1[15]
                    { continue; }
                break;
            }
            cnt = 0;
            for i in 0..LOOP_11
            {
                cnt += 1;
                goto_flag = 0;
                /* A5 */
                s.Q0[17] = rng.gen::<u32>() & !(0x80020000 | 0x00008008);
                s.Q0[17] |= (s.Q0[16] & 0x00008008);
                s.Q1[17] = s.Q0[17] - 0x80000000;

                s.X0[ 1] = RR!(s.Q0[17] - s.Q0[16],  5) - md5_G!(s.Q0[16], s.Q0[15], s.Q0[14])
                    - s.Q0[13] - 0xf61e2562;
                s.X1[ 1] = RR!(s.Q1[17] - s.Q1[16],  5) - md5_G!(s.Q1[16], s.Q1[15], s.Q1[14])
                    - s.Q1[13] - 0xf61e2562;
                if s.X0[ 1] != s.X1[ 1]
{ continue; }

                /* D5 */
                s.Q0[18] = RL!(md5_G!(s.Q0[17], s.Q0[16], s.Q0[15]) + s.Q0[14]
                    + s.X0[ 6] + 0xc040b340,  9) + s.Q0[17];
                if (s.Q0[18] & 0xa0020000)
                    != (0x00020000 | (s.Q0[17] & 0x20000000))
                {
                    continue;
                }
                s.Q1[18] = RL!(md5_G!(s.Q1[17], s.Q1[16], s.Q1[15]) + s.Q1[14]
                    + s.X1[ 6] + 0xc040b340,  9) + s.Q1[17];
                if (s.Q0[18] ^ s.Q1[18]) != 0x80000000
{ continue; }

                /* C5 */
                s.Q0[19] = RL!(md5_G!(s.Q0[18], s.Q0[17], s.Q0[16]) + s.Q0[15]
                    + s.X0[11] + 0x265e5a51, 14) + s.Q0[18];
                if s.Q0[19] & 0x80020000 != 0
{ continue; }
                s.Q1[19] = RL!(md5_G!(s.Q1[18], s.Q1[17], s.Q1[16]) + s.Q1[15]
                    + s.X1[11] + 0x265e5a51, 14) + s.Q1[18];
                if s.Q0[19] - s.Q1[19] != 0x7ffe0000
{ continue; }

                /* B5 */
                s.Q0[20] = rng.gen::<u32>() & !0x80000000;
                s.Q1[20] = s.Q0[20] - 0x80000000;

                s.X0[ 0] = RR!(s.Q0[20] - s.Q0[19], 20) - md5_G!(s.Q0[19], s.Q0[18], s.Q0[17])
                    - s.Q0[16] - 0xe9b6c7aa;
                s.X1[ 0] = RR!(s.Q1[20] - s.Q1[19], 20) - md5_G!(s.Q1[19], s.Q1[18], s.Q1[17])
                    - s.Q1[16] - 0xe9b6c7aa;
                if s.X0[ 0] != s.X1[ 0]
{ continue; }

                s.Q0[ 1] = RL!(md5_F!(IV[1], IV[2], IV[3]) + IV[0]
                    + s.X0[ 0] + 0xd76aa478,  7) + IV[1];
                s.Q1[ 1] = s.Q0[ 1];

                s.Q0[ 2] = RL!(md5_F!(s.Q0[ 1], IV[1], IV[2]) + IV[3]
                    + s.X0[ 1] + 0xe8c7b756, 12) + s.Q0[ 1];
                s.Q1[ 2] = s.Q0[ 2];

                s.X0[ 2] = RR!(s.Q0[ 3] - s.Q0[ 2], 17) - md5_F!(s.Q0[ 2], s.Q0[ 1], IV[1])
                    - IV[2] - 0x242070db;
                s.X1[ 2] = s.X0[ 2];

                s.X0[ 3] = RR!(s.Q0[ 4] - s.Q0[ 3], 22) - md5_F!(s.Q0[ 3], s.Q0[ 2], s.Q0[ 1])
                    - IV[1] - 0xc1bdceee;
                s.X1[ 3] = s.X0[ 3];

                s.X0[ 4] = RR!(s.Q0[ 5] - s.Q0[ 4],  7) - md5_F!(s.Q0[ 4], s.Q0[ 3], s.Q0[ 2])
                    - s.Q0[ 1] - 0xf57c0faf;
                s.X1[ 4] = RR!(s.Q1[ 5] - s.Q1[ 4],  7) - md5_F!(s.Q1[ 4], s.Q1[ 3], s.Q1[ 2])
                    - s.Q1[ 1] - 0xf57c0faf;
                if (s.X0[ 4] ^ s.X1[ 4]) != 0x80000000
{ continue; }

                s.X0[ 5] = RR!(s.Q0[ 6] - s.Q0[ 5], 12) - md5_F!(s.Q0[ 5], s.Q0[ 4], s.Q0[ 3])
                    - s.Q0[ 2] - 0x4787c62a;
                s.X1[ 5] = RR!(s.Q1[ 6] - s.Q1[ 5], 12) - md5_F!(s.Q1[ 5], s.Q1[ 4], s.Q1[ 3])
                    - s.Q1[ 2] - 0x4787c62a;
                if s.X0[ 5] != s.X1[ 5]
{ continue; }

                /* A6 */
                s.Q0[21] = RL!(md5_G!(s.Q0[20], s.Q0[19], s.Q0[18]) + s.Q0[17]
                    + s.X0[ 5] + 0xd62f105d,  5) + s.Q0[20];
                if (s.Q0[21] & 0x80020000) != (s.Q0[20] & 0x00020000)
{ continue; }
                s.Q1[21] = RL!(md5_G!(s.Q1[20], s.Q1[19], s.Q1[18]) + s.Q1[17]
                    + s.X1[ 5] + 0xd62f105d,  5) + s.Q1[20];
                if (s.Q0[21] ^ s.Q1[21]) != 0x80000000
{ continue; }

                /* D6 */
                s.Q0[22] = RL!(md5_G!(s.Q0[21], s.Q0[20], s.Q0[19]) + s.Q0[18]
                    + s.X0[10] + 0x02441453,  9) + s.Q0[21];
                if s.Q0[22] & 0x80000000 != 0
{ continue; }
                s.Q1[22] = RL!(md5_G!(s.Q1[21], s.Q1[20], s.Q1[19]) + s.Q1[18]
                    + s.X1[10] + 0x02441453,  9) + s.Q1[21];
                if (s.Q0[22] ^ s.Q1[22]) != 0x80000000
{ continue; }

                /* C6 */
                s.Q0[23] = RL!(md5_G!(s.Q0[22], s.Q0[21], s.Q0[20]) + s.Q0[19]
                    + s.X0[15] + 0xd8a1e681, 14) + s.Q0[22];
                if s.Q0[23] & 0x80000000 != 0
{ continue; }
                s.Q1[23] = RL!(md5_G!(s.Q1[22], s.Q1[21], s.Q1[20]) + s.Q1[19]
                    + s.X1[15] + 0xd8a1e681, 14) + s.Q1[22];
                if s.Q0[23] != s.Q1[23]
{ continue; }

                /* B6 */
                s.Q0[24] = RL!(md5_G!(s.Q0[23], s.Q0[22], s.Q0[21]) + s.Q0[20]
                    + s.X0[ 4] + 0xe7d3fbc8, 20) + s.Q0[23];
                s.Q1[24] = RL!(md5_G!(s.Q1[23], s.Q1[22], s.Q1[21]) + s.Q1[20]
                    + s.X1[ 4] + 0xe7d3fbc8, 20) + s.Q1[23];
                if s.Q0[24] != s.Q1[24]
{ continue; }

                /* A7 */
                s.Q0[25] = RL!(md5_G!(s.Q0[24], s.Q0[23], s.Q0[22]) + s.Q0[21]
                    + s.X0[ 9] + 0x21e1cde6,  5) + s.Q0[24];
                s.Q1[25] = RL!(md5_G!(s.Q1[24], s.Q1[23], s.Q1[22]) + s.Q1[21]
                    + s.X1[ 9] + 0x21e1cde6,  5) + s.Q1[24];
                if s.Q0[25] != s.Q1[25]
{ continue; }

                        /* D7 */
                        s.Q0[26] = RL!(md5_G!(s.Q0[25], s.Q0[24], s.Q0[23]) + s.Q0[22]
                                + s.X0[14] + 0xc33707d6,  9) + s.Q0[25];
                        s.Q1[26] = RL!(md5_G!(s.Q1[25], s.Q1[24], s.Q1[23]) + s.Q1[22]
                                + s.X1[14] + 0xc33707d6,  9) + s.Q1[25];
                        if s.Q0[26] != s.Q1[26]
{ continue; }

                /* C7 */
                s.Q0[27] = RL!(md5_G!(s.Q0[26], s.Q0[25], s.Q0[24]) + s.Q0[23]
                    + s.X0[ 3] + 0xf4d50d87, 14) + s.Q0[26];
                s.Q1[27] = RL!(md5_G!(s.Q1[26], s.Q1[25], s.Q1[24]) + s.Q1[23]
                    + s.X1[ 3] + 0xf4d50d87, 14) + s.Q1[26];
                if s.Q0[27] != s.Q1[27]
{ continue; }
                
                goto_flag = 1;
                break;
            }
            if cnt == (LOOP_11 - 1) && goto_flag == 0
            {
                /* return to first loop */
                continue;
            }
            ct1 += 1;
            cnt = 0;
            for i in 0..LOOP_12
            {
                cnt += 1;
                goto_flag = 0;
                /* B5 */
                s.Q0[20] ^= (1 << (rng.gen::<u32>() % 31));
                s.Q1[20] = s.Q0[20] - 0x80000000;

                s.X0[ 0] = RR!(s.Q0[20] - s.Q0[19], 20) - md5_G!(s.Q0[19], s.Q0[18], s.Q0[17])
                    - s.Q0[16] - 0xe9b6c7aa;
                s.X1[ 0] = RR!(s.Q1[20] - s.Q1[19], 20) - md5_G!(s.Q1[19], s.Q1[18], s.Q1[17])
                    - s.Q1[16] - 0xe9b6c7aa;
                if s.X0[ 0] != s.X1[ 0]
{ continue; }

                s.Q0[ 1] = RL!(md5_F!(IV[1], IV[2], IV[3]) + IV[0]
                    + s.X0[ 0] + 0xd76aa478,  7) + IV[1];
                s.Q1[ 1] = s.Q0[ 1];

                s.Q0[ 2] = RL!(md5_F!(s.Q0[ 1], IV[1], IV[2]) + IV[3]
                    + s.X0[ 1] + 0xe8c7b756, 12) + s.Q0[ 1];
                s.Q1[ 2] = s.Q0[ 2];
                s.X0[ 2] = RR!(s.Q0[ 3] - s.Q0[ 2], 17) - md5_F!(s.Q0[ 2], s.Q0[ 1], IV[1])
                    - IV[2] - 0x242070db;
                s.X1[ 2] = s.X0[ 2];

                s.X0[ 3] = RR!(s.Q0[ 4] - s.Q0[ 3], 22) - md5_F!(s.Q0[ 3], s.Q0[ 2], s.Q0[ 1])
                    - IV[1] - 0xc1bdceee;
                s.X1[ 3] = s.X0[ 3];

                s.X0[ 4] = RR!(s.Q0[ 5] - s.Q0[ 4],  7) - md5_F!(s.Q0[ 4], s.Q0[ 3], s.Q0[ 2])
                    - s.Q0[ 1] - 0xf57c0faf;
                s.X1[ 4] = RR!(s.Q1[ 5] - s.Q1[ 4],  7) - md5_F!(s.Q1[ 4], s.Q1[ 3], s.Q1[ 2])
                    - s.Q1[ 1] - 0xf57c0faf;
                if (s.X0[ 4] ^ s.X1[ 4]) != 0x80000000
{ continue; }

                s.X0[ 5] = RR!(s.Q0[ 6] - s.Q0[ 5], 12) - md5_F!(s.Q0[ 5], s.Q0[ 4], s.Q0[ 3])
                    - s.Q0[ 2] - 0x4787c62a;
                s.X1[ 5] = RR!(s.Q1[ 6] - s.Q1[ 5], 12) - md5_F!(s.Q1[ 5], s.Q1[ 4], s.Q1[ 3])
                    - s.Q1[ 2] - 0x4787c62a;
                if s.X0[ 5] != s.X1[ 5]
{ continue; }

                /* A6 */
                s.Q0[21] = RL!(md5_G!(s.Q0[20], s.Q0[19], s.Q0[18]) + s.Q0[17]
                    + s.X0[ 5] + 0xd62f105d,  5) + s.Q0[20];
                if (s.Q0[21] & 0x80020000) != (s.Q0[20] & 0x00020000)
{ continue; }
                s.Q1[21] = RL!(md5_G!(s.Q1[20], s.Q1[19], s.Q1[18]) + s.Q1[17]
                    + s.X1[ 5] + 0xd62f105d,  5) + s.Q1[20];
                if (s.Q0[21] ^ s.Q1[21]) != 0x80000000
{ continue; }

                /* D6 */
                s.Q0[22] = RL!(md5_G!(s.Q0[21], s.Q0[20], s.Q0[19]) + s.Q0[18]
                    + s.X0[10] + 0x02441453,  9) + s.Q0[21];
                if s.Q0[22] & 0x80000000 != 0
{ continue; }
                s.Q1[22] = RL!(md5_G!(s.Q1[21], s.Q1[20], s.Q1[19]) + s.Q1[18]
                    + s.X1[10] + 0x02441453,  9) + s.Q1[21];
                if (s.Q0[22] ^ s.Q1[22]) != 0x80000000
{ continue; }

                /* C6 */
                s.Q0[23] = RL!(md5_G!(s.Q0[22], s.Q0[21], s.Q0[20]) + s.Q0[19]
                    + s.X0[15] + 0xd8a1e681, 14) + s.Q0[22];
                if s.Q0[23] & 0x80000000 != 0
{ continue; }
                s.Q1[23] = RL!(md5_G!(s.Q1[22], s.Q1[21], s.Q1[20]) + s.Q1[19]
                    + s.X1[15] + 0xd8a1e681, 14) + s.Q1[22];
                if s.Q0[23] != s.Q1[23]
{ continue; }

                /* B6 */
                s.Q0[24] = RL!(md5_G!(s.Q0[23], s.Q0[22], s.Q0[21]) + s.Q0[20]
                    + s.X0[ 4] + 0xe7d3fbc8, 20) + s.Q0[23];
                s.Q1[24] = RL!(md5_G!(s.Q1[23], s.Q1[22], s.Q1[21]) + s.Q1[20]
                    + s.X1[ 4] + 0xe7d3fbc8, 20) + s.Q1[23];
                if s.Q0[24] != s.Q1[24]
{ continue; }

                /* A7 */
                s.Q0[25] = RL!(md5_G!(s.Q0[24], s.Q0[23], s.Q0[22]) + s.Q0[21]
                    + s.X0[ 9] + 0x21e1cde6,  5) + s.Q0[24];
                s.Q1[25] = RL!(md5_G!(s.Q1[24], s.Q1[23], s.Q1[22]) + s.Q1[21]
                    + s.X1[ 9] + 0x21e1cde6,  5) + s.Q1[24];
                if s.Q0[25] != s.Q1[25]
{ continue; }

                /* D7 */
                s.Q0[26] = RL!(md5_G!(s.Q0[25], s.Q0[24], s.Q0[23]) + s.Q0[22]
                    + s.X0[14] + 0xc33707d6,  9) + s.Q0[25];
                s.Q1[26] = RL!(md5_G!(s.Q1[25], s.Q1[24], s.Q1[23]) + s.Q1[22]
                    + s.X1[14] + 0xc33707d6,  9) + s.Q1[25];
                if s.Q0[26] != s.Q1[26]
{ continue; }

                /* C7 */
                s.Q0[27] = RL!(md5_G!(s.Q0[26], s.Q0[25], s.Q0[24]) + s.Q0[23]
                    + s.X0[ 3] + 0xf4d50d87, 14) + s.Q0[26];
                s.Q1[27] = RL!(md5_G!(s.Q1[26], s.Q1[25], s.Q1[24]) + s.Q1[23]
                    + s.X1[ 3] + 0xf4d50d87, 14) + s.Q1[26];
                if s.Q0[27] != s.Q1[27]
{ continue; }

                /* B7 */
                s.Q0[28] = RL!(md5_G!(s.Q0[27], s.Q0[26], s.Q0[25]) + s.Q0[24]
                    + s.X0[ 8] + 0x455a14ed, 20) + s.Q0[27];
                s.Q1[28] = RL!(md5_G!(s.Q1[27], s.Q1[26], s.Q1[25]) + s.Q1[24]
                    + s.X1[ 8] + 0x455a14ed, 20) + s.Q1[27];
                if s.Q0[28] != s.Q1[28]
{ continue; }

                /* A8 */
                s.Q0[29] = RL!(md5_G!(s.Q0[28], s.Q0[27], s.Q0[26]) + s.Q0[25]
                    + s.X0[13] + 0xa9e3e905,  5) + s.Q0[28];
                s.Q1[29] = RL!(md5_G!(s.Q1[28], s.Q1[27], s.Q1[26]) + s.Q1[25]
                    + s.X1[13] + 0xa9e3e905,  5) + s.Q1[28];
                if s.Q0[29] != s.Q1[29]
{ continue; }

                /* D8 */
                s.Q0[30] = RL!(md5_G!(s.Q0[29], s.Q0[28], s.Q0[27]) + s.Q0[26]
                    + s.X0[ 2] + 0xfcefa3f8,  9) + s.Q0[29];
                s.Q1[30] = RL!(md5_G!(s.Q1[29], s.Q1[28], s.Q1[27]) + s.Q1[26]
                    + s.X1[ 2] + 0xfcefa3f8,  9) + s.Q1[29];
                if s.Q0[30] != s.Q1[30]
{ continue; }

                /* C8 */
                s.Q0[31] = RL!(md5_G!(s.Q0[30], s.Q0[29], s.Q0[28]) + s.Q0[27]
                    + s.X0[ 7] + 0x676f02d9, 14) + s.Q0[30];
                s.Q1[31] = RL!(md5_G!(s.Q1[30], s.Q1[29], s.Q1[28]) + s.Q1[27]
                    + s.X1[ 7] + 0x676f02d9, 14) + s.Q1[30];
                if s.Q0[31] != s.Q1[31]
{ continue; }

                /* B8 */
                s.Q0[32] = RL!(md5_G!(s.Q0[31], s.Q0[30], s.Q0[29]) + s.Q0[28]
                    + s.X0[12] + 0x8d2a4c8a, 20) + s.Q0[31];
                s.Q1[32] = RL!(md5_G!(s.Q1[31], s.Q1[30], s.Q1[29]) + s.Q1[28]
                    + s.X1[12] + 0x8d2a4c8a, 20) + s.Q1[31];
                if s.Q0[32] != s.Q1[32]
{ continue; }

                /* A9 */
                s.Q0[33] = RL!(md5_H!(s.Q0[32], s.Q0[31], s.Q0[30]) + s.Q0[29]
                    + s.X0[ 5] + 0xfffa3942,  4) + s.Q0[32];
                s.Q1[33] = RL!(md5_H!(s.Q1[32], s.Q1[31], s.Q1[30]) + s.Q1[29]
                    + s.X1[ 5] + 0xfffa3942,  4) + s.Q1[32];
                if s.Q0[33] != s.Q1[33]
{ continue; }

                /* D9 */
                s.Q0[34] = RL!(md5_H!(s.Q0[33], s.Q0[32], s.Q0[31]) + s.Q0[30]
                    + s.X0[ 8] + 0x8771f681, 11) + s.Q0[33];
                s.Q1[34] = RL!(md5_H!(s.Q1[33], s.Q1[32], s.Q1[31]) + s.Q1[30]
                    + s.X1[ 8] + 0x8771f681, 11) + s.Q1[33];
                if s.Q0[34] != s.Q1[34]
{ continue; }

                /* C9 */
                s.Q0[35] = RL!(md5_H!(s.Q0[34], s.Q0[33], s.Q0[32]) + s.Q0[31]
                    + s.X0[11] + 0x6d9d6122, 16) + s.Q0[34];
                s.Q1[35] = RL!(md5_H!(s.Q1[34], s.Q1[33], s.Q1[32]) + s.Q1[31]
                    + s.X1[11] + 0x6d9d6122, 16) + s.Q1[34];
                if (s.Q0[35] ^ s.Q1[35]) != 0x80000000
{ continue; }

                /* B9 */
                s.Q0[36] = RL!(md5_H!(s.Q0[35], s.Q0[34], s.Q0[33]) + s.Q0[32]
                    + s.X0[14] + 0xfde5380c, 23) + s.Q0[35];
                s.Q1[36] = RL!(md5_H!(s.Q1[35], s.Q1[34], s.Q1[33]) + s.Q1[32]
                    + s.X1[14] + 0xfde5380c, 23) + s.Q1[35];
                if (s.Q0[36] ^ s.Q1[36]) != 0x80000000
{ continue; }

                /* A10 */
                s.Q0[37] = RL!(md5_H!(s.Q0[36], s.Q0[35], s.Q0[34]) + s.Q0[33]
                    + s.X0[ 1] + 0xa4beea44,  4) + s.Q0[36];
                s.Q1[37] = RL!(md5_H!(s.Q1[36], s.Q1[35], s.Q1[34]) + s.Q1[33]
                    + s.X1[ 1] + 0xa4beea44,  4) + s.Q1[36];
                if (s.Q0[37] ^ s.Q1[37]) != 0x80000000
{ continue; }

                /* D10 */
                s.Q0[38] = RL!(md5_H!(s.Q0[37], s.Q0[36], s.Q0[35]) + s.Q0[34]
                    + s.X0[ 4] + 0x4bdecfa9, 11) + s.Q0[37];
                s.Q1[38] = RL!(md5_H!(s.Q1[37], s.Q1[36], s.Q1[35]) + s.Q1[34]
                    + s.X1[ 4] + 0x4bdecfa9, 11) + s.Q1[37];
                if (s.Q0[38] ^ s.Q1[38]) != 0x80000000
{ continue; }

                /* C10 */
                s.Q0[39] = RL!(md5_H!(s.Q0[38], s.Q0[37], s.Q0[36]) + s.Q0[35]
                    + s.X0[ 7] + 0xf6bb4b60, 16) + s.Q0[38];
                s.Q1[39] = RL!(md5_H!(s.Q1[38], s.Q1[37], s.Q1[36]) + s.Q1[35]
                    + s.X1[ 7] + 0xf6bb4b60, 16) + s.Q1[38];
                if (s.Q0[39] ^ s.Q1[39]) != 0x80000000
{ continue; }

                /* B10 */
                s.Q0[40] = RL!(md5_H!(s.Q0[39], s.Q0[38], s.Q0[37]) + s.Q0[36]
                    + s.X0[10] + 0xbebfbc70, 23) + s.Q0[39];
                s.Q1[40] = RL!(md5_H!(s.Q1[39], s.Q1[38], s.Q1[37]) + s.Q1[36]
                    + s.X1[10] + 0xbebfbc70, 23) + s.Q1[39];
                if (s.Q0[40] ^ s.Q1[40]) != 0x80000000
{ continue; }

                /* A11 */
                s.Q0[41] = RL!(md5_H!(s.Q0[40], s.Q0[39], s.Q0[38]) + s.Q0[37]
                    + s.X0[13] + 0x289b7ec6,  4) + s.Q0[40];
                s.Q1[41] = RL!(md5_H!(s.Q1[40], s.Q1[39], s.Q1[38]) + s.Q1[37]
                    + s.X1[13] + 0x289b7ec6,  4) + s.Q1[40];
                if (s.Q0[41] ^ s.Q1[41]) != 0x80000000
{ continue; }

                /* D11 */
                s.Q0[42] = RL!(md5_H!(s.Q0[41], s.Q0[40], s.Q0[39]) + s.Q0[38]
                    + s.X0[ 0] + 0xeaa127fa, 11) + s.Q0[41];
                s.Q1[42] = RL!(md5_H!(s.Q1[41], s.Q1[40], s.Q1[39]) + s.Q1[38]
                    + s.X1[ 0] + 0xeaa127fa, 11) + s.Q1[41];
                if (s.Q0[42] ^ s.Q1[42]) != 0x80000000
{ continue; }

                /* C11 */
                s.Q0[43] = RL!(md5_H!(s.Q0[42], s.Q0[41], s.Q0[40]) + s.Q0[39]
                    + s.X0[ 3] + 0xd4ef3085, 16) + s.Q0[42];
                s.Q1[43] = RL!(md5_H!(s.Q1[42], s.Q1[41], s.Q1[40]) + s.Q1[39]
                    + s.X1[ 3] + 0xd4ef3085, 16) + s.Q1[42];
                if (s.Q0[43] ^ s.Q1[43]) != 0x80000000
{ continue; }

                /* B11 */
                s.Q0[44] = RL!(md5_H!(s.Q0[43], s.Q0[42], s.Q0[41]) + s.Q0[40]
                    + s.X0[ 6] + 0x04881d05, 23) + s.Q0[43];
                s.Q1[44] = RL!(md5_H!(s.Q1[43], s.Q1[42], s.Q1[41]) + s.Q1[40]
                    + s.X1[ 6] + 0x04881d05, 23) + s.Q1[43];
                if (s.Q0[44] ^ s.Q1[44]) != 0x80000000
{ continue; }

                /* A12 */
                s.Q0[45] = RL!(md5_H!(s.Q0[44], s.Q0[43], s.Q0[42]) + s.Q0[41]
                    + s.X0[ 9] + 0xd9d4d039,  4) + s.Q0[44];
                s.Q1[45] = RL!(md5_H!(s.Q1[44], s.Q1[43], s.Q1[42]) + s.Q1[41]
                    + s.X1[ 9] + 0xd9d4d039,  4) + s.Q1[44];
                if (s.Q0[45] ^ s.Q1[45]) != 0x80000000
{ continue; }

                /* D12 */
                s.Q0[46] = RL!(md5_H!(s.Q0[45], s.Q0[44], s.Q0[43]) + s.Q0[42]
                    + s.X0[12] + 0xe6db99e5, 11) + s.Q0[45];
                s.Q1[46] = RL!(md5_H!(s.Q1[45], s.Q1[44], s.Q1[43]) + s.Q1[42]
                    + s.X1[12] + 0xe6db99e5, 11) + s.Q1[45];
                if (s.Q0[46] ^ s.Q1[46]) != 0x80000000
{ continue; }

                /* C12 */
                s.Q0[47] = RL!(md5_H!(s.Q0[46], s.Q0[45], s.Q0[44]) + s.Q0[43]
                    + s.X0[15] + 0x1fa27cf8, 16) + s.Q0[46];
                s.Q1[47] = RL!(md5_H!(s.Q1[46], s.Q1[45], s.Q1[44]) + s.Q1[43]
                    + s.X1[15] + 0x1fa27cf8, 16) + s.Q1[46];
                if (s.Q0[47] ^ s.Q1[47]) != 0x80000000
{ continue; }

                /* B12 */
                s.Q0[48] = RL!(md5_H!(s.Q0[47], s.Q0[46], s.Q0[45]) + s.Q0[44]
                    + s.X0[ 2] + 0xc4ac5665, 23) + s.Q0[47];
                if (s.Q0[48] ^ s.Q0[46]) & 0x80000000 != 0
{ continue; }
                s.Q1[48] = RL!(md5_H!(s.Q1[47], s.Q1[46], s.Q1[45]) + s.Q1[44]
                    + s.X1[ 2] + 0xc4ac5665, 23) + s.Q1[47];
                if (s.Q0[48] ^ s.Q1[48]) != 0x80000000
{ continue; }

                /* A13 */
                s.Q0[49] = RL!(md5_I!(s.Q0[48], s.Q0[47], s.Q0[46]) + s.Q0[45]
                    + s.X0[ 0] + 0xf4292244,  6) + s.Q0[48];
                if (s.Q0[49] ^ s.Q0[47]) & 0x80000000 != 0
{ continue; }
                s.Q1[49] = RL!(md5_I!(s.Q1[48], s.Q1[47], s.Q1[46]) + s.Q1[45]
                    + s.X1[ 0] + 0xf4292244,  6) + s.Q1[48];
                if (s.Q0[49] ^ s.Q1[49]) != 0x80000000
{ continue; }

                /* D13 */
                s.Q0[50] = RL!(md5_I!(s.Q0[49], s.Q0[48], s.Q0[47]) + s.Q0[46]
                    + s.X0[ 7] + 0x432aff97, 10) + s.Q0[49];
                if !((s.Q0[50] ^ s.Q0[48]) & 0x80000000) != 0
{ continue; }
                s.Q1[50] = RL!(md5_I!(s.Q1[49], s.Q1[48], s.Q1[47]) + s.Q1[46]
                    + s.X1[ 7] + 0x432aff97, 10) + s.Q1[49];
                if (s.Q0[50] ^ s.Q1[50]) != 0x80000000
{ continue; }

                /* C13 */
                s.Q0[51] = RL!(md5_I!(s.Q0[50], s.Q0[49], s.Q0[48]) + s.Q0[47]
                    + s.X0[14] + 0xab9423a7, 15) + s.Q0[50];
                if (s.Q0[51] ^ s.Q0[49]) & 0x80000000 != 0
{ continue; }
                s.Q1[51] = RL!(md5_I!(s.Q1[50], s.Q1[49], s.Q1[48]) + s.Q1[47]
                    + s.X1[14] + 0xab9423a7, 15) + s.Q1[50];
                if (s.Q0[51] ^ s.Q1[51]) != 0x80000000
{ continue; }

                /* B13 */
                s.Q0[52] = RL!(md5_I!(s.Q0[51], s.Q0[50], s.Q0[49]) + s.Q0[48]
                    + s.X0[ 5] + 0xfc93a039, 21) + s.Q0[51];
                if (s.Q0[52] ^ s.Q0[50]) & 0x80000000 != 0
{ continue; }
                s.Q1[52] = RL!(md5_I!(s.Q1[51], s.Q1[50], s.Q1[49]) + s.Q1[48]
                    + s.X1[ 5] + 0xfc93a039, 21) + s.Q1[51];
                if (s.Q0[52] ^ s.Q1[52]) != 0x80000000
{ continue; }

                /* A14 */
                s.Q0[53] = RL!(md5_I!(s.Q0[52], s.Q0[51], s.Q0[50]) + s.Q0[49]
                    + s.X0[12] + 0x655b59c3,  6) + s.Q0[52];
                if (s.Q0[53] ^ s.Q0[51]) & 0x80000000 != 0
{ continue; }
                s.Q1[53] = RL!(md5_I!(s.Q1[52], s.Q1[51], s.Q1[50]) + s.Q1[49]
                    + s.X1[12] + 0x655b59c3,  6) + s.Q1[52];
                if (s.Q0[53] ^ s.Q1[53]) != 0x80000000
{ continue; }

                /* D14 */
                s.Q0[54] = RL!(md5_I!(s.Q0[53], s.Q0[52], s.Q0[51]) + s.Q0[50]
                    + s.X0[ 3] + 0x8f0ccc92, 10) + s.Q0[53];
                if (s.Q0[54] ^ s.Q0[52]) & 0x80000000 != 0
{ continue; }
                s.Q1[54] = RL!(md5_I!(s.Q1[53], s.Q1[52], s.Q1[51]) + s.Q1[50]
                    + s.X1[ 3] + 0x8f0ccc92, 10) + s.Q1[53];
                if (s.Q0[54] ^ s.Q1[54]) != 0x80000000
{ continue; }

                /* C14 */
                s.Q0[55] = RL!(md5_I!(s.Q0[54], s.Q0[53], s.Q0[52]) + s.Q0[51]
                    + s.X0[10] + 0xffeff47d, 15) + s.Q0[54];
                if (s.Q0[55] ^ s.Q0[53]) & 0x80000000 != 0
{ continue; }
                s.Q1[55] = RL!(md5_I!(s.Q1[54], s.Q1[53], s.Q1[52]) + s.Q1[51]
                    + s.X1[10] + 0xffeff47d, 15) + s.Q1[54];
                if (s.Q0[55] ^ s.Q1[55]) != 0x80000000
{ continue; }

                /* B14 */
                s.Q0[56] = RL!(md5_I!(s.Q0[55], s.Q0[54], s.Q0[53]) + s.Q0[52]
                    + s.X0[ 1] + 0x85845dd1, 21) + s.Q0[55];
                if (s.Q0[56] ^ s.Q0[54]) & 0x80000000 != 0
{ continue; }
                s.Q1[56] = RL!(md5_I!(s.Q1[55], s.Q1[54], s.Q1[53]) + s.Q1[52]
                    + s.X1[ 1] + 0x85845dd1, 21) + s.Q1[55];
                if (s.Q0[56] ^ s.Q1[56]) != 0x80000000
{ continue; }

                /* A15 */
                s.Q0[57] = RL!(md5_I!(s.Q0[56], s.Q0[55], s.Q0[54]) + s.Q0[53]
                    + s.X0[ 8] + 0x6fa87e4f,  6) + s.Q0[56];
                if (s.Q0[57] ^ s.Q0[55]) & 0x80000000 != 0
{ continue; }
                s.Q1[57] = RL!(md5_I!(s.Q1[56], s.Q1[55], s.Q1[54]) + s.Q1[53]
                    + s.X1[ 8] + 0x6fa87e4f,  6) + s.Q1[56];
                if (s.Q0[57] ^ s.Q1[57]) != 0x80000000
{ continue; }

                /* D15 */
                s.Q0[58] = RL!(md5_I!(s.Q0[57], s.Q0[56], s.Q0[55]) + s.Q0[54]
                    + s.X0[15] + 0xfe2ce6e0, 10) + s.Q0[57];
                if (s.Q0[58] ^ s.Q0[56]) & 0x80000000 != 0
{ continue; }
                s.Q1[58] = RL!(md5_I!(s.Q1[57], s.Q1[56], s.Q1[55]) + s.Q1[54]
                    + s.X1[15] + 0xfe2ce6e0, 10) + s.Q1[57];
                if (s.Q0[58] ^ s.Q1[58]) != 0x80000000
{ continue; }

                /* C15 */
                s.Q0[59] = RL!(md5_I!(s.Q0[58], s.Q0[57], s.Q0[56]) + s.Q0[55]
                    + s.X0[ 6] + 0xa3014314, 15) + s.Q0[58];
                if (s.Q0[59] ^ s.Q0[57]) & 0x80000000 != 0
{ continue; }
                s.Q1[59] = RL!(md5_I!(s.Q1[58], s.Q1[57], s.Q1[56]) + s.Q1[55]
                    + s.X1[ 6] + 0xa3014314, 15) + s.Q1[58];
                if (s.Q0[59] ^ s.Q1[59]) != 0x80000000
{ continue; }

                /* B15 */
                s.Q0[60] = RL!(md5_I!(s.Q0[59], s.Q0[58], s.Q0[57]) + s.Q0[56]
                    + s.X0[13] + 0x4e0811a1, 21) + s.Q0[59];
                if s.Q0[60] & 0x02000000 != 0
{ continue; }
                s.Q1[60] = RL!(md5_I!(s.Q1[59], s.Q1[58], s.Q1[57]) + s.Q1[56]
                    + s.X1[13] + 0x4e0811a1, 21) + s.Q1[59];
                if (s.Q0[60] ^ s.Q1[60]) != 0x80000000
{ continue; }

                /* A16 */
                s.Q0[61] = RL!(md5_I!(s.Q0[60], s.Q0[59], s.Q0[58]) + s.Q0[57]
                    + s.X0[ 4] + 0xf7537e82,  6) + s.Q0[60];
                s.A0 = IV[0] + s.Q0[61];
                s.Q1[61] = RL!(md5_I!(s.Q1[60], s.Q1[59], s.Q1[58]) + s.Q1[57]
                    + s.X1[ 4] + 0xf7537e82,  6) + s.Q1[60];
                s.A1 = IV[0] + s.Q1[61];
                if (s.A0 ^ s.A1) != 0x80000000
{ continue; }

                /* D16 */
                s.Q0[62] = RL!(md5_I!(s.Q0[61], s.Q0[60], s.Q0[59]) + s.Q0[58]
                    + s.X0[11] + 0xbd3af235, 10) + s.Q0[61];
                s.D0 = IV[3] + s.Q0[62];
                if s.D0 & 0x02000000 != 0
{ continue; }
                s.Q1[62] = RL!(md5_I!(s.Q1[61], s.Q1[60], s.Q1[59]) + s.Q1[58]
                    + s.X1[11] + 0xbd3af235, 10) + s.Q1[61];
                s.D1 = IV[3] + s.Q1[62];
                if (s.D0 - s.D1) != 0x7e000000
{ continue; }

                /* C16 */
                s.Q0[63] = RL!(md5_I!(s.Q0[62], s.Q0[61], s.Q0[60]) + s.Q0[59]
                    + s.X0[ 2] + 0x2ad7d2bb, 15) + s.Q0[62];
                s.C0 = IV[2] + s.Q0[63];
                if (s.C0 & 0x86000000) != ((s.D0 & 0x80000000) | 0x02000000)
{ continue; }
                s.Q1[63] = RL!(md5_I!(s.Q1[62], s.Q1[61], s.Q1[60]) + s.Q1[59]
                    + s.X1[ 2] + 0x2ad7d2bb, 15) + s.Q1[62];
                s.C1 = IV[2] + s.Q1[63];
                if (s.C0 - s.C1) != 0x7e000000
{ continue; }

                /* B16 */
                s.Q0[64] = RL!(md5_I!(s.Q0[63], s.Q0[62], s.Q0[61]) + s.Q0[60]
                    + s.X0[ 9] + 0xeb86d391, 21) + s.Q0[63];
                s.B0 = IV[1] + s.Q0[64];
                if (s.B0 & 0x86000020) != (s.C0 & 0x80000000)
{ continue; }
                s.Q1[64] = RL!(md5_I!(s.Q1[63], s.Q1[62], s.Q1[61]) + s.Q1[60]
                    + s.X1[ 9] + 0xeb86d391, 21) + s.Q1[63];
                s.B1 = IV[1] + s.Q1[64];
                if (s.B0 - s.B1) != 0x7e000000
{ continue; }
            
                goto_flag = 1;
                break;
            }
            if cnt == (LOOP_12 - 1) && goto_flag == 0
            {
                /* return to first loop */
                continue;
            }
            s.ct1 = ct1;
            s.ct2 = cnt >> 20;
            return 0; 
        }
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

fn block2() -> i32
{
    let mut rng = rand::thread_rng();
    let mut goto_flag = 0;
    let mut cnt:u32 = 0;
    let mut ct3: i32 = 0;
    let mut it: i32 = 0;

    unsafe
    {
        /* block2_again */
        loop
        {
            loop
            {
                /* A1 */
                s.Q0[ 1] = (rng.gen::<u32>() | 0x84200000) & !0x0a000820;
                s.Q1[ 1] = s.Q0[ 1] - 0x7e000000;
                s.X0[16] = RR!(s.Q0[ 1] - s.B0,  7) - md5_F!(s.B0, s.C0, s.D0) - s.A0 - 0xd76aa478;
                s.X1[16] = RR!(s.Q1[ 1] - s.B1,  7) - md5_F!(s.B1, s.C1, s.D1) - s.A1 - 0xd76aa478;
                if s.X0[16] != s.X1[16]
                    {continue;}
                break;
            }

            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* D1 */
                s.Q0[ 2] = (rng.gen::<u32>() | 0x8c000800) & !(0x02208026 | 0x701f10c0);
                s.Q0[ 2] |= (s.Q0[ 1] & 0x701f10c0);
                s.Q1[ 2] = s.Q0[ 2] - 0x7dffffe0;
        
                s.X0[17] = RR!(s.Q0[ 2] - s.Q0[ 1], 12) - md5_F!(s.Q0[ 1], s.B0, s.C0)
                        - s.D0 - 0xe8c7b756;
                s.X1[17] = RR!(s.Q1[ 2] - s.Q1[ 1], 12) - md5_F!(s.Q1[ 1], s.B1, s.C1)
                        - s.D1 - 0xe8c7b756;
                if s.X0[17] != s.X1[17]
                        {continue;}
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
            {
                continue;
            }
            
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* C1 */
                s.Q0[ 3] = (rng.gen::<u32>() | 0xbe1f0966) & !(0x40201080 | 0x00000018);
                s.Q0[ 3] |= (s.Q0[ 2] & 0x00000018);
                s.Q1[ 3] = s.Q0[ 3] - 0x7dfef7e0;
        
                s.X0[18] = RR!(s.Q0[ 3] - s.Q0[ 2], 17) - md5_F!(s.Q0[ 2], s.Q0[ 1], s.B0)
                        - s.C0 - 0x242070db;
                s.X1[18] = RR!(s.Q1[ 3] - s.Q1[ 2], 17) - md5_F!(s.Q1[ 2], s.Q1[ 1], s.B1)
                        - s.C1 - 0x242070db;
                if s.X0[18] != s.X1[18]
{ continue; }
                goto_flag = 1;
                break;
            }

            if cnt == 9 && goto_flag == 0
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
                    if (it >= 10000) {
                        return -1;
                    }
                }
                continue;
            }
            
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* B1 */
                s.Q0[ 4] = (rng.gen::<u32>() | 0xba040010) & !(0x443b19ee | 0x00000601);
                s.Q0[ 4] |= (s.Q0[ 3] & 0x00000601);
                s.Q1[ 4] = s.Q0[ 4] - 0x7dffffe2;
        
                s.X0[19] = RR!(s.Q0[ 4] - s.Q0[ 3], 22) - md5_F!(s.Q0[ 3], s.Q0[ 2], s.Q0[ 1])
                        - s.B0 - 0xc1bdceee;
                s.X1[19] = RR!(s.Q1[ 4] - s.Q1[ 3], 22) - md5_F!(s.Q1[ 3], s.Q1[ 2], s.Q1[ 1])
                        - s.B1 - 0xc1bdceee;
                if s.X0[19] != s.X1[19]
{ continue; }
                goto_flag = 1;
                break;
            }

            if cnt == 9 && goto_flag == 0
            {
                continue;
            }
            
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* A2 */
                s.Q0[ 5] = (rng.gen::<u32>() | 0x482f0e50) & !0xb41011af;
                s.Q1[ 5] = s.Q0[ 5] - 0x7ffffcbf;
        
                        s.X0[20] = RR!(s.Q0[ 5] - s.Q0[ 4],  7) - md5_F!(s.Q0[ 4], s.Q0[ 3], s.Q0[ 2])
                                - s.Q0[ 1] - 0xf57c0faf;
                        s.X1[20] = RR!(s.Q1[ 5] - s.Q1[ 4],  7) - md5_F!(s.Q1[ 4], s.Q1[ 3], s.Q1[ 2])
                                - s.Q1[ 1] - 0xf57c0faf;
                        if (s.X0[20] ^ s.X1[20]) != 0x80000000
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* D2 */
                s.Q0[ 6] = (rng.gen::<u32>() | 0x04220c56) & !0x9a1113a9;
                s.Q1[ 6] = s.Q0[ 6] - 0x80110000;
        
                        s.X0[21] = RR!(s.Q0[ 6] - s.Q0[ 5], 12) - md5_F!(s.Q0[ 5], s.Q0[ 4], s.Q0[ 3])
                                - s.Q0[ 2] - 0x4787c62a;
                        s.X1[21] = RR!(s.Q1[ 6] - s.Q1[ 5], 12) - md5_F!(s.Q1[ 5], s.Q1[ 4], s.Q1[ 3])
                                - s.Q1[ 2] - 0x4787c62a;
                        if s.X0[21] != s.X1[21]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* C2 */
                s.Q0[ 7] = (rng.gen::<u32>() | 0x96011e01) & !(0x083201c0 | 0x01808000);
                s.Q0[ 7] |= (s.Q0[ 6] & 0x01808000);
                s.Q1[ 7] = s.Q0[ 7] - 0x88000040;
        
                s.X0[22] = RR!(s.Q0[ 7] - s.Q0[ 6], 17) - md5_F!(s.Q0[ 6], s.Q0[ 5], s.Q0[ 4])
                    - s.Q0[ 3] - 0xa8304613;
                s.X1[22] = RR!(s.Q1[ 7] - s.Q1[ 6], 17) - md5_F!(s.Q1[ 6], s.Q1[ 5], s.Q1[ 4])
                    - s.Q1[ 3] - 0xa8304613;
                if s.X0[22] != s.X1[22]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* B2 */
                s.Q0[ 8] = (rng.gen::<u32>() | 0x843283c0) & !(0x1b810001 | 0x00000002);
                s.Q0[ 8] |= (s.Q0[ 7] & 0x00000002);
                s.Q1[ 8] = s.Q0[ 8] - 0x80818000;
        
                s.X0[23] = RR!(s.Q0[ 8] - s.Q0[ 7], 22) - md5_F!(s.Q0[ 7], s.Q0[ 6], s.Q0[ 5])
                    - s.Q0[ 4] - 0xfd469501;
                s.X1[23] = RR!(s.Q1[ 8] - s.Q1[ 7], 22) - md5_F!(s.Q1[ 7], s.Q1[ 6], s.Q1[ 5])
                    - s.Q1[ 4] - 0xfd469501;
                if s.X0[23] != s.X1[23]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
            
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* A3 */
                s.Q0[ 9] = (rng.gen::<u32>() | 0x9c0101c1) & !(0x03828202 | 0x00001000);
                s.Q0[ 9] |= (s.Q0[ 8] & 0x00001000);
                s.Q1[ 9] = s.Q0[ 9] - 0x7fffffbf;
        
                s.X0[24] = RR!(s.Q0[ 9] - s.Q0[ 8],  7) - md5_F!(s.Q0[ 8], s.Q0[ 7], s.Q0[ 6])
                    - s.Q0[ 5] - 0x698098d8;
                s.X1[24] = RR!(s.Q1[ 9] - s.Q1[ 8],  7) - md5_F!(s.Q1[ 8], s.Q1[ 7], s.Q1[ 6])
                    - s.Q1[ 5] - 0x698098d8;
                if s.X0[24] != s.X1[24]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* D3 */
                s.Q0[10] = (rng.gen::<u32>() | 0x878383c0) & !0x00041003;
                s.Q1[10] = s.Q0[10] - 0x7ffff000;
        
                s.X0[25] = RR!(s.Q0[10] - s.Q0[ 9], 12) - md5_F!(s.Q0[ 9], s.Q0[ 8], s.Q0[ 7])
                    - s.Q0[ 6] - 0x8b44f7af;
                s.X1[25] = RR!(s.Q1[10] - s.Q1[ 9], 12) - md5_F!(s.Q1[ 9], s.Q1[ 8], s.Q1[ 7])
                    - s.Q1[ 6] - 0x8b44f7af;
                if s.X0[25] != s.X1[25]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* C3 */
                s.Q0[11] = (rng.gen::<u32>() | 0x800583c3) & !(0x00021000 | 0x00086000);
                s.Q0[11] |= (s.Q0[10] & 0x00086000);
                s.Q1[11] = s.Q0[11] - 0x80000000;
        
                s.X0[26] = RR!(s.Q0[11] - s.Q0[10], 17) - md5_F!(s.Q0[10], s.Q0[ 9], s.Q0[ 8])
                    - s.Q0[ 7] - 0xffff5bb1;
                s.X1[26] = RR!(s.Q1[11] - s.Q1[10], 17) - md5_F!(s.Q1[10], s.Q1[ 9], s.Q1[ 8])
                    - s.Q1[ 7] - 0xffff5bb1;
                if s.X0[26] != s.X1[26]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* B3 */
                s.Q0[12] = (rng.gen::<u32>() | 0x80081080) & !(0x0007e000 | 0x7f000000);
                s.Q0[12] |= (s.Q0[11] & 0x7f000000);
                s.Q1[12] = s.Q0[12] - 0x80002080;
        
                s.X0[27] = RR!(s.Q0[12] - s.Q0[11], 22) - md5_F!(s.Q0[11], s.Q0[10], s.Q0[ 9])
                    - s.Q0[ 8] - 0x895cd7be;
                s.X1[27] = RR!(s.Q1[12] - s.Q1[11], 22) - md5_F!(s.Q1[11], s.Q1[10], s.Q1[ 9])
                    - s.Q1[ 8] - 0x895cd7be;
                if (s.X0[27] ^ s.X1[27]) != 0x00008000
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* A4 */
                s.Q0[13] = (rng.gen::<u32>() | 0x3f0fe008) & !0x80000080;
                s.Q1[13] = s.Q0[13] - 0x7f000000;
        
                s.X0[28] = RR!(s.Q0[13] - s.Q0[12],  7) - md5_F!(s.Q0[12], s.Q0[11], s.Q0[10])
                    - s.Q0[ 9] - 0x6b901122;
                s.X1[28] = RR!(s.Q1[13] - s.Q1[12],  7) - md5_F!(s.Q1[12], s.Q1[11], s.Q1[10])
                    - s.Q1[ 9] - 0x6b901122;
                if s.X0[28] != s.X1[28]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* D4 */
                s.Q0[14] = (rng.gen::<u32>() | 0x400be088) & !0xbf040000;
                s.Q1[14] = s.Q0[14] - 0x80000000;
        
                s.X0[29] = RR!(s.Q0[14] - s.Q0[13], 12) - md5_F!(s.Q0[13], s.Q0[12], s.Q0[11])
                    - s.Q0[10] - 0xfd987193;
                s.X1[29] = RR!(s.Q1[14] - s.Q1[13], 12) - md5_F!(s.Q1[13], s.Q1[12], s.Q1[11])
                    - s.Q1[10] - 0xfd987193;
                if s.X0[29] != s.X1[29]
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..10
            {
                cnt += 1;
                goto_flag = 0;
                /* C4 */
                s.Q0[15] = (rng.gen::<u32>() | 0x7d000000) & !0x82008008;
                s.Q1[15] = s.Q0[15] - 0x7fff7ff8;
        
                s.X0[30] = RR!(s.Q0[15] - s.Q0[14], 17) - md5_F!(s.Q0[14], s.Q0[13], s.Q0[12])
                    - s.Q0[11] - 0xa679438e;
                s.X1[30] = RR!(s.Q1[15] - s.Q1[14], 17) - md5_F!(s.Q1[14], s.Q1[13], s.Q1[12])
                    - s.Q1[11] - 0xa679438e;
                if (s.X0[30] ^ s.X1[30]) != 0x80000000
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == 9 && goto_flag == 0
                { continue; }
        
            cnt = 0;
            for i in 0..LOOP_21
            {
                cnt += 1;
                goto_flag = 0;
                /* B4 */
                s.Q0[16] = (rng.gen::<u32>() | 0x20000000) & !0x80000000;
                s.Q1[16] = s.Q0[16] - 0xa0000000;
        
                s.X0[31] = RR!(s.Q0[16] - s.Q0[15], 22) - md5_F!(s.Q0[15], s.Q0[14], s.Q0[13])
                    - s.Q0[12] - 0x49b40821;
                s.X1[31] = RR!(s.Q1[16] - s.Q1[15], 22) - md5_F!(s.Q1[15], s.Q1[14], s.Q1[13])
                    - s.Q1[12] - 0x49b40821;
                if s.X0[31] != s.X1[31]
{ continue; }
        
                        /* A5 */
                        s.Q0[17] = RL!(md5_G!(s.Q0[16], s.Q0[15], s.Q0[14]) + s.Q0[13]
                                + s.X0[17] + 0xf61e2562,  5) + s.Q0[16];
                        if (s.Q0[17] & 0x80028008) != (s.Q0[16] & 0x00008008)
{ continue; }
                        s.Q1[17] = RL!(md5_G!(s.Q1[16], s.Q1[15], s.Q1[14]) + s.Q1[13]
                                + s.X1[17] + 0xf61e2562,  5) + s.Q1[16];
                        if (s.Q0[17] ^ s.Q1[17]) != 0x80000000
{ continue; }
        
                        /* D5 */
                        s.Q0[18] = RL!(md5_G!(s.Q0[17], s.Q0[16], s.Q0[15]) + s.Q0[14]
                                + s.X0[22] + 0xc040b340,  9) + s.Q0[17];
                        if (s.Q0[18] & 0xa0020000)
                                != ((s.Q0[17] & 0x20000000) | 0x00020000)
                        {
                                continue;
                        }
                        s.Q1[18] = RL!(md5_G!(s.Q1[17], s.Q1[16], s.Q1[15]) + s.Q1[14]
                                + s.X1[22] + 0xc040b340,  9) + s.Q1[17];
                        if (s.Q0[18] ^ s.Q1[18]) != 0x80000000
{ continue; }
        
                        /* C5 */
                        s.Q0[19] = RL!(md5_G!(s.Q0[18], s.Q0[17], s.Q0[16]) + s.Q0[15]
                                + s.X0[27] + 0x265e5a51, 14) + s.Q0[18];
                        if s.Q0[19] & 0x80020000 != 0
{ continue; }
                        s.Q1[19] = RL!(md5_G!(s.Q1[18], s.Q1[17], s.Q1[16]) + s.Q1[15]
                                + s.X1[27] + 0x265e5a51, 14) + s.Q1[18];
                        if (s.Q0[19] - s.Q1[19]) != 0x7ffe0000
{ continue; }
        
                        /* B5 */
                        s.Q0[20] = RL!(md5_G!(s.Q0[19], s.Q0[18], s.Q0[17]) + s.Q0[16]
                                + s.X0[16] + 0xe9b6c7aa, 20) + s.Q0[19];
                        if s.Q0[20] & 0x80000000 != 0
{ continue; }
                        s.Q1[20] = RL!(md5_G!(s.Q1[19], s.Q1[18], s.Q1[17]) + s.Q1[16]
                                + s.X1[16] + 0xe9b6c7aa, 20) + s.Q1[19];
                        if (s.Q0[20] ^ s.Q1[20]) != 0x80000000
{ continue; }
        
                        /* A6 */
                        s.Q0[21] = RL!(md5_G!(s.Q0[20], s.Q0[19], s.Q0[18]) + s.Q0[17]
                                + s.X0[21] + 0xd62f105d,  5) + s.Q0[20];
                        if (s.Q0[21] & 0x80020000) != (s.Q0[20] & 0x00020000)
{ continue; }
                        s.Q1[21] = RL!(md5_G!(s.Q1[20], s.Q1[19], s.Q1[18]) + s.Q1[17]
                                + s.X1[21] + 0xd62f105d,  5) + s.Q1[20];
                        if (s.Q0[21] ^ s.Q1[21]) != 0x80000000
{ continue; }
                goto_flag = 1;
                break;
            }
            
            if cnt == (LOOP_21 - 1) && goto_flag == 0
                { continue; }
        
            ct3 += 1;
            cnt = 0;
            for i in 0..LOOP_22
            {
                cnt += 1;
                goto_flag = 0;
        
                        /* B4 */
                        s.Q0[16] ^= mask22[rng.gen::<usize>() % 30];
                        s.Q1[16] = s.Q0[16] - 0xa0000000;
        
                        s.X0[31] = RR!(s.Q0[16] - s.Q0[15], 22) - md5_F!(s.Q0[15], s.Q0[14], s.Q0[13])
                                - s.Q0[12] - 0x49b40821;
                        s.X1[31] = RR!(s.Q1[16] - s.Q1[15], 22) - md5_F!(s.Q1[15], s.Q1[14], s.Q1[13])
                                - s.Q1[12] - 0x49b40821;
                        if s.X0[31] != s.X1[31]
{ continue; }
        
                /* A5 */
                s.Q0[17] = RL!(md5_G!(s.Q0[16], s.Q0[15], s.Q0[14]) + s.Q0[13]
                    + s.X0[17] + 0xf61e2562,  5) + s.Q0[16];
                if (s.Q0[17] & 0x80028008) != (s.Q0[16] & 0x00008008)
{ continue; }
                s.Q1[17] = RL!(md5_G!(s.Q1[16], s.Q1[15], s.Q1[14]) + s.Q1[13]
                    + s.X1[17] + 0xf61e2562,  5) + s.Q1[16];
                if (s.Q0[17] ^ s.Q1[17]) != 0x80000000
{ continue; }
        
                /* D5 */
                s.Q0[18] = RL!(md5_G!(s.Q0[17], s.Q0[16], s.Q0[15]) + s.Q0[14]
                    + s.X0[22] + 0xc040b340,  9) + s.Q0[17];
                if (s.Q0[18] & 0xa0020000)
                    != (s.Q0[17] & 0x20000000) | 0x00020000
                {
                    continue;
                }
                s.Q1[18] = RL!(md5_G!(s.Q1[17], s.Q1[16], s.Q1[15]) + s.Q1[14]
                    + s.X1[22] + 0xc040b340,  9) + s.Q1[17];
                if (s.Q0[18] ^ s.Q1[18]) != 0x80000000
{ continue; }
        
                /* C5 */
                s.Q0[19] = RL!(md5_G!(s.Q0[18], s.Q0[17], s.Q0[16]) + s.Q0[15]
                    + s.X0[27] + 0x265e5a51, 14) + s.Q0[18];
                if s.Q0[19] & 0x80020000 != 0
{ continue; }
                s.Q1[19] = RL!(md5_G!(s.Q1[18], s.Q1[17], s.Q1[16]) + s.Q1[15]
                    + s.X1[27] + 0x265e5a51, 14) + s.Q1[18];
                if (s.Q0[19] - s.Q1[19]) != 0x7ffe0000
{ continue; }
        
                /* B5 */
                s.Q0[20] = RL!(md5_G!(s.Q0[19], s.Q0[18], s.Q0[17]) + s.Q0[16]
                    + s.X0[16] + 0xe9b6c7aa, 20) + s.Q0[19];
                if s.Q0[20] & 0x80000000 != 0
{ continue; }
                s.Q1[20] = RL!(md5_G!(s.Q1[19], s.Q1[18], s.Q1[17]) + s.Q1[16]
                    + s.X1[16] + 0xe9b6c7aa, 20) + s.Q1[19];
                if (s.Q0[20] ^ s.Q1[20]) != 0x80000000
{ continue; }
        
                /* A6 */
                s.Q0[21] = RL!(md5_G!(s.Q0[20], s.Q0[19], s.Q0[18]) + s.Q0[17]
                    + s.X0[21] + 0xd62f105d,  5) + s.Q0[20];
                if (s.Q0[21] & 0x80020000) != (s.Q0[20] & 0x00020000)
{ continue; }
                s.Q1[21] = RL!(md5_G!(s.Q1[20], s.Q1[19], s.Q1[18]) + s.Q1[17]
                    + s.X1[21] + 0xd62f105d,  5) + s.Q1[20];
                if (s.Q0[21] ^ s.Q1[21]) != 0x80000000
{ continue; }
        
                /* D6 */
                s.Q0[22] = RL!(md5_G!(s.Q0[21], s.Q0[20], s.Q0[19]) + s.Q0[18]
                    + s.X0[26] + 0x02441453,  9) + s.Q0[21];
                if s.Q0[22] & 0x80000000 != 0
{ continue; }
                s.Q1[22] = RL!(md5_G!(s.Q1[21], s.Q1[20], s.Q1[19]) + s.Q1[18]
                    + s.X1[26] + 0x02441453,  9) + s.Q1[21];
                if (s.Q0[22] ^ s.Q1[22]) != 0x80000000
{ continue; }
        
                /* C6 */
                s.Q0[23] = RL!(md5_G!(s.Q0[22], s.Q0[21], s.Q0[20]) + s.Q0[19]
                    + s.X0[31] + 0xd8a1e681, 14) + s.Q0[22];
                if s.Q0[23] & 0x80000000 != 0
{ continue; }
                s.Q1[23] = RL!(md5_G!(s.Q1[22], s.Q1[21], s.Q1[20]) + s.Q1[19]
                    + s.X1[31] + 0xd8a1e681, 14) + s.Q1[22];
                if s.Q0[23] != s.Q1[23]
{ continue; }
        
                /* B6 */
                s.Q0[24] = RL!(md5_G!(s.Q0[23], s.Q0[22], s.Q0[21]) + s.Q0[20]
                    + s.X0[20] + 0xe7d3fbc8, 20) + s.Q0[23];
                s.Q1[24] = RL!(md5_G!(s.Q1[23], s.Q1[22], s.Q1[21]) + s.Q1[20]
                    + s.X1[20] + 0xe7d3fbc8, 20) + s.Q1[23];
                if s.Q0[24] != s.Q1[24]
{ continue; }
        
                /* A7 */
                s.Q0[25] = RL!(md5_G!(s.Q0[24], s.Q0[23], s.Q0[22]) + s.Q0[21]
                    + s.X0[25] + 0x21e1cde6,  5) + s.Q0[24];
                s.Q1[25] = RL!(md5_G!(s.Q1[24], s.Q1[23], s.Q1[22]) + s.Q1[21]
                    + s.X1[25] + 0x21e1cde6,  5) + s.Q1[24];
                if s.Q0[25] != s.Q1[25]
{ continue; }
        
                /* D7 */
                s.Q0[26] = RL!(md5_G!(s.Q0[25], s.Q0[24], s.Q0[23]) + s.Q0[22]
                    + s.X0[30] + 0xc33707d6,  9) + s.Q0[25];
                s.Q1[26] = RL!(md5_G!(s.Q1[25], s.Q1[24], s.Q1[23]) + s.Q1[22]
                    + s.X1[30] + 0xc33707d6,  9) + s.Q1[25];
                if s.Q0[26] != s.Q1[26]
{ continue; }
        
                /* C7 */
                s.Q0[27] = RL!(md5_G!(s.Q0[26], s.Q0[25], s.Q0[24]) + s.Q0[23]
                    + s.X0[19] + 0xf4d50d87, 14) + s.Q0[26];
                s.Q1[27] = RL!(md5_G!(s.Q1[26], s.Q1[25], s.Q1[24]) + s.Q1[23]
                    + s.X1[19] + 0xf4d50d87, 14) + s.Q1[26];
                if s.Q0[27] != s.Q1[27]
{ continue; }
        
                /* B7 */
                s.Q0[28] = RL!(md5_G!(s.Q0[27], s.Q0[26], s.Q0[25]) + s.Q0[24]
                    + s.X0[24] + 0x455a14ed, 20) + s.Q0[27];
                s.Q1[28] = RL!(md5_G!(s.Q1[27], s.Q1[26], s.Q1[25]) + s.Q1[24]
                    + s.X1[24] + 0x455a14ed, 20) + s.Q1[27];
                if s.Q0[28] != s.Q1[28]
{ continue; }
        
                /* A8 */
                s.Q0[29] = RL!(md5_G!(s.Q0[28], s.Q0[27], s.Q0[26]) + s.Q0[25]
                    + s.X0[29] + 0xa9e3e905,  5) + s.Q0[28];
                s.Q1[29] = RL!(md5_G!(s.Q1[28], s.Q1[27], s.Q1[26]) + s.Q1[25]
                    + s.X1[29] + 0xa9e3e905,  5) + s.Q1[28];
                if s.Q0[29] != s.Q1[29]
{ continue; }
        
                /* D8 */
                s.Q0[30] = RL!(md5_G!(s.Q0[29], s.Q0[28], s.Q0[27]) + s.Q0[26]
                    + s.X0[18] + 0xfcefa3f8,  9) + s.Q0[29];
                s.Q1[30] = RL!(md5_G!(s.Q1[29], s.Q1[28], s.Q1[27]) + s.Q1[26]
                    + s.X1[18] + 0xfcefa3f8,  9) + s.Q1[29];
                if s.Q0[30] != s.Q1[30]
{ continue; }
        
                /* C8 */
                s.Q0[31] = RL!(md5_G!(s.Q0[30], s.Q0[29], s.Q0[28]) + s.Q0[27]
                    + s.X0[23] + 0x676f02d9, 14) + s.Q0[30];
                s.Q1[31] = RL!(md5_G!(s.Q1[30], s.Q1[29], s.Q1[28]) + s.Q1[27]
                    + s.X1[23] + 0x676f02d9, 14) + s.Q1[30];
                if s.Q0[31] != s.Q1[31]
{ continue; }
        
                /* B8 */
                s.Q0[32] = RL!(md5_G!(s.Q0[31], s.Q0[30], s.Q0[29]) + s.Q0[28]
                    + s.X0[28] + 0x8d2a4c8a, 20) + s.Q0[31];
                s.Q1[32] = RL!(md5_G!(s.Q1[31], s.Q1[30], s.Q1[29]) + s.Q1[28]
                    + s.X1[28] + 0x8d2a4c8a, 20) + s.Q1[31];
                if s.Q0[32] != s.Q1[32]
{ continue; }
        
                /* A9 */
                s.Q0[33] = RL!(md5_H!(s.Q0[32], s.Q0[31], s.Q0[30]) + s.Q0[29]
                    + s.X0[21] + 0xfffa3942,  4) + s.Q0[32];
                s.Q1[33] = RL!(md5_H!(s.Q1[32], s.Q1[31], s.Q1[30]) + s.Q1[29]
                    + s.X1[21] + 0xfffa3942,  4) + s.Q1[32];
                if s.Q0[33] != s.Q1[33]
{ continue; }
        
                /* D9 */
                s.Q0[34] = RL!(md5_H!(s.Q0[33], s.Q0[32], s.Q0[31]) + s.Q0[30]
                    + s.X0[24] + 0x8771f681, 11) + s.Q0[33];
                s.Q1[34] = RL!(md5_H!(s.Q1[33], s.Q1[32], s.Q1[31]) + s.Q1[30]
                    + s.X1[24] + 0x8771f681, 11) + s.Q1[33];
                if s.Q0[34] != s.Q1[34]
{ continue; }
        
                /* C9 */
                s.Q0[35] = RL!(md5_H!(s.Q0[34], s.Q0[33], s.Q0[32]) + s.Q0[31]
                    + s.X0[27] + 0x6d9d6122, 16) + s.Q0[34];
                s.Q1[35] = RL!(md5_H!(s.Q1[34], s.Q1[33], s.Q1[32]) + s.Q1[31]
                    + s.X1[27] + 0x6d9d6122, 16) + s.Q1[34];
                if (s.Q0[35] ^ s.Q1[35]) != 0x80000000
{ continue; }
        
                /* B9 */
                s.Q0[36] = RL!(md5_H!(s.Q0[35], s.Q0[34], s.Q0[33]) + s.Q0[32]
                    + s.X0[30] + 0xfde5380c, 23) + s.Q0[35];
                s.Q1[36] = RL!(md5_H!(s.Q1[35], s.Q1[34], s.Q1[33]) + s.Q1[32]
                    + s.X1[30] + 0xfde5380c, 23) + s.Q1[35];
                if (s.Q0[36] ^ s.Q1[36]) != 0x80000000
{ continue; }
        
                /* A10 */
                s.Q0[37] = RL!(md5_H!(s.Q0[36], s.Q0[35], s.Q0[34]) + s.Q0[33]
                    + s.X0[17] + 0xa4beea44,  4) + s.Q0[36];
                s.Q1[37] = RL!(md5_H!(s.Q1[36], s.Q1[35], s.Q1[34]) + s.Q1[33]
                    + s.X1[17] + 0xa4beea44,  4) + s.Q1[36];
                if (s.Q0[37] ^ s.Q1[37]) != 0x80000000
{ continue; }
        
                /* D10 */
                s.Q0[38] = RL!(md5_H!(s.Q0[37], s.Q0[36], s.Q0[35]) + s.Q0[34]
                    + s.X0[20] + 0x4bdecfa9, 11) + s.Q0[37];
                s.Q1[38] = RL!(md5_H!(s.Q1[37], s.Q1[36], s.Q1[35]) + s.Q1[34]
                    + s.X1[20] + 0x4bdecfa9, 11) + s.Q1[37];
                if (s.Q0[38] ^ s.Q1[38]) != 0x80000000
{ continue; }
        
                /* C10 */
                s.Q0[39] = RL!(md5_H!(s.Q0[38], s.Q0[37], s.Q0[36]) + s.Q0[35]
                    + s.X0[23] + 0xf6bb4b60, 16) + s.Q0[38];
                s.Q1[39] = RL!(md5_H!(s.Q1[38], s.Q1[37], s.Q1[36]) + s.Q1[35]
                    + s.X1[23] + 0xf6bb4b60, 16) + s.Q1[38];
                if (s.Q0[39] ^ s.Q1[39]) != 0x80000000
{ continue; }
        
                /* B10 */
                s.Q0[40] = RL!(md5_H!(s.Q0[39], s.Q0[38], s.Q0[37]) + s.Q0[36]
                    + s.X0[26] + 0xbebfbc70, 23) + s.Q0[39];
                s.Q1[40] = RL!(md5_H!(s.Q1[39], s.Q1[38], s.Q1[37]) + s.Q1[36]
                    + s.X1[26] + 0xbebfbc70, 23) + s.Q1[39];
                if (s.Q0[40] ^ s.Q1[40]) != 0x80000000
{ continue; }
        
                /* A11 */
                s.Q0[41] = RL!(md5_H!(s.Q0[40], s.Q0[39], s.Q0[38]) + s.Q0[37]
                    + s.X0[29] + 0x289b7ec6,  4) + s.Q0[40];
                s.Q1[41] = RL!(md5_H!(s.Q1[40], s.Q1[39], s.Q1[38]) + s.Q1[37]
                    + s.X1[29] + 0x289b7ec6,  4) + s.Q1[40];
                if (s.Q0[41] ^ s.Q1[41]) != 0x80000000
{ continue; }
        
                /* D11 */
                s.Q0[42] = RL!(md5_H!(s.Q0[41], s.Q0[40], s.Q0[39]) + s.Q0[38]
                    + s.X0[16] + 0xeaa127fa, 11) + s.Q0[41];
                s.Q1[42] = RL!(md5_H!(s.Q1[41], s.Q1[40], s.Q1[39]) + s.Q1[38]
                    + s.X1[16] + 0xeaa127fa, 11) + s.Q1[41];
                if (s.Q0[42] ^ s.Q1[42]) != 0x80000000
{ continue; }
        
                /* C11 */
                s.Q0[43] = RL!(md5_H!(s.Q0[42], s.Q0[41], s.Q0[40]) + s.Q0[39]
                    + s.X0[19] + 0xd4ef3085, 16) + s.Q0[42];
                s.Q1[43] = RL!(md5_H!(s.Q1[42], s.Q1[41], s.Q1[40]) + s.Q1[39]
                    + s.X1[19] + 0xd4ef3085, 16) + s.Q1[42];
                if (s.Q0[43] ^ s.Q1[43]) != 0x80000000
{ continue; }
        
                /* B11 */
                s.Q0[44] = RL!(md5_H!(s.Q0[43], s.Q0[42], s.Q0[41]) + s.Q0[40]
                    + s.X0[22] + 0x04881d05, 23) + s.Q0[43];
                s.Q1[44] = RL!(md5_H!(s.Q1[43], s.Q1[42], s.Q1[41]) + s.Q1[40]
                    + s.X1[22] + 0x04881d05, 23) + s.Q1[43];
                if (s.Q0[44] ^ s.Q1[44]) != 0x80000000
{ continue; }
        
                /* A12 */
                s.Q0[45] = RL!(md5_H!(s.Q0[44], s.Q0[43], s.Q0[42]) + s.Q0[41]
                    + s.X0[25] + 0xd9d4d039,  4) + s.Q0[44];
                s.Q1[45] = RL!(md5_H!(s.Q1[44], s.Q1[43], s.Q1[42]) + s.Q1[41]
                    + s.X1[25] + 0xd9d4d039,  4) + s.Q1[44];
                if (s.Q0[45] ^ s.Q1[45]) != 0x80000000
{ continue; }
        
                /* D12 */
                s.Q0[46] = RL!(md5_H!(s.Q0[45], s.Q0[44], s.Q0[43]) + s.Q0[42]
                    + s.X0[28] + 0xe6db99e5, 11) + s.Q0[45];
                s.Q1[46] = RL!(md5_H!(s.Q1[45], s.Q1[44], s.Q1[43]) + s.Q1[42]
                    + s.X1[28] + 0xe6db99e5, 11) + s.Q1[45];
                if (s.Q0[46] ^ s.Q1[46]) != 0x80000000
{ continue; }
        
                /* C12 */
                s.Q0[47] = RL!(md5_H!(s.Q0[46], s.Q0[45], s.Q0[44]) + s.Q0[43]
                    + s.X0[31] + 0x1fa27cf8, 16) + s.Q0[46];
                s.Q1[47] = RL!(md5_H!(s.Q1[46], s.Q1[45], s.Q1[44]) + s.Q1[43]
                    + s.X1[31] + 0x1fa27cf8, 16) + s.Q1[46];
                if (s.Q0[47] ^ s.Q1[47]) != 0x80000000
{ continue; }
        
                /* B12 */
                s.Q0[48] = RL!(md5_H!(s.Q0[47], s.Q0[46], s.Q0[45]) + s.Q0[44]
                    + s.X0[18] + 0xc4ac5665, 23) + s.Q0[47];
                if (s.Q0[48] & 0x80000000) != (s.Q0[46] & 0x80000000)
{ continue; }
                s.Q1[48] = RL!(md5_H!(s.Q1[47], s.Q1[46], s.Q1[45]) + s.Q1[44]
                    + s.X1[18] + 0xc4ac5665, 23) + s.Q1[47];
                if (s.Q0[48] ^ s.Q1[48]) != 0x80000000
{ continue; }
        
                /* A13 */
                s.Q0[49] = RL!(md5_I!(s.Q0[48], s.Q0[47], s.Q0[46]) + s.Q0[45]
                    + s.X0[16] + 0xf4292244,  6) + s.Q0[48];
                if (s.Q0[49] & 0x80000000) != (s.Q0[47] & 0x80000000)
{ continue; }
                s.Q1[49] = RL!(md5_I!(s.Q1[48], s.Q1[47], s.Q1[46]) + s.Q1[45]
                    + s.X1[16] + 0xf4292244,  6) + s.Q1[48];
                if (s.Q0[49] ^ s.Q1[49]) != 0x80000000
{ continue; }
        
                /* D13 */
                s.Q0[50] = RL!(md5_I!(s.Q0[49], s.Q0[48], s.Q0[47]) + s.Q0[46]
                    + s.X0[23] + 0x432aff97, 10) + s.Q0[49];
                s.Q1[50] = RL!(md5_I!(s.Q1[49], s.Q1[48], s.Q1[47]) + s.Q1[46]
                    + s.X1[23] + 0x432aff97, 10) + s.Q1[49];
                if (s.Q0[50] ^ s.Q1[50]) != 0x80000000
{ continue; }
        
                /* C13 */
                s.Q0[51] = RL!(md5_I!(s.Q0[50], s.Q0[49], s.Q0[48]) + s.Q0[47]
                    + s.X0[30] + 0xab9423a7, 15) + s.Q0[50];
                if (s.Q0[51] & 0x80000000) != (s.Q0[49] & 0x80000000)
{ continue; }
                s.Q1[51] = RL!(md5_I!(s.Q1[50], s.Q1[49], s.Q1[48]) + s.Q1[47]
                    + s.X1[30] + 0xab9423a7, 15) + s.Q1[50];
                if (s.Q0[51] ^ s.Q1[51]) != 0x80000000
{ continue; }
        
                /* B13 */
                s.Q0[52] = RL!(md5_I!(s.Q0[51], s.Q0[50], s.Q0[49]) + s.Q0[48]
                    + s.X0[21] + 0xfc93a039, 21) + s.Q0[51];
                if (s.Q0[52] & 0x80000000) != (s.Q0[50] & 0x80000000)
{ continue; }
                s.Q1[52] = RL!(md5_I!(s.Q1[51], s.Q1[50], s.Q1[49]) + s.Q1[48]
                    + s.X1[21] + 0xfc93a039, 21) + s.Q1[51];
                if (s.Q0[52] ^ s.Q1[52]) != 0x80000000
{ continue; }
        
                /* A14 */
                s.Q0[53] = RL!(md5_I!(s.Q0[52], s.Q0[51], s.Q0[50]) + s.Q0[49]
                    + s.X0[28] + 0x655b59c3,  6) + s.Q0[52];
                if (s.Q0[53] & 0x80000000) != (s.Q0[51] & 0x80000000)
{ continue; }
                s.Q1[53] = RL!(md5_I!(s.Q1[52], s.Q1[51], s.Q1[50]) + s.Q1[49]
                    + s.X1[28] + 0x655b59c3,  6) + s.Q1[52];
                if (s.Q0[53] ^ s.Q1[53]) != 0x80000000
{ continue; }
        
                /* D14 */
                s.Q0[54] = RL!(md5_I!(s.Q0[53], s.Q0[52], s.Q0[51]) + s.Q0[50]
                    + s.X0[19] + 0x8f0ccc92, 10) + s.Q0[53];
                if (s.Q0[54] & 0x80000000) != (s.Q0[52] & 0x80000000)
{ continue; }
                s.Q1[54] = RL!(md5_I!(s.Q1[53], s.Q1[52], s.Q1[51]) + s.Q1[50]
                    + s.X1[19] + 0x8f0ccc92, 10) + s.Q1[53];
                if (s.Q0[54] ^ s.Q1[54]) != 0x80000000
{ continue; }
        
                /* C14 */
                s.Q0[55] = RL!(md5_I!(s.Q0[54], s.Q0[53], s.Q0[52]) + s.Q0[51]
                    + s.X0[26] + 0xffeff47d, 15) + s.Q0[54];
                if (s.Q0[55] & 0x80000000) != (s.Q0[53] & 0x80000000)
{ continue; }
                s.Q1[55] = RL!(md5_I!(s.Q1[54], s.Q1[53], s.Q1[52]) + s.Q1[51]
                    + s.X1[26] + 0xffeff47d, 15) + s.Q1[54];
                if (s.Q0[55] ^ s.Q1[55]) != 0x80000000
{ continue; }
        
                /* B14 */
                s.Q0[56] = RL!(md5_I!(s.Q0[55], s.Q0[54], s.Q0[53]) + s.Q0[52]
                    + s.X0[17] + 0x85845dd1, 21) + s.Q0[55];
                if (s.Q0[56] & 0x80000000) != (s.Q0[54] & 0x80000000)
{ continue; }
                s.Q1[56] = RL!(md5_I!(s.Q1[55], s.Q1[54], s.Q1[53]) + s.Q1[52]
                    + s.X1[17] + 0x85845dd1, 21) + s.Q1[55];
                if (s.Q0[56] ^ s.Q1[56]) != 0x80000000
{ continue; }
        
                /* A15 */
                s.Q0[57] = RL!(md5_I!(s.Q0[56], s.Q0[55], s.Q0[54]) + s.Q0[53]
                    + s.X0[24] + 0x6fa87e4f,  6) + s.Q0[56];
                if (s.Q0[57] & 0x80000000) != (s.Q0[55] & 0x80000000)
{ continue; }
                s.Q1[57] = RL!(md5_I!(s.Q1[56], s.Q1[55], s.Q1[54]) + s.Q1[53]
                    + s.X1[24] + 0x6fa87e4f,  6) + s.Q1[56];
                if (s.Q0[57] ^ s.Q1[57]) != 0x80000000
{ continue; }
        
                /* D15 */
                s.Q0[58] = RL!(md5_I!(s.Q0[57], s.Q0[56], s.Q0[55]) + s.Q0[54]
                    + s.X0[31] + 0xfe2ce6e0, 10) + s.Q0[57];
                if (s.Q0[58] & 0x80000000) != (s.Q0[56] & 0x80000000)
{ continue; }
                s.Q1[58] = RL!(md5_I!(s.Q1[57], s.Q1[56], s.Q1[55]) + s.Q1[54]
                    + s.X1[31] + 0xfe2ce6e0, 10) + s.Q1[57];
                if (s.Q0[58] ^ s.Q1[58]) != 0x80000000
{ continue; }
        
                /* C15 */
                s.Q0[59] = RL!(md5_I!(s.Q0[58], s.Q0[57], s.Q0[56]) + s.Q0[55]
                    + s.X0[22] + 0xa3014314, 15) + s.Q0[58];
                if (s.Q0[59] & 0x80000000) != (s.Q0[57] & 0x80000000)
{ continue; }
                s.Q1[59] = RL!(md5_I!(s.Q1[58], s.Q1[57], s.Q1[56]) + s.Q1[55]
                    + s.X1[22] + 0xa3014314, 15) + s.Q1[58];
                if (s.Q0[59] ^ s.Q1[59]) != 0x80000000
{ continue; }
        
                /* B15 */
                s.Q0[60] = RL!(md5_I!(s.Q0[59], s.Q0[58], s.Q0[57]) + s.Q0[56]
                    + s.X0[29] + 0x4e0811a1, 21) + s.Q0[59];
                s.Q1[60] = RL!(md5_I!(s.Q1[59], s.Q1[58], s.Q1[57]) + s.Q1[56]
                    + s.X1[29] + 0x4e0811a1, 21) + s.Q1[59];
                if (s.Q0[60] ^ s.Q1[60]) != 0x80000000
{ continue; }
        
                /* A16 */
                s.Q0[61] = RL!(md5_I!(s.Q0[60], s.Q0[59], s.Q0[58]) + s.Q0[57]
                    + s.X0[20] + 0xf7537e82,  6) + s.Q0[60];
                s.Q1[61] = RL!(md5_I!(s.Q1[60], s.Q1[59], s.Q1[58]) + s.Q1[57]
                    + s.X1[20] + 0xf7537e82,  6) + s.Q1[60];
                if (s.Q0[61] ^ s.Q1[61]) != 0x80000000
{ continue; }
                if (s.A0 + s.Q0[61]) != (s.A1 + s.Q1[61])
{ continue; }
        
                /* D16 */
                s.Q0[62] = RL!(md5_I!(s.Q0[61], s.Q0[60], s.Q0[59]) + s.Q0[58]
                    + s.X0[27] + 0xbd3af235, 10) + s.Q0[61];
                s.Q1[62] = RL!(md5_I!(s.Q1[61], s.Q1[60], s.Q1[59]) + s.Q1[58]
                    + s.X1[27] + 0xbd3af235, 10) + s.Q1[61];
                if (s.D0 + s.Q0[62]) != (s.D1 + s.Q1[62])
{ continue; }
        
                /* C16 */
                s.Q0[63] = RL!(md5_I!(s.Q0[62], s.Q0[61], s.Q0[60]) + s.Q0[59]
                    + s.X0[18] + 0x2ad7d2bb, 15) + s.Q0[62];
                s.Q1[63] = RL!(md5_I!(s.Q1[62], s.Q1[61], s.Q1[60]) + s.Q1[59]
                    + s.X1[18] + 0x2ad7d2bb, 15) + s.Q1[62];
                if (s.C0 + s.Q0[63]) != (s.C1 + s.Q1[63])
{ continue; }
        
                /* B16 */
                s.Q0[64] = RL!(md5_I!(s.Q0[63], s.Q0[62], s.Q0[61]) + s.Q0[60]
                    + s.X0[25] + 0xeb86d391, 21) + s.Q0[63];
                s.Q1[64] = RL!(md5_I!(s.Q1[63], s.Q1[62], s.Q1[61]) + s.Q1[60]
                    + s.X1[25] + 0xeb86d391, 21) + s.Q1[63];
                if (s.B0 + s.Q0[64]) != (s.B1 + s.Q1[64])
{ continue; }
                goto_flag = 1;
                break;
            }
            if cnt == (LOOP_22 - 1) && goto_flag == 0
                { continue; }
            return 0;
        }
    }
}

/*
fn md5_init_ctx(ctx: &mut md5_ctx)
{
  ctx.A = 0x67452301;
  ctx.B = 0xefcdab89;
  ctx.C = 0x98badcfe;
  ctx.D = 0x10325476;

  ctx.total[0] = 0;
  ctx.total[1] = 0;
  ctx.buflen = 0;
}


fn find_iv(file: & String, IV: &mut [u32; 4]) -> Result<(), std::io::ERR!or>
{
    //let mut buf_reader = BufReader::new(file);
    let mut buf = fs::read(file)?;
    //md5_init_ctx(&mut ctx);

    Ok(())
}
*/

fn main() -> std::io::Result<()>
{
    println!("MD5 Collision in Rust Program Started");
    let args: Vec<String> = env::args().collect();

    //let option = &args[1];
    //let filename = &args[2];
    //let file = File::open(filename)?;
    let mut a: u32 = 5;
    let mut IV : [u32; 4] = [0; 4];
    //find_iv(filename, &mut IV);
    block1(IV_default, 0);
    Ok(())
}