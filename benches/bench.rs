use criterion::{criterion_group, criterion_main, Criterion};

use std::hint::black_box;
use std::iter::successors;
use rand::{rngs, Rng, SeedableRng};
use time::{Date, Duration, Month};
use paste::paste;

use pyxirr::core;

fn gen_amounts<const LEN: usize>() -> Box<[f64]> {
    let mut rng = rngs::StdRng::seed_from_u64(0);
    let mut amounts: Box<[f64]> = Box::from([0f64; LEN]);
    rng.fill(&mut *amounts);
    let bal: f64 = (*amounts).iter().sum::<f64>() * 0.9;
    amounts[0] = -bal;
    amounts
}

fn gen_dates<const LEN: usize>() -> Box<[core::DateLike]> {
    let mut rng = rngs::StdRng::seed_from_u64(0);
    let base_date = Date::from_calendar_date(2019, Month::January, 1).unwrap();
    successors(Some(base_date), |x| Some(*x + Duration::days(rng.gen_range(1..7)))).map(|x| x.into()).take(LEN).collect()
}

macro_rules! bench_xirr {
    ($($num:expr)*) => {
        paste! {
            $(
                fn [<bench_xirr $num>](c: &mut Criterion) {
                    let amounts = gen_amounts::<$num>();
                    let dates = gen_dates::<$num>();
                    let day_count = core::DayCount::THIRTY_E_360_ISDA;
                    c.bench_function(concat!("bench xirr ", $num), |b| {
                        b.iter(|| core::xirr(&dates, &amounts, None, Some(day_count)).unwrap())
                    });
                }
            )*
            criterion_group!(xirr, $([<bench_xirr $num>]),*);
        }
    }
}

macro_rules! bench_irr {
    ($($num:expr)*) => {
        paste! {
            $(
                fn [<bench_irr $num>](c: &mut Criterion) {
                    let amounts = gen_amounts::<$num>();
                    c.bench_function(concat!("bench irr ", $num), |b| {
                        b.iter(|| core::irr(&amounts, None).unwrap())
                    });
                }
            )*
            criterion_group!(irr, $([<bench_irr $num>]),*);
        }
    }
}

macro_rules! bench_npv {
    ($($num:expr)*) => {
        paste! {
            $(
                fn [<bench_npv $num>](c: &mut Criterion) {
                    let amounts = gen_amounts::<$num>();
                    let rate = 0.01f64;
                    c.bench_function(concat!("bench npv ", $num), |b| {
                        b.iter(|| black_box(core::npv(rate, &amounts, None)))
                    });
                }
            )*
            criterion_group!(npv, $([<bench_npv $num>]),*);
        }
    }
}

bench_xirr!(100 500 700 1000 1500 2000);
bench_irr!(100 500 700 1000 1500 2000);
bench_npv!(100 500 700 1000 1500 2000);
criterion_main!(xirr, irr, npv);
