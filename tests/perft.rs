// This file is part of the shakmaty library.
// Copyright (C) 2017 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use shakmaty::{CastlingMode, Chess, FromSetup, Position};
use shakmaty::variants::{Atomic, Antichess, Crazyhouse, RacingKings, Horde};
use shakmaty::fen::Fen;
use shakmaty::perft;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn test_perft_file<P>(path: &str, node_limit: u64)
where
    P: Position + FromSetup + Default + Clone,
{
    let file = File::open(path).expect("failed to open test suite");
    let reader = BufReader::new(file);

    let mut pos = P::default();

    for line in reader.lines().map(|l| l.unwrap()) {
        println!("{}", line);

        let trimmed = line.trim();
        let mut slices = trimmed.splitn(2, ' ');

        match slices.next() {
            Some("epd") => {
                pos = slices.next()
                    .expect("missing epd")
                    .parse::<Fen>()
                    .expect("invalid fen")
                    .position(CastlingMode::Chess960)
                    .expect("illegal fen");
            },
            Some("perft") => {
                let mut params = slices.next().expect("missing perft params").splitn(2, ' ');

                let depth = params.next().expect("missing perft depth")
                                  .parse().expect("depth not an integer");

                let nodes = params.next().expect("missing perft nodes")
                                  .parse().expect("nodes not an integer");

                if nodes <= node_limit {
                    assert_eq!(perft(&pos, depth), nodes);
                }
            }
            _ => {}
        }
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_random() {
    test_perft_file::<Chess>("tests/random.perft", 10_000);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_tricky() {
    test_perft_file::<Chess>("tests/tricky.perft", 100_0000);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_atomic() {
    test_perft_file::<Atomic>("tests/atomic.perft", 1_000_000);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_antichess() {
    test_perft_file::<Antichess>("tests/antichess.perft", 1_000_000);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_crazyhouse() {
    test_perft_file::<Crazyhouse>("tests/crazyhouse.perft", 1_000_000);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_racingkings() {
    test_perft_file::<RacingKings>("tests/racingkings.perft", 1_000_000);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_horde() {
    test_perft_file::<Horde>("tests/horde.perft", 1_000_000);
}
