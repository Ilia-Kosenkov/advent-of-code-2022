use std::{
    fmt::Debug,
    io::{self, BufRead},
};

use itertools::Itertools;
use ndarray::{Array2, Axis};

#[derive(Copy, Clone, Default)]
struct Visibility {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

impl Debug for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({},{},{},{})",
            &self.top, &self.right, &self.bottom, &self.left
        ))
    }
}

impl Visibility {
    fn area(&self) -> usize {
        self.top * self.right * self.bottom * self.left
    }
}

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

    let mut scenic_scores: Array2<Visibility> =
        Array2::default((forest.shape()[0], forest.shape()[1]));

    populate_scenic_scores(&forest, &mut scenic_scores);

    let max_score = scenic_scores
        .iter()
        .map(|v| v.area())
        .max()
        .unwrap_or(0usize);

    println!("{}", max_score);
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
    let mut iter = forest.into_iter().zip(visibility);
    let mut max_height = *iter.by_ref().next().unwrap().0;

    for (f, v) in iter {
        if *f > max_height {
            *v = true;
        }
        max_height = max_height.max(*f);
    }
}

fn populate_scenic_scores(trees: &Array2<u8>, scenic_scores: &mut Array2<Visibility>) {
    trees
        .axis_iter(Axis(0))
        .zip(scenic_scores.axis_iter_mut(Axis(0)))
        .for_each(|(t, mut ss)| {
            populate_scenic_scores_1d(t.iter(), ss.iter_mut().map(|s| &mut s.left));
            populate_scenic_scores_1d(t.iter().rev(), ss.iter_mut().rev().map(|s| &mut s.right));
        });

    trees
        .axis_iter(Axis(1))
        .zip(scenic_scores.axis_iter_mut(Axis(1)))
        .for_each(|(t, mut ss)| {
            populate_scenic_scores_1d(t.iter(), ss.iter_mut().map(|s| &mut s.top));
            populate_scenic_scores_1d(t.iter().rev(), ss.iter_mut().rev().map(|s| &mut s.bottom));
        });
}

fn populate_scenic_scores_1d<'a, T, U>(trees: T, scenic_scores: U)
where
    T: DoubleEndedIterator<Item = &'a u8>,
    U: DoubleEndedIterator<Item = &'a mut usize>,
{
    let mut checked_trees = Vec::<u8>::with_capacity(trees.size_hint().1.unwrap_or(0));
    for (&tree, score) in trees.zip(scenic_scores) {
        *score = checked_trees
            .iter()
            .rev()
            .take_while(|&&t| t < tree)
            .count()
            + 1;

        *score = checked_trees.len().min(*score);

        checked_trees.push(tree);
    }
}
