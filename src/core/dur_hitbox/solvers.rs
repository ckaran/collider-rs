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

use crate::core::dur_hitbox::DurHitbox;
use crate::geom::shape::PlacedBounds;
use crate::geom::*;
use crate::util;
use num::BigRational;
use std::f64;

// This module contains methods to solve for the collision/separation time
// of two hitboxes.

pub fn collide_time(a: &DurHitbox, b: &DurHitbox) -> BigRational {
    let duration = a.vel.duration.min(b.vel.duration);
    if a.bounding_box_for(duration)
        .overlaps(&b.bounding_box_for(duration))
    {
        time_unpadded(a, b, true, duration)
    } else {
        BigRational::from_float(f64::INFINITY).unwrap()
    }
}

pub fn separate_time(a: &DurHitbox, b: &DurHitbox, padding: BigRational) -> BigRational {
    let (a, b) = match (a.value.kind(), b.value.kind()) {
        (ShapeKind::Rect, ShapeKind::Circle) => (b, a),
        _ => (a, b),
    };
    let mut a = *a;
    a.value.shape = Shape::new(
        a.value.kind(),
        a.value.dims() + v2(padding, padding) * BigRational::from_float(2.0).unwrap(),
    );
    time_unpadded(&a, b, false, a.vel.duration.min(b.vel.duration))
}

fn time_unpadded(
    a: &DurHitbox,
    b: &DurHitbox,
    for_collide: bool,
    duration: BigRational,
) -> BigRational {
    let result = match (a.value.kind(), b.value.kind()) {
        (ShapeKind::Rect, ShapeKind::Rect) => rect_rect_time(a, b, for_collide),
        (ShapeKind::Circle, ShapeKind::Circle) => circle_circle_time(a, b, for_collide),
        (ShapeKind::Rect, ShapeKind::Circle) => rect_circle_time(a, b, for_collide, duration),
        (ShapeKind::Circle, ShapeKind::Rect) => rect_circle_time(b, a, for_collide, duration),
    };
    if result >= duration {
        BigRational::from_float(f64::INFINITY).unwrap()
    } else {
        result
    }
}

fn rect_rect_time(a: &DurHitbox, b: &DurHitbox, for_collide: bool) -> BigRational {
    let mut overlap_start = BigRational::from_float(0.0).unwrap();
    let mut overlap_end = BigRational::from_float(f64::INFINITY).unwrap();
    for &card in &Card::values() {
        let overlap = a.value.card_overlap(&b.value, card);
        let overlap_vel = a.vel.card_overlap(&b.vel, card);
        if overlap < BigRational::from_float(0.0).unwrap() {
            if !for_collide {
                return BigRational::from_float(0.0).unwrap();
            } else if overlap_vel <= BigRational::from_float(0.0).unwrap() {
                return BigRational::from_float(f64::INFINITY).unwrap();
            } else {
                overlap_start = overlap_start.max(-overlap / overlap_vel);
            }
        } else if overlap_vel < BigRational::from_float(0.0).unwrap() {
            overlap_end = overlap_end.min(-overlap / overlap_vel);
        }
        if overlap_start >= overlap_end {
            return if for_collide {
                BigRational::from_float(f64::INFINITY).unwrap()
            } else {
                BigRational::from_float(0.0).unwrap()
            };
        }
    }
    if for_collide {
        overlap_start
    } else {
        overlap_end
    }
}

fn circle_circle_time(a: &DurHitbox, b: &DurHitbox, for_collide: bool) -> BigRational {
    let sign = if for_collide {
        BigRational::from_float(1.0).unwrap()
    } else {
        BigRational::from_float(-1.0).unwrap()
    };

    let net_rad = (a.value.dims().x + b.value.dims().x) * BigRational::from_float(0.5).unwrap();
    let dist = a.value.pos - b.value.pos;

    let coeff_c = sign * (net_rad * net_rad - dist.len_sq());
    if coeff_c > BigRational::from_float(0.0).unwrap() {
        return BigRational::from_float(0.0).unwrap();
    }

    let net_rad_vel = (a.vel.resize.x + b.vel.resize.x) * BigRational::from_float(0.5).unwrap();
    let dist_vel = a.vel.value - b.vel.value;

    let coeff_a = sign * (net_rad_vel * net_rad_vel - dist_vel.len_sq());
    let coeff_b =
        sign * BigRational::from_float(2.0).unwrap() * (net_rad * net_rad_vel - dist * dist_vel);

    match util::quad_root_ascending(coeff_a, coeff_b, coeff_c) {
        Some(result) if result >= BigRational::from_float(0.0).unwrap() => result,
        _ => BigRational::from_float(f64::INFINITY).unwrap(),
    }
}

fn rect_circle_time(
    rect: &DurHitbox,
    circle: &DurHitbox,
    for_collide: bool,
    duration: BigRational,
) -> BigRational {
    if for_collide {
        rect_circle_collide_time(rect, circle, duration)
    } else {
        rect_circle_separate_time(rect, circle)
    }
}

fn rect_circle_collide_time(
    rect: &DurHitbox,
    circle: &DurHitbox,
    duration: BigRational,
) -> BigRational {
    let base_time = rect_rect_time(rect, circle, true);
    if base_time >= duration {
        BigRational::from_float(f64::INFINITY).unwrap()
    } else {
        let mut rect = *rect;
        rect.value = rect.advanced_shape(base_time);
        let mut circle = *circle;
        circle.value = circle.advanced_shape(base_time);

        base_time + rebased_rect_circle_collide_time(&rect, &circle)
    }
}

fn rect_circle_separate_time(rect: &DurHitbox, circle: &DurHitbox) -> BigRational {
    let base_time = rect_rect_time(rect, circle, false);
    if base_time == BigRational::from_float(0.0).unwrap() {
        return BigRational::from_float(0.0).unwrap();
    }

    let mut rect = *rect;
    rect.value = rect.advanced_shape(base_time);
    rect.vel = rect.vel.negate();

    let mut circle = *circle;
    circle.value = circle.advanced_shape(base_time);
    circle.vel = circle.vel.negate();

    (base_time - rebased_rect_circle_collide_time(&rect, &circle))
        .max(BigRational::from_float(0.0).unwrap())
}

fn rebased_rect_circle_collide_time(rect: &DurHitbox, circle: &DurHitbox) -> BigRational {
    let sector = rect.value.sector(circle.value.pos);
    if sector.is_corner() {
        let mut corner = DurHitbox::new(PlacedShape::new(
            rect.value.corner(sector),
            Shape::circle(BigRational::from_float(0.0).unwrap()),
        ));
        corner.vel.value = rect.vel.corner(sector);
        circle_circle_time(&corner, circle, true)
    } else {
        BigRational::from_float(0.0).unwrap()
    }
}
