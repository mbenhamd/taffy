//! The min-content main size of a multi-line row flex container is the largest min-content
//! contribution among its items (CSS Flexbox 1 §9.9.1). An item's flex basis does not raise that
//! contribution, also when the item is a scroll container, and its margins add on top of its
//! min/max-clamped size. Expected values are Chrome's.
//! <https://www.w3.org/TR/css-flexbox-1/#intrinsic-main-sizes>
#[cfg(feature = "flexbox")]
mod row_wrap_min_content_width {
    use taffy::prelude::*;
    use taffy::{Overflow, Point};

    /// A wrapping row holding `item` (with a `content_width`-wide child), optionally followed by a
    /// 100px item, sized under a min-content constraint.
    fn min_content_width(item: Style, content_width: f32, with_second_item: bool) -> f32 {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let content = tree
            .new_leaf(Style { size: Size { width: length(content_width), height: length(5.0) }, ..Default::default() })
            .unwrap();
        let mut children = vec![tree.new_with_children(item, &[content]).unwrap()];
        if with_second_item {
            children.push(
                tree.new_leaf(Style { size: Size { width: length(100.0), height: length(5.0) }, ..Default::default() })
                    .unwrap(),
            );
        }
        let flex = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_wrap: FlexWrap::Wrap,
                    gap: Size { width: length(12.0), height: zero() },
                    ..Default::default()
                },
                &children,
            )
            .unwrap();
        tree.compute_layout(flex, Size { width: AvailableSpace::MinContent, height: AvailableSpace::MaxContent })
            .unwrap();
        tree.layout(flex).unwrap().size.width
    }

    fn basis_item(flex_shrink: f32) -> Style {
        Style { flex_grow: 1.0, flex_shrink, flex_basis: length(320.0), ..Default::default() }
    }

    #[test]
    fn a_shrinkable_flex_basis_does_not_widen_the_min_content_size() {
        assert_eq!(min_content_width(basis_item(1.0), 20.0, true), 100.0);
        assert_eq!(min_content_width(basis_item(1.0), 20.0, false), 20.0);
    }

    #[test]
    fn an_unshrinkable_flex_basis_does_not_widen_the_min_content_size() {
        assert_eq!(min_content_width(basis_item(0.0), 20.0, true), 100.0);
    }

    #[test]
    fn a_scroll_container_flex_basis_does_not_widen_the_min_content_size() {
        let hidden = Point { x: Overflow::Hidden, y: Overflow::Hidden };
        assert_eq!(min_content_width(Style { overflow: hidden, ..basis_item(1.0) }, 20.0, true), 100.0);
        assert_eq!(min_content_width(Style { overflow: hidden, ..basis_item(1.0) }, 20.0, false), 20.0);
    }

    #[test]
    fn item_margins_add_to_the_clamped_item_size() {
        let margin = Rect { left: length(10.0), right: length(10.0), top: zero(), bottom: zero() };
        let min_width = Style { min_size: Size { width: length(200.0), height: auto() }, margin, ..Default::default() };
        let max_width = Style { max_size: Size { width: length(100.0), height: auto() }, margin, ..Default::default() };
        assert_eq!(min_content_width(min_width, 20.0, false), 220.0);
        assert_eq!(min_content_width(max_width, 300.0, false), 120.0);
        assert_eq!(min_content_width(Style { margin, ..Default::default() }, 20.0, false), 40.0);
    }
}
