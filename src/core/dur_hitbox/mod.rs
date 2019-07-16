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

mod solvers;

use crate::geom::shape::PlacedBounds;
use crate::geom::*;
use rug::{
    float,
    float::{prec_max, OrdFloat, Round},
    Float,
};

#[cfg(feature = "enable_serde")]
extern crate serde;
#[cfg(feature = "enable_serde")]
use self::serde::*;

// DurHitbox (and DurHbVel) is almost identical to Hitbox (and HbVel), except
// it uses a `duration` (amount of time until invalidation of the hitbox)
// rather than an `end_time` (time of the invalidation of the hitbox). This
// new struct is meant to make that distinction clear.

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct DurHbVel {
    pub value: Vec2,
    pub resize: Vec2,
    pub duration: OrdFloat,
}

impl DurHbVel {
    pub fn still() -> DurHbVel {
        DurHbVel {
            value: Vec2::zero(),
            resize: Vec2::zero(),
            duration: OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity)),
        }
    }

    fn is_still(&self) -> bool {
        self.value == Vec2::zero() && self.resize == Vec2::zero()
    }

    fn negate(&self) -> DurHbVel {
        DurHbVel {
            value: -self.value,
            resize: -self.resize,
            duration: self.duration,
        }
    }
}

impl PlacedBounds for DurHbVel {
    fn bounds_center(&self) -> &Vec2 {
        &self.value
    }
    fn bounds_dims(&self) -> &Vec2 {
        &self.resize
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct DurHitbox {
    pub value: PlacedShape,
    pub vel: DurHbVel,
}

impl DurHitbox {
    pub fn new(value: PlacedShape) -> DurHitbox {
        DurHitbox {
            value,
            vel: DurHbVel::still(),
        }
    }

    pub fn advanced_shape(&self, time: OrdFloat) -> PlacedShape {
        self.value.advance(self.vel.value, self.vel.resize, time)
    }

    pub fn bounding_box(&self) -> PlacedShape {
        self.bounding_box_for(self.vel.duration)
    }

    pub fn bounding_box_for(&self, duration: OrdFloat) -> PlacedShape {
        if self.vel.is_still() {
            self.value.as_rect()
        } else {
            let end_value = self.advanced_shape(duration);
            self.value.bounding_box(&end_value)
        }
    }

    pub fn collide_time(&self, other: &DurHitbox) -> OrdFloat {
        solvers::collide_time(self, other)
    }

    pub fn separate_time(&self, other: &DurHitbox, padding: OrdFloat) -> OrdFloat {
        solvers::separate_time(self, other, padding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dur_hitbox::DurHitbox;

    #[cfg(feature = "enable_serde")]
    use bincode::{deserialize, serialize};

    #[cfg(feature = "enable_serde")]
    #[test]
    fn test_serde_db_hb_vel() {
        let original = DurHbVel::still();

        let serialized = serialize(&original).unwrap();
        let duplicate: DurHbVel = deserialize(&serialized).unwrap();
        assert_eq!(original, duplicate);
    }

    #[cfg(feature = "enable_serde")]
    #[test]
    fn test_serde_dur_hitbox() {
        let mut original = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -11.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            )),
        ));
        original.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        original.vel.duration =
            OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);

