use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tree::avltree::AVLTree;
use tree::rbtree::RBTree;
use tree::tree::Tree;
use tree::{avltree, rbtree};

fn bench_rbtree(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("rbtree_tests");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("insert:{size}")),
            size,
            |bench, &size| {
                bench.iter(|| {
                    let mut tree: RBTree = rbtree::RBTree::new();
                    for index in 1..size {
                        tree.insert(index);
                    }
                })
            },
        );

        let mut tree: RBTree = rbtree::RBTree::new();
        for index in 1..*size {
            tree.insert(index);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("search:{size}")),
            size,
            |bench, &size| {
                bench.iter(|| {
                    for index in 0..size / 10 {
                        tree.contain(index);
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_avltree(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("avltree_tests");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("insert:{size}")),
            size,
            |bench, &size| {
                bench.iter(|| {
                    let mut tree: AVLTree = avltree::AVLTree::new();
                    for index in 1..size {
                        tree.insert(index);
                    }
                })
            },
        );

        let mut tree: AVLTree = avltree::AVLTree::new();
        for index in 1..*size {
            tree.insert(index);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("search:{size}")),
            size,
            |bench, &size| {
                bench.iter(|| {
                    for index in 0..size / 10 {
                        tree.contain(index);
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_rbtree, bench_avltree
}

criterion_main!(benches);
