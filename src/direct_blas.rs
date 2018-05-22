#![allow(unsafe_code)]

use rand::Rng;
use std::iter::Sum;
use util::{gen_rng, fill_rand, myrand};
use cblas::{dgemm, Layout, Transpose};
use itertools::Itertools;

pub fn randmatstat(t: usize) -> (f64, f64) {
    let mut rng = gen_rng(1234u64);

    const N: usize = 5;

    let mut v = vec![0.; t];
    let mut w = vec![0.; t];

    {
        let mut a = [0.; N * N];
        let mut b = [0.; N * N];
        let mut c = [0.; N * N];
        let mut d = [0.; N * N];
        let mut p = [0.; (N) * (4 * N)];
        let mut q = [0.; (2 * N) * (2 * N)];

        let mut pt_p1 = [0.; (4 * N) * (4 * N)];
        let mut pt_p2 = [0.; (4 * N) * (4 * N)];
        let mut qt_q1 = [0.; (2 * N) * (2 * N)];
        let mut qt_q2 = [0.; (2 * N) * (2 * N)];

        for (ve, we) in v.iter_mut().zip(w.iter_mut()) {
            fill_rand(&mut a, &mut rng);
            fill_rand(&mut b, &mut rng);
            fill_rand(&mut c, &mut rng);
            fill_rand(&mut d, &mut rng);

            p[0 .. N * N].copy_from_slice(&a);
            p[N * N .. 2 * N * N].copy_from_slice(&b);
            p[2 * N * N .. 3 * N * N].copy_from_slice(&c);
            p[3 * N * N .. 4 * N * N].copy_from_slice(&d);

            for j in 0..N {
                for k in 0..N {
                    q[2 * N * j + k] = a[k];
                    q[2 * N * j + N + k] = b[k];
                    q[2 * N * (N + j) + k] = c[k];
                    q[2 * N * (N + j) + N + k] = d[k];
                }
            }

            unsafe {
                let n = N as i32;

                dgemm(Layout::ColumnMajor, Transpose::Ordinary, Transpose::None,
                    n , n, 4 * n, 1., &p, 4 * n, &p, 4 * n, 0.,
                    &mut pt_p1, 4 * n);
                dgemm(Layout::ColumnMajor, Transpose::None, Transpose::None,
                    4 * n, 4 * n, 4 * n, 1., &pt_p1, 4 * n, &pt_p1, 4 * n, 0.,
                    &mut pt_p2, 4 * n);
                dgemm(Layout::ColumnMajor, Transpose::None, Transpose::None,
                    4 * n, 4 * n, 4 * n, 1., &pt_p2, 4 * n, &pt_p2, 4 * n, 0.,
                    &mut pt_p1, 4 * n);
            }

            *ve = trace(&pt_p1, N * 4);

            unsafe {
                let n = N as i32;

                dgemm(Layout::ColumnMajor, Transpose::Ordinary, Transpose::None,
                    2 * n, 2 * n, 2 * n, 1., &q, 2 * n, &q, 2 * n, 0.,
                    &mut qt_q1, 2 * n);
                dgemm(Layout::ColumnMajor, Transpose::None, Transpose::None,
                    2 * n, 2 * n, 2 * n, 1., &qt_q1, 2 * n, &qt_q1, 2 * n, 0.,
                    &mut qt_q2, 2 * n);
                dgemm(Layout::ColumnMajor, Transpose::None, Transpose::None,
                    2 * n, 2 * n, 2 * n, 1., &qt_q2, 2 * n, &qt_q2, 2 * n, 0.,
                    &mut qt_q1, 2 * n);
            }

            *we = trace(&qt_q1, 2 * N);
        }
    }

    let (v1, v2, w1, w2) = v.iter()
        .zip(w.iter())
        .fold((0., 0., 0., 0.), |(v1, v2, w1, w2), (ve, we)| (
            v1 + *ve,
            v2 + ve * ve,
            w1 + *we,
            w2 + we * we
        ));

    let t = t as f64;

    (
        f64::sqrt((t * (t * v2 - v1 * v1)) / ((t - 1.) * v1 * v1)),
        f64::sqrt((t * (t * w2 - w1 * w1)) / ((t - 1.) * w1 * w1)),
    )
}

/// Calculate the trace of a square matrix
#[inline]
fn trace<'a, T>(m: &'a [T], n: usize) -> T
where
    T: Sum<&'a T>
{
    debug_assert_eq!(m.len(), n * n);
    m.into_iter().step(n + 1).sum()
}

pub fn randmatmul<R: Rng>(n: usize, mut rng: R) -> Vec<f64> {
    let a = myrand(n * n, &mut rng);
    let b = myrand(n * n, &mut rng);
    let mut c = vec![0.; n * n];

    unsafe {
        let n = n as i32;
        dgemm(Layout::ColumnMajor, Transpose::None, Transpose::None,
            n, n, n, 1., &a, n, &b, n, 0., &mut c, n);
    }

    c
}

#[inline]
pub fn check_randmatmul(m: Vec<f64>) {
    assert!(0. <= m[0]);
}
