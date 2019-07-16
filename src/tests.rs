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

#[cfg(feature = "enable_serde")]
extern crate serde;
#[cfg(feature = "enable_serde")]
use self::serde::*;

use super::{Collider, HbEvent, HbId, HbProfile, HbVel};
use crate::geom::{v2, Shape};
use rug::{
    float,
    float::{prec_max, OrdFloat, Round},
    Float,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
struct TestHbProfile {
    id: HbId,
}

impl From<HbId> for TestHbProfile {
    fn from(id: HbId) -> TestHbProfile {
        TestHbProfile { id }
    }
}

impl HbProfile for TestHbProfile {
    fn id(&self) -> HbId {
        self.id
    }
    fn can_interact(&self, _other: &TestHbProfile) -> bool {
        true
    }
}

fn advance_to_event(collider: &mut Collider<TestHbProfile>, time: OrdFloat) {
    advance(collider, time);
    assert_eq!(collider.next_time(), collider.time());
}

fn advance(collider: &mut Collider<TestHbProfile>, time: OrdFloat) {
    while collider.time() < time {
        assert!(collider.next().is_none());
        let new_time = collider.next_time().min(time);
        collider.set_time(new_time);
    }
    assert_eq!(collider.time(), time);
}

fn advance_through_events(collider: &mut Collider<TestHbProfile>, time: OrdFloat) {
    while collider.time() < time {
        collider.next();
        let new_time = collider.next_time().min(time);
        collider.set_time(new_time);
    }
    assert_eq!(collider.time(), time);
}

fn sort(mut vector: Vec<TestHbProfile>) -> Vec<TestHbProfile> {
    vector.sort();
    vector
}

#[test]
fn smoke_test() {
    let mut collider = Collider::<TestHbProfile>::new(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
    );

    let mut hitbox = Shape::square(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -10.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ))
    .still();
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    );
    let overlaps = collider.add_hitbox(0.into(), hitbox);
    assert_eq!(overlaps, vec![]);

    let mut hitbox = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ))
    .still();
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    );
    let overlaps = collider.add_hitbox(1.into(), hitbox);
    assert_eq!(overlaps, vec![]);

    advance_to_event(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 9.0, Round::Up).0),
    );
    assert_eq!(
        collider.next(),
        Some((HbEvent::Collide, 0.into(), 1.into()))
    );
    advance_to_event(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 11.125, Round::Up).0),
    );
    assert_eq!(
        collider.next(),
        Some((HbEvent::Separate, 0.into(), 1.into()))
    );
    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 23.0, Round::Up).0),
    );
}

#[test]
fn test_hitbox_updates() {
    let mut collider = Collider::<TestHbProfile>::new(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
    );

    let mut hitbox = Shape::square(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -10.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ))
    .still();
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    );
    let overlaps = collider.add_hitbox(0.into(), hitbox);
    assert!(overlaps.is_empty());

    let mut hitbox = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ))
    .still();
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    );
    let overlaps = collider.add_hitbox(1.into(), hitbox);
    assert!(overlaps.is_empty());

    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 11.0, Round::Up).0),
    );

    let mut hitbox = collider.get_hitbox(0);
    assert_eq!(
        hitbox.value,
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        ))
    );
    assert_eq!(
        hitbox.vel.value,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.resize,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.end_time,
        OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
    );
    hitbox.value.pos = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    );
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
    );
    let overlaps = collider.remove_hitbox(0);
    assert_eq!(overlaps, vec![]);
    let overlaps = collider.add_hitbox(0.into(), hitbox);
    assert_eq!(overlaps, vec![]);

    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 14.0, Round::Up).0),
    );

    let mut hitbox = collider.get_hitbox(1);
    assert_eq!(
        hitbox.value,
        Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 24.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        ))
    );
    assert_eq!(
        hitbox.vel.value,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.resize,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.end_time,
        OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
    );
    hitbox.value.pos = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -8.0, Round::Up).0),
    );
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    );
    let overlaps = collider.remove_hitbox(1);
    assert_eq!(overlaps, vec![]);
    let overlaps = collider.add_hitbox(1.into(), hitbox);
    assert_eq!(overlaps, vec![]);

    advance_to_event(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 19.0, Round::Up).0),
    );

    assert_eq!(
        collider.next(),
        Some((HbEvent::Collide, 0.into(), 1.into()))
    );
    let mut hitbox = collider.get_hitbox(0);
    assert_eq!(
        hitbox.value,
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -6.0, Round::Up).0)
        ))
    );
    assert_eq!(
        hitbox.vel.value,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.resize,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.end_time,
        OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
    );
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    );
    collider.set_hitbox_vel(0, hitbox.vel);

    let mut hitbox = collider.get_hitbox(1);
    assert_eq!(
        hitbox.value,
        Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -8.0, Round::Up).0)
        ))
    );
    assert_eq!(
        hitbox.vel.value,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.resize,
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    assert_eq!(
        hitbox.vel.end_time,
        OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
    );
    hitbox.vel.value = v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    );
    collider.set_hitbox_vel(1, hitbox.vel);

    let hitbox = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 20.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ))
    .still();
    assert_eq!(
        sort(collider.add_hitbox(2.into(), hitbox)),
        vec![0.into(), 1.into()]
    );

    advance_to_event(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 21.125, Round::Up).0),
    );

    assert_eq!(
        collider.next(),
        Some((HbEvent::Separate, 0.into(), 1.into()))
    );

    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 26.125, Round::Up).0),
    );

    let overlaps = collider.remove_hitbox(1);
    assert_eq!(overlaps, vec![2.into()]);

    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 37.125, Round::Up).0),
    );
}