        let serialized = serialize(&original).unwrap();
        let duplicate: DurHitbox = deserialize(&serialized).unwrap();
        assert_eq!(original, duplicate);
    }

    #[test]
    fn test_rect_rect_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -11.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            )),
        ));
        a.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 12.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -0.5, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        b.vel.resize = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 7.0, Round::Up).0)
        );
        assert_eq!(
            b.collide_time(&a),
            OrdFloat::from(Float::with_val_round(prec_max(), 7.0, Round::Up).0)
        );
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_circle_circle_collision() {
        let sqrt2 = OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0.sqrt());
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -0.1, Round::Up).0) * sqrt2,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            )),
        ));
        a.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0) * sqrt2,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::circle(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0)
                    + sqrt2 * OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0),
            ),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        b.vel.resize = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -0.1, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -0.1, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert!(
            (a.collide_time(&b) - sqrt2).abs()
                < OrdFloat::from(Float::with_val_round(prec_max(), 1e-7, Round::Up).0)
        );
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_rect_circle_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -11.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            )),
        ));
        a.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 12.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 7.0, Round::Up).0)
        );
        assert_eq!(
            b.collide_time(&a),
            OrdFloat::from(Float::with_val_round(prec_max(), 7.0, Round::Up).0)
        );
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_rect_circle_angled_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
            ),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2., Round::Up).0,
            )),
        ));
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 5., Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 5., Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2., Round::Up).0,
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1., Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -1., Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let collide_time = a.collide_time(&b);
        let expected_time = OrdFloat::from(Float::with_val_round(prec_max(), 4., Round::Up).0)
            - OrdFloat::from(Float::with_val_round(prec_max(), 1., Round::Up).0)
                / OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0.sqrt());
        assert_eq!(collide_time, expected_time);
    }

    #[test]
    fn test_rect_rect_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 6.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            )),
        ));
        a.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.5, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 4.1, Round::Up).0)
        );
        assert_eq!(
            b.separate_time(
                &a,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 4.1, Round::Up).0)
        );
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_circle_circle_separation() {
        let sqrt2 = OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0.sqrt());
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            )),
        ));
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 1.8, Round::Up).0,
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0) + sqrt2
        );
        assert_eq!(
            b.separate_time(
                &a,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0) + sqrt2
        );
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_rect_circle_separation() {
        let sqrt2 = OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0.sqrt());
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 6.0, Round::Up).0),
            )),
        ));
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 3.8, Round::Up).0,
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0) + sqrt2
        );
        assert_eq!(
            b.separate_time(
                &a,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0) + sqrt2
        );
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_rect_circle_angled_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
            ),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2., Round::Up).0,
            )),
        ));
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1., Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1., Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2., Round::Up).0,
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1., Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -1., Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let separate_time = a.separate_time(
            &b,
            OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0),
        );
        let expected_time = OrdFloat::from(Float::with_val_round(prec_max(), 2., Round::Up).0)
            + OrdFloat::from(Float::with_val_round(prec_max(), 1.1, Round::Up).0)
                / OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0.sqrt());
        assert_eq!(separate_time, expected_time);
    }

    #[test]
    fn test_no_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -11.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            )),
        ));
        a.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 12.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );

        b.value.shape = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ));
        b.vel.resize = Vec2::zero();
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );

        a.value.shape = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ));
        a.vel.resize = Vec2::zero();
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_no_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            )),
        ));
        a.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            ),
            Shape::rect(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 100.0, Round::Up).0);
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );

        b.value.shape = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ));
        b.vel.resize = Vec2::zero();
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );

        a.value.shape = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ));
        a.vel.resize = Vec2::zero();
        assert_eq!(
            a.separate_time(
                &b,
                OrdFloat::from(Float::with_val_round(prec_max(), 0.1, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        );
    }

    #[test]
    fn test_low_duration() {
        let sqrt2 = OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0.sqrt());
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            )),
        ));
        a.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0)
            - sqrt2
            + OrdFloat::from(Float::with_val_round(prec_max(), 0.01, Round::Up).0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            ),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            )),
        ));
        b.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
        );
        b.vel.duration = OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0)
            - sqrt2
            + OrdFloat::from(Float::with_val_round(prec_max(), 0.01, Round::Up).0);
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0) - sqrt2
        );
        a.vel.duration -= OrdFloat::from(Float::with_val_round(prec_max(), 0.02, Round::Up).0);
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        b.vel.duration -= OrdFloat::from(Float::with_val_round(prec_max(), 0.02, Round::Up).0);
        assert_eq!(
            a.collide_time(&b),
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
    }
}
