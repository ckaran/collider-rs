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
use rug::{
    float::{prec_max, OrdFloat, Round},
    Float,
};

#[test]
fn test_circle_advance() {
    let shape_1 = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
    ));
    assert_eq!(
        shape_1.advance(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0)
            ),
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -0.25, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -0.25, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0)
        ),
        Shape::circle(OrdFloat::from(
            Float::with_val_round(prec_max(), 1.5, Round::Up).0
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 9.0, Round::Up).0)
        ))
    );
}

#[test]
fn test_rect_advance() {
    let shape_1 = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
    ));
    assert_eq!(
        shape_1.advance(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0)
            ),
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -0.25, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0)
        ),
        Shape::rect(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.5, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 7.0, Round::Up).0)
        ))
        .place(v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 9.0, Round::Up).0)
        ))
    );
}

#[test]
#[should_panic]
fn test_illegal_circle_advance() {
    let shape = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
    ));
    shape.advance(
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        ),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), -0.25, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), -0.24, Round::Up).0),
        ),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    );
}

#[test]
fn test_edges() {
    let shape = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 6.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 3.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
    ));
    assert_eq!(
        shape.min_x(),
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
    );
    assert_eq!(
        shape.min_y(),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0)
    );
    assert_eq!(
        shape.max_x(),
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0)
    );
    assert_eq!(
        shape.max_y(),
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0)
    );
}

#[test]
fn test_rect_rect_normal() {
    let src = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
    ));
    let dst = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 1.5, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0)
        )
    );
    let dst = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.5, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0)
        )
    );
    let dst = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 3.8, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    let dst = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -3.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
        )
    );
}

#[test]
fn test_circle_circle_normal() {
    let src = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
    ));
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 3.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 2.5, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
}

#[test]
fn test_rect_circle_normal() {
    let src = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ));

    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0)
        )
    );
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0)
        )
    );
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0)
        )
    );
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0)
        )
    );

    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
}

#[test]
fn test_masked_rect_rect_normal() {
    let src = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
    ));
    let dst = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 6.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), -5.0, Round::Up).0),
    ));
    let mut mask = CardMask::full();
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
        )
    );
    mask[Card::MinusY] = false;
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
        )
    );
}

#[test]
fn test_masked_rect_circle_normal() {
    let src = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0),
    ));
    let dst = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.5, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ));
    let mut mask = CardMask::full();
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
    mask[Card::PlusX] = false;
    assert_eq!(
        src.masked_normal_from(&dst, mask.flip()),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 1.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 1.25, Round::Up).0)
                - OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0).sqrt()
        )
    );
    mask[Card::PlusY] = false;
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                OrdFloat::from(Float::with_val_round(prec_max(), -1.0, Round::Up).0),
                OrdFloat::from(Float::with_val_round(prec_max(), 0.0, Round::Up).0)
            ),
            OrdFloat::from(Float::with_val_round(prec_max(), 0.25, Round::Up).0)
        )
    );
}

#[test]
fn test_rect_rect_contact() {
    let a = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 10.0, Round::Up).0),
    ));
    let b = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), -2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 12.0, Round::Up).0),
    ));
    assert_eq!(
        a.contact_point(&b),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.5, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 10.5, Round::Up).0)
        )
    );
    assert_eq!(
        b.contact_point(&a),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 0.5, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 10.5, Round::Up).0)
        )
    );
}

#[test]
fn test_circle_circle_contact() {
    let a = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 15.0, Round::Up).0),
    ));
    let b = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 8.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 19.0, Round::Up).0),
    ));
    assert_eq!(
        a.contact_point(&b),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 15.5, Round::Up).0)
        )
    );
    assert_eq!(
        b.contact_point(&a),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 15.5, Round::Up).0)
        )
    );
}

#[test]
fn test_circle_rect_contact() {
    let a = Shape::circle(OrdFloat::from(
        Float::with_val_round(prec_max(), 2.0, Round::Up).0,
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 5.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 15.0, Round::Up).0),
    ));
    let b = Shape::rect(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 8.0, Round::Up).0),
    ))
    .place(v2(
        OrdFloat::from(Float::with_val_round(prec_max(), 2.0, Round::Up).0),
        OrdFloat::from(Float::with_val_round(prec_max(), 18.0, Round::Up).0),
    ));
    assert_eq!(
        a.contact_point(&b),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 15.0, Round::Up).0)
        )
    );
    assert_eq!(
        b.contact_point(&a),
        v2(
            OrdFloat::from(Float::with_val_round(prec_max(), 4.0, Round::Up).0),
            OrdFloat::from(Float::with_val_round(prec_max(), 15.0, Round::Up).0)
        )
    );
}
