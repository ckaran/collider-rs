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
use num::BigRational;

#[test]
fn test_circle_advance() {
    let shape_1 = Shape::circle(BigRational::from_float(2.0).unwrap()).place(v2(
        BigRational::from_float(3.0).unwrap(),
        BigRational::from_float(5.0).unwrap(),
    ));
    assert_eq!(
        shape_1.advance(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(2.0).unwrap()
            ),
            v2(
                BigRational::from_float(-0.25),
                BigRational::from_float(-0.25)
            ),
            BigRational::from_float(2.0).unwrap()
        ),
        Shape::circle(BigRational::from_float(1.5).unwrap()).place(v2(
            BigRational::from_float(5.0).unwrap(),
            BigRational::from_float(9.0).unwrap()
        ))
    );
}

#[test]
fn test_rect_advance() {
    let shape_1 = Shape::rect(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(5.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(3.0).unwrap(),
        BigRational::from_float(5.0).unwrap(),
    ));
    assert_eq!(
        shape_1.advance(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(2.0).unwrap()
            ),
            v2(
                BigRational::from_float(-0.25),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(2.0).unwrap()
        ),
        Shape::rect(v2(
            BigRational::from_float(1.5).unwrap(),
            BigRational::from_float(7.0).unwrap()
        ))
        .place(v2(
            BigRational::from_float(5.0).unwrap(),
            BigRational::from_float(9.0).unwrap()
        ))
    );
}

#[test]
#[should_panic]
fn test_illegal_circle_advance() {
    let shape = Shape::circle(BigRational::from_float(2.0).unwrap()).place(v2(
        BigRational::from_float(3.0).unwrap(),
        BigRational::from_float(5.0).unwrap(),
    ));
    shape.advance(
        v2(
            BigRational::from_float(1.0).unwrap(),
            BigRational::from_float(2.0).unwrap(),
        ),
        v2(
            BigRational::from_float(-0.25),
            BigRational::from_float(-0.24),
        ),
        BigRational::from_float(2.0).unwrap(),
    );
}

#[test]
fn test_edges() {
    let shape = Shape::rect(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(6.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(3.0).unwrap(),
        BigRational::from_float(5.0).unwrap(),
    ));
    assert_eq!(shape.min_x(), BigRational::from_float(1.0).unwrap());
    assert_eq!(shape.min_y(), BigRational::from_float(2.0).unwrap());
    assert_eq!(shape.max_x(), BigRational::from_float(5.0).unwrap());
    assert_eq!(shape.max_y(), BigRational::from_float(8.0).unwrap());
}

#[test]
fn test_rect_rect_normal() {
    let src = Shape::rect(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(4.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(1.0).unwrap(),
        BigRational::from_float(1.0).unwrap(),
    ));
    let dst = Shape::rect(v2(
        BigRational::from_float(8.0).unwrap(),
        BigRational::from_float(8.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(1.5).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(0.0).unwrap()
            ),
            BigRational::from_float(5.0).unwrap()
        )
    );
    let dst = Shape::rect(v2(
        BigRational::from_float(8.0).unwrap(),
        BigRational::from_float(8.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(0.0).unwrap(),
        BigRational::from_float(0.5).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(-1.0),
                BigRational::from_float(0.0).unwrap()
            ),
            BigRational::from_float(5.0).unwrap()
        )
    );
    let dst = Shape::rect(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(3.8).unwrap(),
        BigRational::from_float(4.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(0.0).unwrap()
        )
    );
    let dst = Shape::rect(v2(
        BigRational::from_float(8.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(-2.0),
        BigRational::from_float(-3.0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(-1.0)
            ),
            BigRational::from_float(-1.0)
        )
    );
}

#[test]
fn test_circle_circle_normal() {
    let src = Shape::circle(BigRational::from_float(2.0).unwrap()).place(v2(
        BigRational::from_float(1.0).unwrap(),
        BigRational::from_float(1.0).unwrap(),
    ));
    let dst = Shape::circle(BigRational::from_float(3.0).unwrap()).place(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(0.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(-1.0)
            ),
            BigRational::from_float(2.5).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
}

#[test]
fn test_rect_circle_normal() {
    let src = Shape::rect(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(0.0).unwrap(),
        BigRational::from_float(0.0).unwrap(),
    ));

    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(-2.0),
        BigRational::from_float(0.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(-1.0),
                BigRational::from_float(0.0).unwrap()
            ),
            BigRational::from_float(0.25).unwrap()
        )
    );
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(0.0).unwrap(),
        BigRational::from_float(-2.0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(-1.0)
            ),
            BigRational::from_float(0.25).unwrap()
        )
    );
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(0.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(0.0).unwrap()
            ),
            BigRational::from_float(0.25).unwrap()
        )
    );
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(0.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(0.25).unwrap()
        )
    );

    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(-2.0),
        BigRational::from_float(-2.0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(BigRational::from_float(-1.0), BigRational::from_float(-1.0)),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(-2.0),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(-1.0)
            ),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(-2.0),
        BigRational::from_float(2.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(-1.0),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ));
    assert_eq!(
        dst.normal_from(&src),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
}