#[test]
fn test_get_overlaps() {
    let mut collider = Collider::<TestHbProfile>::new(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
    );

    collider.add_hitbox(
        0.into(),
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .moving(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        )),
    );
    collider.add_hitbox(
        1.into(),
        Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .moving(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        )),
    );
    collider.add_hitbox(
        2.into(),
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still(),
    );

    assert_eq!(collider.get_overlaps(0), vec![]);
    assert_eq!(collider.get_overlaps(1), vec![]);
    assert_eq!(collider.get_overlaps(2), vec![]);
    assert!(!collider.is_overlapping(0, 1));
    assert!(!collider.is_overlapping(0, 2));
    assert!(!collider.is_overlapping(1, 2));
    assert!(!collider.is_overlapping(1, 0));

    advance_through_events(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
    );

    assert_eq!(sort(collider.get_overlaps(0)), vec![1.into(), 2.into()]);
    assert_eq!(sort(collider.get_overlaps(1)), vec![0.into(), 2.into()]);
    assert_eq!(sort(collider.get_overlaps(2)), vec![0.into(), 1.into()]);
    assert!(collider.is_overlapping(0, 1));
    assert!(collider.is_overlapping(0, 2));
    assert!(collider.is_overlapping(1, 2));
    assert!(collider.is_overlapping(1, 0));

    collider.set_hitbox_vel(
        1,
        HbVel::moving(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        )),
    );
    advance_through_events(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 20.0, Round::Up).0),
    );

    assert_eq!(collider.get_overlaps(0), vec![1.into()]);
    assert_eq!(collider.get_overlaps(1), vec![0.into()]);
    assert_eq!(collider.get_overlaps(2), vec![]);
    assert!(collider.is_overlapping(0, 1));
    assert!(!collider.is_overlapping(0, 2));
    assert!(!collider.is_overlapping(1, 2));

    collider.remove_hitbox(2);
    assert_eq!(collider.get_overlaps(0), vec![1.into()]);
    assert_eq!(collider.get_overlaps(1), vec![0.into()]);
    assert!(collider.is_overlapping(0, 1));

    collider.remove_hitbox(1);
    assert_eq!(collider.get_overlaps(0), vec![]);
}

#[test]
fn test_query_overlaps() {
    let mut collider = Collider::<TestHbProfile>::new(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
    );

    collider.add_hitbox(
        0.into(),
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -5.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .moving(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        )),
    );
    collider.add_hitbox(
        1.into(),
        Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still(),
    );
    collider.add_hitbox(
        2.into(),
        Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .moving(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        )),
    );

    let test_shape = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.5, Round::Up).0),
    ));
    assert_eq!(
        collider.query_overlaps(&test_shape, &5.into()),
        vec![1.into()]
    );

    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
    );
    assert_eq!(
        sort(collider.query_overlaps(&test_shape, &5.into())),
        vec![0.into(), 1.into()]
    );
}

#[test]
fn test_separate_initial_overlap() {
    let mut collider = Collider::<TestHbProfile>::new(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
    );

    let overlaps = collider.add_hitbox(
        0.into(),
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 1., Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
        ))
        .moving(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 1., Round::Up).0),
        )),
    );
    assert_eq!(overlaps, vec![]);
    let overlaps = collider.add_hitbox(
        1.into(),
        Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 1., Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
        ))
        .still(),
    );
    assert_eq!(overlaps, vec![0.into()]);

    advance_to_event(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0),
    );
    assert_eq!(
        collider.next(),
        Some((HbEvent::Separate, 0.into(), 1.into()))
    );

    advance(
        &mut collider,
        OrdFloat::from(Float::with_val_round(prec_max(), 1.5, Round::Up).0),
    );
}

