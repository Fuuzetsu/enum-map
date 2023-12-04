// SPDX-FileCopyrightText: 2023 Mateusz Kowalczyk <fuuzetsu@fuuzetsu.co.uk>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Enum, EnumMap};
use borsh::{BorshDeserialize, BorshSerialize};

/// Requires crate feature `"borsh"`
impl<K, V> BorshSerialize for EnumMap<K, V>
where
    K: Enum + crate::EnumArray<V>,
    <K as crate::EnumArray<V>>::Array: BorshSerialize,
{
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        <<K as crate::EnumArray<V>>::Array as BorshSerialize>::serialize(&self.array, writer)
    }
}

/// Requires crate feature `"borsh"`
impl<K, V> BorshDeserialize for EnumMap<K, V>
where
    K: Enum + crate::EnumArray<V>,
    <K as crate::EnumArray<V>>::Array: BorshDeserialize,
{
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        <<K as crate::EnumArray<V>>::Array as BorshDeserialize>::deserialize_reader(reader)
            .map(|array| Self { array })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn compiles() {
        fn der<T: borsh::BorshDeserialize>() {}
        fn ser<T: borsh::BorshSerialize>() {}

        der::<crate::EnumMap<u8, ()>>();
        ser::<crate::EnumMap<u8, ()>>();
    }
}
