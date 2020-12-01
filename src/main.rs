use std::time::Instant;

use plotters::prelude::*;
use rayon::prelude::*;

struct Pop {
    a: f32,
    curr: f32,
    next: f32,
}

impl Pop {
    fn adv(&mut self) -> f32 {
        let ret = self.next;
        self.curr = ret;
        self.next = self.a * ret * (1.0 - ret);
        ret
    }
}

fn eps_eq(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < 1.0 / STEPS_PER_INT
}

const SKIP: usize = 200;
const GENS: usize = 1000;

const STEPS_PER_INT: f32 = 10000.0;

const SIZE: u32 = 8;
const BORDER_SIZE: i32 = 30 * SIZE as i32;
const LABEL_SIZE: i32 = 13 * SIZE as i32;

fn main() {
    let fill = Instant::now();
    // fill
    let hist: Vec<_> = (0..=(4 * STEPS_PER_INT as usize))
        .into_par_iter()
        .map(|a| {
            let mut hist = [0.0; GENS - SKIP + 1];
            let a = (a as f32) / STEPS_PER_INT;
            let mut pop = Pop { a, curr: 0.0, next: 0.25 };
            for i in 0..=GENS {
                let x = pop.adv();
                if i >= SKIP {
                    hist[i - SKIP] = x;
                }
            }
            hist
        }).collect();

    let dedup = Instant::now();
    eprintln!("fill  = {:?}", dedup.duration_since(fill));

    // dedup
    let vec: Vec<_> = hist
        .par_iter()
        .enumerate()
        .flat_map(|(a_int, hist)| {
            let a = (a_int as f32) / STEPS_PER_INT;
            let mut new = Vec::new();
            for &x in hist {
                if new.iter().find(|(_, x1)| eps_eq(*x1, x)).is_none() {
                    new.push((a, x));
                }
            }
            new.into_par_iter()
        })
        .collect();


    let build = Instant::now();
    eprintln!("dedup = {:?}", build.duration_since(dedup));

    let root = BitMapBackend::new("chaos_small.png", (640 * SIZE, 480 * SIZE)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(50)
        .x_label_area_size(BORDER_SIZE)
        .y_label_area_size(BORDER_SIZE)
        .build_cartesian_2d(-0f32..4f32, -0f32..1.001f32).unwrap();

    chart.configure_mesh()
        .x_desc("rate (a)")
        .x_label_style(("sans-serif", LABEL_SIZE))
        .y_desc("population")
        .y_label_style(("sans-serif", LABEL_SIZE))
        .bold_line_style(ShapeStyle {
            color: BLACK.mix(0.3),
            filled: true,
            stroke_width: SIZE / 2,
        })
        .light_line_style(ShapeStyle {
            color: BLACK.mix(0.1),
            filled: true,
            stroke_width: SIZE / 2,
        })
        .axis_style(ShapeStyle {
            color: BLACK.to_rgba(),
            filled: true,
            stroke_width: SIZE / 2,
        })
        .draw().unwrap();

    let draw = Instant::now();
    eprintln!("build = {:?}", draw.duration_since(build));

    chart.draw_series(PointSeries::of_element(
        vec.into_iter().map(|(a, x)| (a as f32, x as f32)),
        1,
        &RED,
        &|c, s, st| {
            return Circle::new(c, s, st.filled());
        },
    )).unwrap();

    let save = Instant::now();
    eprintln!("draw  = {:?}", save.duration_since(draw));

    drop(chart);
    drop(root);
    eprintln!("save  = {:?}", save.elapsed());
}
