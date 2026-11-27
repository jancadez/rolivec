use crate::{Vary, Vec3, Vec4};

#[derive(Clone, Copy, Default)]
pub(crate) struct ClipVertex<T> {
    pub pos: Vec4,
    pub bary: Vec3,
    pub vary: T,
}

pub(crate) fn clip_triangle<T: Vary>(
    vs: &[Vec4; 3],
    vary: &[T; 3],
    out: &mut [ClipVertex<T>; 9],
) -> usize {
    let mut bits: [u8; 3] = [0; 3];

    for i in 0..3 {
        bits[i] = check_vert(vs[i]);
    }

    if (bits[0] & bits[1] & bits[2]) > 0 {
        return 0;
    }

    let initial = [
        ClipVertex {
            pos: vs[0],
            bary: Vec3::new(1.0, 0.0, 0.0),
            vary: vary[0],
        },
        ClipVertex {
            pos: vs[1],
            bary: Vec3::new(0.0, 1.0, 0.0),
            vary: vary[1],
        },
        ClipVertex {
            pos: vs[2],
            bary: Vec3::new(0.0, 0.0, 1.0),
            vary: vary[2],
        },
    ];

    if (bits[0] | bits[1] | bits[2]) == 0 {
        out[..3].copy_from_slice(&initial);
        return 3;
    }

    let mut curr = [ClipVertex::default(); 9];
    let mut next = [ClipVertex::default(); 9];
    curr[..3].copy_from_slice(&initial);

    let mut n = 3;
    for i in 0..6 {
        n = clip_plane(&curr[..n], i, &mut next);
        std::mem::swap(&mut curr, &mut next);

        if n < 3 {
            return 0;
        }
    }

    out[..n].copy_from_slice(&curr[..n]);

    n
}

fn plane_dist(p: Vec4, plane: usize) -> f32 {
    match plane {
        0 => p.w + p.x,
        1 => p.w - p.x,
        2 => p.w + p.y,
        3 => p.w - p.y,
        4 => p.w + p.z,
        5 => p.w - p.z,
        _ => unreachable!(),
    }
}

fn check_vert(p: Vec4) -> u8 {
    let mut bitset: u8 = 0;

    for i in 0..6 {
        if plane_dist(p, i) < 0.0 {
            bitset |= 1 << i;
        }
    }

    bitset
}

fn clip_plane<T: Vary>(
    input: &[ClipVertex<T>],
    plane: usize,
    out: &mut [ClipVertex<T>; 9],
) -> usize {
    let mut n = 0;

    for i in 0..input.len() {
        let curr = input[i];
        let dc = plane_dist(curr.pos, plane);

        let next = input[(i + 1) % input.len()];
        let dn = plane_dist(next.pos, plane);

        if dc >= 0.0 {
            out[n] = curr;
            n += 1;
        }

        if (dc >= 0.0) ^ (dn >= 0.0) {
            let t = dc / (dc - dn);
            out[n] = ClipVertex {
                pos: curr.pos + (next.pos - curr.pos) * t,
                bary: curr.bary + (next.bary - curr.bary) * t,
                vary: curr.vary + (next.vary - curr.vary) * t,
            };

            n += 1;
        }
    }

    n
}
