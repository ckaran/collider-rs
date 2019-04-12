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

use noisy_float::prelude::*;
use crate::core;
use crate::core::dur_hitbox::DurHitbox;
use crate::geom::shape::PlacedBounds;
use crate::geom::*;
use std::f64;
use crate::util;

// This module contains methods to solve for the collision/separation time
// of two hitboxes.

pub fn collide_time(a: &DurHitbox, b: &DurHitbox) -> N64 {
    let duration = a.vel.duration.min(b.vel.duration);
    if a.bounding_box_for(duration)
        .overlaps(&b.bounding_box_for(duration))
    {
        time_unpadded(a, b, true, duration)
    } else {
        n64(f64::INFINITY)
    }
}

pub fn separate_time(a: &DurHitbox, b: &DurHitbox, padding: N64) -> N64 {
    let (a, b) = match (a.value.kind(), b.value.kind()) {
        (ShapeKind::Rect, ShapeKind::Circle) => (b, a),
        _ => (a, b),
    };
    let mut a = a.clone();
    a.value.shape = Shape::new(a.value.kind(), a.value.dims() + v2(padding, padding) * n64(2.0));
    time_unpadded(&a, b, false, a.vel.duration.min(b.vel.duration))
}

fn time_unpadded(a: &DurHitbox, b: &DurHitbox, for_collide: bool, duration: N64) -> N64 {
    let result = match (a.value.kind(), b.value.kind()) {
        (ShapeKind::Rect, ShapeKind::Rect) => rect_rect_time(a, b, for_collide),
        (ShapeKind::Circle, ShapeKind::Circle) => circle_circle_time(a, b, for_collide),
        (ShapeKind::Rect, ShapeKind::Circle) => rect_circle_time(a, b, for_collide, duration),
        (ShapeKind::Circle, ShapeKind::Rect) => rect_circle_time(b, a, for_collide, duration),
    };
    if result >= duration {
        n64(f64::INFINITY)
    } else {
        result
    }
}

fn rect_rect_time(a: &DurHitbox, b: &DurHitbox, for_collide: bool) -> N64 {
    let mut overlap_start = n64(0.0);
    let mut overlap_end = n64(f64::INFINITY);
    for &card in &Card::values() {
        let overlap = a.value.card_overlap(&b.value, card);
        let overlap_vel = a.vel.card_overlap(&b.vel, card);
        if overlap < n64(0.0) {
            if !for_collide {
                return n64(0.0);
            } else if overlap_vel <= n64(0.0) {
                return n64(f64::INFINITY);
            } else {
                overlap_start = overlap_start.max(-overlap / overlap_vel);
            }
        } else if overlap_vel < n64(0.0) {
            overlap_end = overlap_end.min(-overlap / overlap_vel);
        }
        if overlap_start >= overlap_end {
            return if for_collide { n64(f64::INFINITY) } else { n64(0.0) };
        }
    }
    if for_collide {
        overlap_start
    } else {
        overlap_end
    }
}

fn circle_circle_time(a: &DurHitbox, b: &DurHitbox, for_collide: bool) -> N64 {
    let sign = if for_collide { n64(1.0) } else { n64(-1.0) };

    let net_rad = (a.value.dims().x + b.value.dims().x) * n64(0.5);
    let dist = a.value.pos - b.value.pos;

    let coeff_c = sign * (net_rad * net_rad - dist.len_sq());
    if coeff_c > n64(0.0) {
        return n64(0.0);
    }

    let net_rad_vel = (a.vel.resize.x + b.vel.resize.x) * n64(0.5);
    let dist_vel = a.vel.value - b.vel.value;

    let coeff_a = sign * (net_rad_vel * net_rad_vel - dist_vel.len_sq());
    let coeff_b = sign * n64(2.0) * (net_rad * net_rad_vel - dist * dist_vel);

    match util::quad_root_ascending(coeff_a, coeff_b, coeff_c) {
        Some(result) if result >= n64(0.0) => result,
        _ => n64(f64::INFINITY),
    }
}

fn rect_circle_time(rect: &DurHitbox, circle: &DurHitbox, for_collide: bool, duration: N64) -> N64 {
    if for_collide {
        rect_circle_collide_time(rect, circle, duration)
    } else {
        rect_circle_separate_time(rect, circle)
    }
}

fn rect_circle_collide_time(rect: &DurHitbox, circle: &DurHitbox, duration: N64) -> N64 {
    let base_time = rect_rect_time(rect, circle, true);
    if base_time >= duration {
        n64(f64::INFINITY)
    } else {
        let mut rect = rect.clone();
        rect.value = rect.advanced_shape(base_time);
        let mut circle = circle.clone();
        circle.value = circle.advanced_shape(base_time);

        base_time + rebased_rect_circle_collide_time(&rect, &circle)
    }
}

fn rect_circle_separate_time(rect: &DurHitbox, circle: &DurHitbox) -> N64 {
    let base_time = rect_rect_time(rect, circle, false);
    if base_time == n64(0.0) {
        return n64(0.0);
    }
    if base_time >= n64(core::HIGH_TIME) {
        return n64(f64::INFINITY);
    }

    let mut rect = rect.clone();
    rect.value = rect.advanced_shape(base_time);
    rect.vel = rect.vel.negate();

    let mut circle = circle.clone();
    circle.value = circle.advanced_shape(base_time);
    circle.vel = circle.vel.negate();

    (base_time - rebased_rect_circle_collide_time(&rect, &circle)).max(n64(0.0))
}

fn rebased_rect_circle_collide_time(rect: &DurHitbox, circle: &DurHitbox) -> N64 {
    let sector = rect.value.sector(circle.value.pos);
    if sector.is_corner() {
        let mut corner = DurHitbox::new(PlacedShape::new(
            rect.value.corner(sector),
            Shape::circle(n64(0.0)),
        ));
        corner.vel.value = rect.vel.corner(sector);
        circle_circle_time(&corner, circle, true)
    } else {
        n64(0.0)
    }
}
