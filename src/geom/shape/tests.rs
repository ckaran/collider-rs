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

use crate::geom::*;
use noisy_float::prelude::*;

#[test]
fn test_circle_advance() {
    let shape_1 = Shape::circle(n64(2.0)).place(v2(n64(3.0), n64(5.0)));
    assert_eq!(
        shape_1.advance(v2(n64(1.0), n64(2.0)), v2(n64(-0.25), n64(-0.25)), n64(2.0)),
        Shape::circle(n64(1.5)).place(v2(n64(5.0), n64(9.0)))
    );
}

#[test]
fn test_rect_advance() {
    let shape_1 = Shape::rect(v2(n64(2.0), n64(5.0))).place(v2(n64(3.0), n64(5.0)));
    assert_eq!(
        shape_1.advance(v2(n64(1.0), n64(2.0)), v2(n64(-0.25), n64(1.0)), n64(2.0)),
        Shape::rect(v2(n64(1.5), n64(7.0))).place(v2(n64(5.0), n64(9.0)))
    );
}

#[test]
#[should_panic]
fn test_illegal_circle_advance() {
    let shape = Shape::circle(n64(2.0)).place(v2(n64(3.0), n64(5.0)));
    shape.advance(v2(n64(1.0), n64(2.0)), v2(n64(-0.25), n64(-0.24)), n64(2.0));
}

#[test]
fn test_edges() {
    let shape = Shape::rect(v2(n64(4.0), n64(6.0))).place(v2(n64(3.0), n64(5.0)));
    assert_eq!(shape.min_x(), n64(1.0));
    assert_eq!(shape.min_y(), n64(2.0));
    assert_eq!(shape.max_x(), n64(5.0));
    assert_eq!(shape.max_y(), n64(8.0));
}

#[test]
fn test_rect_rect_normal() {
    let src = Shape::rect(v2(n64(4.0), n64(4.0))).place(v2(n64(1.0), n64(1.0)));
    let dst = Shape::rect(v2(n64(8.0), n64(8.0))).place(v2(n64(2.0), n64(1.5)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(1.0), n64(0.0)), n64(5.0))
    );
    let dst = Shape::rect(v2(n64(8.0), n64(8.0))).place(v2(n64(0.0), n64(0.5)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(-1.0), n64(0.0)), n64(5.0))
    );
    let dst = Shape::rect(v2(n64(4.0), n64(2.0))).place(v2(n64(3.8), n64(4.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(0.0), n64(1.0)), n64(0.0))
    );
    let dst = Shape::rect(v2(n64(8.0), n64(2.0))).place(v2(n64(-2.0), n64(-3.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(0.0), n64(-1.0)), n64(-1.0))
    );
}

#[test]
fn test_circle_circle_normal() {
    let src = Shape::circle(n64(2.0)).place(v2(n64(1.0), n64(1.0)));
    let dst = Shape::circle(n64(3.0)).place(v2(n64(2.0), n64(0.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(1.0), n64(-1.0)), n64(2.5) - n64(2.0).sqrt())
    );
}

#[test]
fn test_rect_circle_normal() {
    let src = Shape::rect(v2(n64(2.0), n64(2.0))).place(v2(n64(0.0), n64(0.0)));

    let dst = Shape::circle(n64(2.5)).place(v2(n64(-2.0), n64(0.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(-1.0), n64(0.0)), n64(0.25))
    );
    let dst = Shape::circle(n64(2.5)).place(v2(n64(0.0), n64(-2.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(0.0), n64(-1.0)), n64(0.25))
    );
    let dst = Shape::circle(n64(2.5)).place(v2(n64(2.0), n64(0.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(1.0), n64(0.0)), n64(0.25))
    );
    let dst = Shape::circle(n64(2.5)).place(v2(n64(0.0), n64(2.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(0.0), n64(1.0)), n64(0.25))
    );

    let dst = Shape::circle(n64(2.5)).place(v2(n64(-2.0), n64(-2.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(-1.0), n64(-1.0)), n64(1.25) - n64(2.0).sqrt())
    );
    let dst = Shape::circle(n64(2.5)).place(v2(n64(2.0), n64(-2.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(1.0), n64(-1.0)), n64(1.25) - n64(2.0).sqrt())
    );
    let dst = Shape::circle(n64(2.5)).place(v2(n64(-2.0), n64(2.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(-1.0), n64(1.0)), n64(1.25) - n64(2.0).sqrt())
    );
    let dst = Shape::circle(n64(2.5)).place(v2(n64(2.0), n64(2.0)));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(v2(n64(1.0), n64(1.0)), n64(1.25) - n64(2.0).sqrt())
    );
}

#[test]
fn test_masked_rect_rect_normal() {
    let src = Shape::rect(v2(n64(4.0), n64(4.0))).place(v2(n64(1.0), n64(1.0)));
    let dst = Shape::rect(v2(n64(8.0), n64(8.0))).place(v2(n64(6.0), n64(-5.0)));
    let mut mask = CardMask::full();
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(v2(n64(0.0), n64(-1.0)), n64(0.0))
    );
    mask[Card::MinusY] = false;
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(v2(n64(1.0), n64(0.0)), n64(1.0))
    );
}

#[test]
fn test_masked_rect_circle_normal() {
    let src = Shape::rect(v2(n64(2.0), n64(2.0))).place(v2(n64(0.0), n64(0.0)));
    let dst = Shape::circle(n64(2.5)).place(v2(n64(-2.0), n64(2.0)));
    let mut mask = CardMask::full();
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(v2(n64(-1.0), n64(1.0)), n64(1.25) - n64(2.0).sqrt())
    );
    mask[Card::PlusX] = false;
    assert_eq!(
        src.masked_normal_from(&dst, mask.flip()),
        DirVec2::new(v2(n64(1.0), n64(-1.0)), n64(1.25) - n64(2.0).sqrt())
    );
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(v2(n64(-1.0), n64(1.0)), n64(1.25) - n64(2.0).sqrt())
    );
    mask[Card::PlusY] = false;
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(v2(n64(-1.0), n64(0.0)), n64(0.25))
    );
}

#[test]
fn test_rect_rect_contact() {
    let a = Shape::rect(v2(n64(4.0), n64(2.0))).place(v2(n64(4.0), n64(10.0)));
    let b = Shape::rect(v2(n64(2.0), n64(4.0))).place(v2(n64(-2.0), n64(12.0)));
    assert_eq!(a.contact_point(&b), v2(n64(0.5), n64(10.5)));
    assert_eq!(b.contact_point(&a), v2(n64(0.5), n64(10.5)));
}

#[test]
fn test_circle_circle_contact() {
    let a = Shape::circle(n64(2.0)).place(v2(n64(5.0), n64(15.0)));
    let b = Shape::circle(n64(8.0)).place(v2(n64(5.0), n64(19.0)));
    assert_eq!(a.contact_point(&b), v2(n64(5.0), n64(15.5)));
    assert_eq!(b.contact_point(&a), v2(n64(5.0), n64(15.5)));
}

#[test]
fn test_circle_rect_contact() {
    let a = Shape::circle(n64(2.0)).place(v2(n64(5.0), n64(15.0)));
    let b = Shape::rect(v2(n64(4.0), n64(8.0))).place(v2(n64(2.0), n64(18.0)));
    assert_eq!(a.contact_point(&b), v2(n64(4.0), n64(15.0)));
    assert_eq!(b.contact_point(&a), v2(n64(4.0), n64(15.0)));
}
