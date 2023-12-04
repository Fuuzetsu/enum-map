#![cfg(feature = "borsh")]

// SPDX-FileCopyrightText: 2023 Mateusz Kowalczyk <fuuzetsu@fuuzetsu.co.uk>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use enum_map::enum_map;
use enum_map_derive::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Enum, Deserialize, Serialize, PartialEq)]
enum Inner {
    One,
    Two,
    Three,
}

#[derive(Debug, Enum, Deserialize, Serialize, PartialEq)]
enum Example {
    A,
    B(Inner),
    C,
}

#[test]
fn round_trip() {
    let map = enum_map! {
        Example::A => 5,
        Example::B(Inner::One) => 10,
        Example::B(Inner::Two) => 11,
        Example::B(Inner::Three) => 12,
        Example::C => 42,
    };

    let ser = borsh::to_vec(&map).unwrap();
    let deser = borsh::from_slice(&ser).unwrap();

    assert_eq!(map, deser);
}
