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

//! Collider is a library for continuous 2D collision detection, for use with
//! game developement.
//!
//! Most game engines follow the approach of periodically updating the positions
//! of all shapes and checking for collisions at a frozen snapshot in time.
//! [Continuous collision detection](https://en.wikipedia.org/wiki/Collision_detection#A_posteriori_.28discrete.29_versus_a_priori_.28continuous.29),
//! on the other hand, means that the time of collision is determined very
//! precisely, and the user is not restricted to a fixed time-stepping method.
//! There are currently two kinds of shapes supported by Collider: circles and
//! rectangles. The user specifies the positions and velocities of these shapes,
//! which they can update at any time, and Collider will solve for the precise
//! times of collision and separation.
//!
//! There are certain advantages that continuous collision detection holds over
//! the traditional approach. In a game engine, the position of a sprite may be
//! updated to overlap a wall, and in a traditional collision system there would
//! need to be a post-correction to make sure the sprite does not appear inside
//! of the wall. This is not needed with continuous collision detection, since
//! the precise time and location at which the sprite touches the wall is known.
//! Traditional collision detection may have an issue with "tunneling," in which
//! a fast small object runs into a narrow wall and collision detection misses
//! it, or two fast small objects fly right through each other and collision
//! detection misses it. This is also not a problem for continuous collision
//! detection. It is also debatable that continuous collision detection may be
//! more efficient in certain circumstances, since the hitboxes may be updated
//! less frequently and still maintain a smooth appearance over time.
//!
//! #Example
//! ```
//! use collider::{Collider, HbEvent, HbId, HbProfile};
//! use collider::geom::{Shape, v2};
//! use num::BigRational;
//!
//! #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
//! struct DemoHbProfile { id: HbId } // add any additional identfying data to this struct
//!
//! impl HbProfile for DemoHbProfile {
//!     fn id(&self) -> HbId { self.id }
//!     fn can_interact(&self, _other: &DemoHbProfile) -> bool { true }
//! }
//!
//! let mut collider: Collider<DemoHbProfile> = Collider::new(BigRational::from_float(4.0).unwrap(), BigRational::from_float(0.01).unwrap());
//!
//! let hitbox = Shape::square(BigRational::from_float(2.0).unwrap()).place(v2(BigRational::from_float(-10.0).unwrap(), BigRational::from_float(0.0).unwrap())).moving(v2(BigRational::from_float(1.0).unwrap(), BigRational::from_float(0.0).unwrap()));
//! let overlaps = collider.add_hitbox(DemoHbProfile { id: 0 }, hitbox);
//! assert!(overlaps.is_empty());
//!
//! let hitbox = Shape::square(BigRational::from_float(2.0).unwrap()).place(v2(BigRational::from_float(10.0).unwrap(), BigRational::from_float(0.0).unwrap())).moving(v2(BigRational::from_float(-1.0).unwrap(), BigRational::from_float(0.0).unwrap()));
//! let overlaps = collider.add_hitbox(DemoHbProfile { id: 1 }, hitbox);
//! assert!(overlaps.is_empty());
//!
//! while collider.time() < BigRational::from_float(20.0).unwrap() {
//!     let time = collider.next_time().min(BigRational::from_float(20.0).unwrap());
//!     collider.set_time(time);
//!     if let Some((event, profile_1, profile_2)) = collider.next() {
//!         println!("{:?} between {:?} and {:?} at time {}.",
//!                  event, profile_1, profile_2, collider.time());
//!         if event == HbEvent::Collide {
//!             println!("Speed of collided hitboxes is halved.");
//!             for profile in [profile_1, profile_2].iter() {
//!                 let mut hb_vel = collider.get_hitbox(profile.id()).vel;
//!                 hb_vel.value *= BigRational::from_float(0.5).unwrap();
//!                 collider.set_hitbox_vel(profile.id(), hb_vel);
//!             }
//!         }
//!     }
//! }
//!
//! // the above loop prints the following events:
//! //   Collide between DemoHbProfile { id: 0 } and DemoHbProfile { id: 1 } at time BigRational::from_float(9.).unwrap()
//! //   Speed of collided hitboxes is halved.
//! //   Separate between DemoHbProfile { id: 0 } and DemoHbProfile { id: 1 } at time BigRational::from_float(13.01).unwrap().
//! ```

extern crate fnv;

mod core;
pub mod geom;
mod index_rect;
#[cfg(test)]
mod tests;
mod util;

pub use crate::core::*;
