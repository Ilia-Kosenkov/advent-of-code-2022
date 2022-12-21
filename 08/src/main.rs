use std::io::{self, BufRead};

use itertools::Itertools;
use ndarray::{Array2, Axis};

fn main() {
    let reader = io::BufReader::new(io::stdin());

    let input = reader
        .lines()
        .map_ok(|s| s.bytes().map(|b| b - ('0' as u8)).collect_vec())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let (n, m) = (input[0].len(), input.len());
    let mut forest: Array2<u8> = Array2::default((n, m));

    forest
        .axis_iter_mut(Axis(0))
        .zip(&input)
        .for_each(|(mut dst, src)| {
            dst.iter_mut()
                .zip(src)
                .for_each(|(dst_item, src_item)| *dst_item = *src_item);
        });

    let mut visibility: Array2<bool> = Array2::default((forest.shape()[0], forest.shape()[1]));

    set_boundary_visibility(&mut visibility);
    check_visibility(&forest, &mut visibility);

    let n_visible = visibility.iter().filter(|x| **x).count();

    println!("{}", n_visible);
}

fn set_boundary_visibility(visibility: &mut Array2<bool>) {
    set_boundary_visibility_1d(visibility, Axis(0));
    set_boundary_visibility_1d(visibility, Axis(1));
}

fn set_boundary_visibility_1d(visibility: &mut Array2<bool>, axis: Axis) {
    let mut iter = visibility.axis_iter_mut(axis);
    iter.nth(0).unwrap().iter_mut().for_each(|x| *x = true);
    iter.nth_back(0).unwrap().iter_mut().for_each(|x| *x = true);
}

fn check_visibility(forest: &Array2<u8>, visibility: &mut Array2<bool>) {
    check_visibility_2d(forest, visibility, Axis(0));
    check_visibility_2d(forest, visibility, Axis(1));
}

fn check_visibility_2d(forest: &Array2<u8>, visibility: &mut Array2<bool>, axis: Axis) {
    for (forest, mut visibility) in forest.axis_iter(axis).zip(visibility.axis_iter_mut(axis)) {
        check_visibility_1d(forest.iter(), visibility.iter_mut());
        check_visibility_1d(forest.iter().rev(), visibility.iter_mut().rev())
    }
}

fn check_visibility_1d<'a, T, U>(forest: T, visibility: U)
where
    T: Iterator<Item = &'a u8>,
    U: Iterator<Item = &'a mut bool>,
{
    let mut max_height = 0;
    for (f, v) in forest.into_iter().zip(visibility) {
        if *f > max_height {
            *v = true;
        }
        max_height = max_height.max(*f);
    }
}
