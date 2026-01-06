mod outer;
mod rect;
mod scale;
mod border;

pub use outer::OuterArea;
pub use rect::Rect;
pub use scale::Scale;
pub use border::Border;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct Params {
    pub outer: OuterArea,
    pub inner: Rect,
    pub horizontal: Option<Scale>,
    pub vertical: Option<Scale>,
    pub method: Border,
}

impl Params {
    pub const fn new_checked(
        outer: OuterArea,
        inner: Rect,
        horizontal: Option<Scale>,
        vertical: Option<Scale>,
        method: Border,
    ) -> Option<Self> {
        let size = outer.size();
        // Is `inner` contained by `outer`?
        if inner.position.x > 0 && inner.position.y > 0 && inner.position.x + inner.size.w < size.w && inner.position.y + inner.size.h < size.h && match &outer {
            OuterArea::Rect(rect) => {
                inner.position.x >= rect.position.x
                    && inner.position.y >= rect.position.y
            }
            OuterArea::Size(_) => {
                true
            }
        } {
            if (horizontal.is_some() || vertical.is_some())  {
                Some(Self {
                    outer,
                    inner,
                    horizontal,
                    vertical,
                    method,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blittle::*;

    #[test]
    fn test_params() {
        let outer = OuterArea::Size(Size { w: 1024, h: 1024 });
        let inner = Rect {
            position: PositionU { x: 4, y: 4 },
            size: Size { w: 1016, h: 1016 },
        };
        assert!(
            Params::new_checked(
                outer,
                inner,
                Some(Scale::To(300)),
                None,
                Border::default()
            )
            .is_some()
        );
        assert!(
            Params::new_checked(
                outer,
                inner,
                Some(Scale::To(300)),
                Some(Scale::By(0.5)),
                Border::default()
            )
            .is_some()
        );
        assert!(Params::new_checked(outer, inner, None, None, Border::default()).is_none());

        let outer_rect = OuterArea::Rect(Rect {
            position: PositionU { x: 2, y: 2 },
            size: *outer.size(),
        });
        assert!(
            Params::new_checked(
                outer_rect,
                inner,
                Some(Scale::To(300)),
                None,
                Border::default()
            )
            .is_some()
        );

        let inner_bad = Rect {
            position: PositionU { x: 4, y: 4 },
            size: Size { w: 3000, h: 3000 },
        };
        assert!(
            Params::new_checked(
                outer,
                inner_bad,
                Some(Scale::To(300)),
                None,
                Border::default()
            )
            .is_none()
        );

        let outer_bad_size = OuterArea::Size(Size { w: 4, h: 4 });
        assert!(
            Params::new_checked(
                outer_bad_size,
                inner,
                Some(Scale::To(300)),
                None,
                Border::default()
            )
            .is_none()
        );

        assert!(
            Params::new_checked(
                outer_bad_size,
                inner_bad,
                Some(Scale::To(300)),
                None,
                Border::default()
            )
            .is_none()
        );

        let outer_bad_rect = OuterArea::Rect(Rect {
            position: PositionU { x: 1000, y: 1000 },
            size: *outer.size(),
        });
        assert!(
            Params::new_checked(
                outer_bad_rect,
                inner,
                Some(Scale::To(300)),
                None,
                Border::default()
            )
            .is_none()
        );
    }
}