#[test]
fn test_masked_rect_rect_normal() {
    let src = Shape::rect(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(4.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(1.0).unwrap(),
        BigRational::from_float(1.0).unwrap(),
    ));
    let dst = Shape::rect(v2(
        BigRational::from_float(8.0).unwrap(),
        BigRational::from_float(8.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(6.0).unwrap(),
        BigRational::from_float(-5.0),
    ));
    let mut mask = CardMask::full();
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(-1.0)
            ),
            BigRational::from_float(0.0).unwrap()
        )
    );
    mask[Card::MinusY] = false;
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(0.0).unwrap()
            ),
            BigRational::from_float(1.0).unwrap()
        )
    );
}

#[test]
fn test_masked_rect_circle_normal() {
    let src = Shape::rect(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(0.0).unwrap(),
        BigRational::from_float(0.0).unwrap(),
    ));
    let dst = Shape::circle(BigRational::from_float(2.5).unwrap()).place(v2(
        BigRational::from_float(-2.0),
        BigRational::from_float(2.0).unwrap(),
    ));
    let mut mask = CardMask::full();
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                BigRational::from_float(-1.0),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
    mask[Card::PlusX] = false;
    assert_eq!(
        src.masked_normal_from(&dst, mask.flip()),
        DirVec2::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(-1.0)
            ),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                BigRational::from_float(-1.0),
                BigRational::from_float(1.0).unwrap()
            ),
            BigRational::from_float(1.25).unwrap() - BigRational::from_float(2.0).unwrap().sqrt()
        )
    );
    mask[Card::PlusY] = false;
    assert_eq!(
        dst.masked_normal_from(&src, mask),
        DirVec2::new(
            v2(
                BigRational::from_float(-1.0),
                BigRational::from_float(0.0).unwrap()
            ),
            BigRational::from_float(0.25).unwrap()
        )
    );
}

#[test]
fn test_rect_rect_contact() {
    let a = Shape::rect(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(2.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(10.0).unwrap(),
    ));
    let b = Shape::rect(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(4.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(-2.0),
        BigRational::from_float(12.0).unwrap(),
    ));
    assert_eq!(
        a.contact_point(&b),
        v2(
            BigRational::from_float(0.5).unwrap(),
            BigRational::from_float(10.5).unwrap()
        )
    );
    assert_eq!(
        b.contact_point(&a),
        v2(
            BigRational::from_float(0.5).unwrap(),
            BigRational::from_float(10.5).unwrap()
        )
    );
}

#[test]
fn test_circle_circle_contact() {
    let a = Shape::circle(BigRational::from_float(2.0).unwrap()).place(v2(
        BigRational::from_float(5.0).unwrap(),
        BigRational::from_float(15.0).unwrap(),
    ));
    let b = Shape::circle(BigRational::from_float(8.0).unwrap()).place(v2(
        BigRational::from_float(5.0).unwrap(),
        BigRational::from_float(19.0).unwrap(),
    ));
    assert_eq!(
        a.contact_point(&b),
        v2(
            BigRational::from_float(5.0).unwrap(),
            BigRational::from_float(15.5).unwrap()
        )
    );
    assert_eq!(
        b.contact_point(&a),
        v2(
            BigRational::from_float(5.0).unwrap(),
            BigRational::from_float(15.5).unwrap()
        )
    );
}

#[test]
fn test_circle_rect_contact() {
    let a = Shape::circle(BigRational::from_float(2.0).unwrap()).place(v2(
        BigRational::from_float(5.0).unwrap(),
        BigRational::from_float(15.0).unwrap(),
    ));
    let b = Shape::rect(v2(
        BigRational::from_float(4.0).unwrap(),
        BigRational::from_float(8.0).unwrap(),
    ))
    .place(v2(
        BigRational::from_float(2.0).unwrap(),
        BigRational::from_float(18.0).unwrap(),
    ));
    assert_eq!(
        a.contact_point(&b),
        v2(
            BigRational::from_float(4.0).unwrap(),
            BigRational::from_float(15.0).unwrap()
        )
    );
    assert_eq!(
        b.contact_point(&a),
        v2(
            BigRational::from_float(4.0).unwrap(),
            BigRational::from_float(15.0).unwrap()
        )
    );
}