#[cfg(all(test, feature = "enable_serde"))]
pub(crate) mod test_serde {
    use super::*;
    use bincode::{deserialize, serialize};

    #[test]
    fn test_trivial() {
        let collider = Collider::<TestHbProfile>::new(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
        );

        let serialized = serialize(&collider).unwrap();
        let duplicate: Collider<TestHbProfile> = deserialize(&serialized).unwrap();
        assert_eq!(collider, duplicate);
    }

    #[test]
    fn smoke_test() {
        let mut collider = Collider::<TestHbProfile>::new(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
        );

        let mut hitbox = Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still();
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        let overlaps = collider.add_hitbox(0.into(), hitbox);
        assert_eq!(overlaps, vec![]);

        let mut hitbox = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still();
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        let overlaps = collider.add_hitbox(1.into(), hitbox);
        assert_eq!(overlaps, vec![]);

        advance_to_event(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 9.0, Round::Up).0),
        );
        assert_eq!(
            collider.next(),
            Some((HbEvent::Collide, 0.into(), 1.into()))
        );
        advance_to_event(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 11.125, Round::Up).0),
        );
        assert_eq!(
            collider.next(),
            Some((HbEvent::Separate, 0.into(), 1.into()))
        );
        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 23.0, Round::Up).0),
        );

        let serialized = serialize(&collider).unwrap();
        let duplicate: Collider<TestHbProfile> = deserialize(&serialized).unwrap();
        assert_eq!(collider, duplicate);
    }

    #[test]
    fn test_hitbox_updates() {
        let mut collider = Collider::<TestHbProfile>::new(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
        );

        let mut hitbox = Shape::square(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still();
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        let overlaps = collider.add_hitbox(0.into(), hitbox);
        assert!(overlaps.is_empty());

        let mut hitbox = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still();
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        let overlaps = collider.add_hitbox(1.into(), hitbox);
        assert!(overlaps.is_empty());

        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 11.0, Round::Up).0),
        );

        let mut hitbox = collider.get_hitbox(0);
        assert_eq!(
            hitbox.value,
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ))
        );
        assert_eq!(
            hitbox.vel.value,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.resize,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.end_time,
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        hitbox.value.pos = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        );
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
        );
        let overlaps = collider.remove_hitbox(0);
        assert_eq!(overlaps, vec![]);
        let overlaps = collider.add_hitbox(0.into(), hitbox);
        assert_eq!(overlaps, vec![]);

        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 14.0, Round::Up).0),
        );

        let mut hitbox = collider.get_hitbox(1);
        assert_eq!(
            hitbox.value,
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 24.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ))
        );
        assert_eq!(
            hitbox.vel.value,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.resize,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.end_time,
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        hitbox.value.pos = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -8.0, Round::Up).0),
        );
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        let overlaps = collider.remove_hitbox(1);
        assert_eq!(overlaps, vec![]);
        let overlaps = collider.add_hitbox(1.into(), hitbox);
        assert_eq!(overlaps, vec![]);

        advance_to_event(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 19.0, Round::Up).0),
        );

        assert_eq!(
            collider.next(),
            Some((HbEvent::Collide, 0.into(), 1.into()))
        );
        let mut hitbox = collider.get_hitbox(0);
        assert_eq!(
            hitbox.value,
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -6.0, Round::Up).0)
            ))
        );
        assert_eq!(
            hitbox.vel.value,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.resize,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.end_time,
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        );
        collider.set_hitbox_vel(0, hitbox.vel);

        let mut hitbox = collider.get_hitbox(1);
        assert_eq!(
            hitbox.value,
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -8.0, Round::Up).0)
            ))
        );
        assert_eq!(
            hitbox.vel.value,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.resize,
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            )
        );
        assert_eq!(
            hitbox.vel.end_time,
            OrdFloat::from(Float::with_val(prec_max(), float::Special::Infinity))
        );
        hitbox.vel.value = v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        );
        collider.set_hitbox_vel(1, hitbox.vel);

        let hitbox = Shape::rect(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 20.0, Round::Up).0),
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        ))
        .still();
        assert_eq!(
            sort(collider.add_hitbox(2.into(), hitbox)),
            vec![0.into(), 1.into()]
        );

        advance_to_event(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 21.125, Round::Up).0),
        );

        assert_eq!(
            collider.next(),
            Some((HbEvent::Separate, 0.into(), 1.into()))
        );

        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 26.125, Round::Up).0),
        );

        let overlaps = collider.remove_hitbox(1);
        assert_eq!(overlaps, vec![2.into()]);

        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 37.125, Round::Up).0),
        );

        let serialized = serialize(&collider).unwrap();
        let duplicate: Collider<TestHbProfile> = deserialize(&serialized).unwrap();
        assert_eq!(collider, duplicate);
    }

    #[test]
    fn test_get_overlaps() {
        let mut collider = Collider::<TestHbProfile>::new(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
        );

        collider.add_hitbox(
            0.into(),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -10.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ))
            .moving(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            )),
        );
        collider.add_hitbox(
            1.into(),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ))
            .moving(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            )),
        );
        collider.add_hitbox(
            2.into(),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ))
            .still(),
        );

        assert_eq!(collider.get_overlaps(0), vec![]);
        assert_eq!(collider.get_overlaps(1), vec![]);
        assert_eq!(collider.get_overlaps(2), vec![]);
        assert!(!collider.is_overlapping(0, 1));
        assert!(!collider.is_overlapping(0, 2));
        assert!(!collider.is_overlapping(1, 2));
        assert!(!collider.is_overlapping(1, 0));

        advance_through_events(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
        );

        assert_eq!(sort(collider.get_overlaps(0)), vec![1.into(), 2.into()]);
        assert_eq!(sort(collider.get_overlaps(1)), vec![0.into(), 2.into()]);
        assert_eq!(sort(collider.get_overlaps(2)), vec![0.into(), 1.into()]);
        assert!(collider.is_overlapping(0, 1));
        assert!(collider.is_overlapping(0, 2));
        assert!(collider.is_overlapping(1, 2));
        assert!(collider.is_overlapping(1, 0));

        collider.set_hitbox_vel(
            1,
            HbVel::moving(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            )),
        );
        advance_through_events(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 20.0, Round::Up).0),
        );

        assert_eq!(collider.get_overlaps(0), vec![1.into()]);
        assert_eq!(collider.get_overlaps(1), vec![0.into()]);
        assert_eq!(collider.get_overlaps(2), vec![]);
        assert!(collider.is_overlapping(0, 1));
        assert!(!collider.is_overlapping(0, 2));
        assert!(!collider.is_overlapping(1, 2));

        collider.remove_hitbox(2);
        assert_eq!(collider.get_overlaps(0), vec![1.into()]);
        assert_eq!(collider.get_overlaps(1), vec![0.into()]);
        assert!(collider.is_overlapping(0, 1));

        collider.remove_hitbox(1);
        assert_eq!(collider.get_overlaps(0), vec![]);

        let serialized = serialize(&collider).unwrap();
        let duplicate: Collider<TestHbProfile> = deserialize(&serialized).unwrap();
        assert_eq!(collider, duplicate);
    }

    #[test]
    fn test_query_overlaps() {
        let mut collider = Collider::<TestHbProfile>::new(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
        );

        collider.add_hitbox(
            0.into(),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -5.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ))
            .moving(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            )),
        );
        collider.add_hitbox(
            1.into(),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ))
            .still(),
        );
        collider.add_hitbox(
            2.into(),
            Shape::circle(OrdFloat::from(
                Float::with_val_round(prec_max(), 2.0, Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            ))
            .moving(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
            )),
        );

        let test_shape = Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 2.0, Round::Up).0,
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.5, Round::Up).0),
        ));
        assert_eq!(
            collider.query_overlaps(&test_shape, &5.into()),
            vec![1.into()]
        );

        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
        );
        assert_eq!(
            sort(collider.query_overlaps(&test_shape, &5.into())),
            vec![0.into(), 1.into()]
        );

        let serialized = serialize(&collider).unwrap();
        let duplicate: Collider<TestHbProfile> = deserialize(&serialized).unwrap();
        assert_eq!(collider, duplicate);
    }

    #[test]
    fn test_separate_initial_overlap() {
        let mut collider = Collider::<TestHbProfile>::new(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0),
        );

        let overlaps = collider.add_hitbox(
            0.into(),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 1., Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
            ))
            .moving(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1., Round::Up).0),
            )),
        );
        assert_eq!(overlaps, vec![]);
        let overlaps = collider.add_hitbox(
            1.into(),
            Shape::square(OrdFloat::from(
                Float::with_val_round(prec_max(), 1., Round::Up).0,
            ))
            .place(v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0., Round::Up).0),
            ))
            .still(),
        );
        assert_eq!(overlaps, vec![0.into()]);

        advance_to_event(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0),
        );
        assert_eq!(
            collider.next(),
            Some((HbEvent::Separate, 0.into(), 1.into()))
        );

        advance(
            &mut collider,
            OrdFloat::from(Float::with_val_round(prec_max(), 1.5, Round::Up).0),
        );

        let serialized = serialize(&collider).unwrap();
        let duplicate: Collider<TestHbProfile> = deserialize(&serialized).unwrap();
        assert_eq!(collider, duplicate);
    }
}

//TODO test custom interactivities...
