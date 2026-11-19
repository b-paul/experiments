use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
use itertools::iproduct;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Dir {
    U,
    D,
    F,
    B,
    L,
    R,
}

impl Dir {
    const ALL: [Dir; 6] = [Dir::U, Dir::D, Dir::F, Dir::B, Dir::L, Dir::R];

    fn adj(self) -> [Dir; 4] {
        match self {
            Dir::U | Dir::D => [Dir::F, Dir::B, Dir::L, Dir::R],
            Dir::F | Dir::B => [Dir::U, Dir::D, Dir::L, Dir::R],
            Dir::L | Dir::R => [Dir::U, Dir::D, Dir::F, Dir::B],
        }
    }

    fn mv(self, (x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        match self {
            Dir::U => (x, y, z + 1),
            Dir::D => (x, y, z - 1),
            Dir::F => (x + 1, y, z),
            Dir::B => (x - 1, y, z),
            Dir::L => (x, y + 1, z),
            Dir::R => (x, y - 1, z),
        }
    }
}

fn make_pieces() -> Vec<[(i32, i32, i32); 5]> {
    iproduct!(0..5, 0..5, 0..5)
        .flat_map(|a| {
            Dir::ALL.into_iter().flat_map(move |dir| {
                let b = dir.mv(a);
                let c = dir.mv(b);
                dir.adj().into_iter().map(move |dir2| {
                    let d = dir2.mv(c);
                    let e = dir.mv(d);
                    [a, b, c, d, e]
                })
            })
        })
        .filter(|piece| {
            piece
                .iter()
                .all(|&(x, y, z)| (0..5).contains(&x) && (0..5).contains(&y) && (0..5).contains(&z))
        })
        .collect()
}

fn main() {
    variables! {problem: };

    let pieces = make_pieces();
    println!("{:?}", pieces.len());

    let vars: Vec<_> = pieces
        .iter()
        .map(|_| variable().integer().max(1).min(0))
        .collect();

    let vars: Vec<_> = problem.add_all(vars);

    let obj = vars.iter().sum::<Expression>();
    let model = problem.maximise(&obj).using(default_solver);

    let model = iproduct!(0..5, 0..5, 0..5).fold(model, |model, p| {
        model.with(
            vars.iter()
                .zip(pieces.iter())
                .filter(|(_, piece)| piece.contains(&p))
                .map(|(v, _)| v)
                .sum::<Expression>()
                .leq(1),
        )
    });

    println!("model constructed");

    let sol = model.solve().unwrap();
    println!("{}", sol.eval(&obj));
    for (i, v) in vars.into_iter().enumerate() {
        if sol.eval(v) > 0. {
            println!("{:?}", pieces[i]);
        }
    }
}
