// Copyright 2016-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{self, Debug, Formatter};
use std::ops::{Index, IndexMut};

#[cfg(feature = "enable_serde")]
extern crate serde;
#[cfg(feature = "enable_serde")]
use self::serde::*;

/// Represents the four cardinal directions in 2D space.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub enum Card {
    /// Negative X direction.
    MinusX,

    /// Negative Y direction.
    MinusY,

    /// Positive X direction.
    PlusX,

    /// Positive Y direction.
    PlusY,
}

impl Card {
    /// Returns the negative of the current direction.
    pub fn flip(self) -> Card {
        match self {
            Card::MinusX => Card::PlusX,
            Card::PlusX => Card::MinusX,
            Card::MinusY => Card::PlusY,
            Card::PlusY => Card::MinusY,
        }
    }

    /// Returns all cardinal directions.
    #[inline]
    pub fn values() -> [Card; 4] {
        [Card::MinusX, Card::MinusY, Card::PlusX, Card::PlusY]
    }
}

/// A map from `Card` to `bool`, typically used to specify allowed normal vector
/// directions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct CardMask {
    flags: [bool; 4],
}

impl CardMask {
    /// Creates a `CardMask` with all values set to `false`.
    #[inline]
    pub fn empty() -> CardMask {
        CardMask { flags: [false; 4] }
    }

    /// Creates a `CardMask` with all values set to `true`.
    #[inline]
    pub fn full() -> CardMask {
        CardMask { flags: [true; 4] }
    }

    pub(crate) fn flip(self) -> CardMask {
        let mut result = CardMask::empty();
        result[Card::PlusX] = self[Card::MinusX];
        result[Card::MinusX] = self[Card::PlusX];
        result[Card::PlusY] = self[Card::MinusY];
        result[Card::MinusY] = self[Card::PlusY];
        result
    }
}

impl From<Card> for CardMask {
    fn from(card: Card) -> CardMask {
        let mut result = CardMask::empty();
        result[card] = true;
        result
    }
}

impl Index<Card> for CardMask {
    type Output = bool;

    #[inline]
    fn index(&self, index: Card) -> &bool {
        &self.flags[index as usize]
    }
}

impl IndexMut<Card> for CardMask {
    #[inline]
    fn index_mut(&mut self, index: Card) -> &mut bool {
        &mut self.flags[index as usize]
    }
}

impl Debug for CardMask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "CardMask {{ MinusX: {}, MinusY: {}, PlusX: {}, PlusY: {} }}",
            self[Card::MinusX],
            self[Card::MinusY],
            self[Card::PlusX],
            self[Card::PlusY]
        )
    }
}

#[cfg(all(test, feature = "enable_serde"))]
pub(crate) mod test_serde {
    use super::*;
    use ron::de;
    use ron::ser;
    use std::default::Default;

    #[test]
    fn test_card() {
        let pretty: ser::PrettyConfig = Default::default();

        let dut = Card::MinusX;
        let serialized = ser::to_string_pretty(&dut, pretty.clone()).unwrap();
        let dut2: Card = de::from_str(&serialized).unwrap();
        assert_eq!(dut, dut2);

        let dut = Card::MinusY;
        let serialized = ser::to_string_pretty(&dut, pretty.clone()).unwrap();
        let dut2: Card = de::from_str(&serialized).unwrap();
        assert_eq!(dut, dut2);

        let dut = Card::PlusX;
        let serialized = ser::to_string_pretty(&dut, pretty.clone()).unwrap();
        let dut2: Card = de::from_str(&serialized).unwrap();
        assert_eq!(dut, dut2);

        let dut = Card::PlusY;
        let serialized = ser::to_string_pretty(&dut, pretty.clone()).unwrap();
        let dut2: Card = de::from_str(&serialized).unwrap();
        assert_eq!(dut, dut2);
    }

    #[test]
    fn test_card_mask() {
        let pretty: ser::PrettyConfig = Default::default();

        let choices = vec![true, false];
        for i in choices.iter() {
            for j in choices.iter() {
                for k in choices.iter() {
                    for l in choices.iter() {
                        let dut = CardMask {
                            flags: [*i, *j, *k, *l],
                        };
                        let serialized = ser::to_string_pretty(&dut, pretty.clone()).unwrap();
                        let dut2: CardMask = de::from_str(&serialized).unwrap();
                        assert_eq!(dut, dut2);
                    }
                }
            }
        }
    }
}
