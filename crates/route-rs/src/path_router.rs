///
/// 
use crate::SegmentType;
use std::collections::HashMap;
use std::ops::Deref;

// TODO: Evaluate SmallVec/TinyVec for params and maybe node children

#[derive(Clone, Debug, PartialEq)]
pub enum InsertError {
    EmptyPath,
    InvalidPath(String),
    TrailingWildcardPath,
}

impl From<url::ParseError> for InsertError {
    fn from(src: url::ParseError) -> InsertError {
        InsertError::InvalidPath(src.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchError {
    NotFound,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentIdx(usize);

impl Deref for SegmentIdx {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteIdx(usize);

impl Deref for RouteIdx {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Route {
    pub segment_idx: SegmentIdx,
    pub parent: Option<RouteIdx>,
    pub children: Vec<RouteIdx>,
}

#[derive(Debug)]
pub struct PathRouter {
    pub segments: Vec<SegmentType>,
    pub routes: Vec<Route>,
}

impl PathRouter {
    pub fn new() -> Self {
        let root = Route {
            segment_idx: SegmentIdx(0),
            parent: None,
            children: Vec::new(),
        };
        PathRouter {
            segments: vec!["".into()],
            routes: vec![root],
        }
    }

    #[inline]
    fn get_segment(&mut self, segment: &SegmentType) -> Option<SegmentIdx> {
        for i in 0..self.segments.len() {
            if self.segments[i] == *segment {
                return Some(SegmentIdx(i));
            }
        }
        None
    }

    #[inline]
    fn get_or_insert_segment(&mut self, segment: SegmentType) -> SegmentIdx {
        self.get_segment(&segment).unwrap_or_else(|| {
            self.segments.push(segment);
            SegmentIdx(self.segments.len() - 1)
        })
    }

    #[inline]
    pub fn find_child_node_by_segment(
        &self,
        route_idx: RouteIdx,
        segment_idx: SegmentIdx,
    ) -> Option<RouteIdx> {
        let route = &self.routes[*route_idx];
        for i in 0..route.children.len() {
            let child = route.children[i];
            if self.routes[*child].segment_idx == segment_idx {
                return Some(child);
            }
        }
        return None;
    }

    #[inline]
    fn insert_new_node_as_child(
        &mut self,
        parent_route_idx: RouteIdx,
        segment_idx: SegmentIdx,
    ) -> RouteIdx {
        let new_route_id = RouteIdx(self.routes.len());
        let route = Route {
            segment_idx: segment_idx,
            parent: Some(parent_route_idx),
            children: vec![],
        };
        self.routes.push(route);
        self.routes[*parent_route_idx].children.push(new_route_id);
        new_route_id
    }

    pub fn insert(&mut self, path: impl Into<String>) -> Result<RouteIdx, InsertError> {
        let path = path.into();
        if path == "" {
            return Err(InsertError::EmptyPath);
        }
        // Starting from the root node
        let mut parent_route_idx = RouteIdx(0);
        let mut segment_idx = SegmentIdx(0);
        if path == "/" {
            return Ok(parent_route_idx);
        }
        let segments = path.split("/");
        // Add a child node for each segment, skipping the root
        for segment_str in segments.skip(1) {
            // Should not define path following *
            match self.segments[*segment_idx] {
                SegmentType::Consume(_) | SegmentType::Wildcard => {
                    return Err(InsertError::TrailingWildcardPath)
                }
                _ => {}
            }
            let segment: SegmentType = segment_str.into();
            segment_idx = self.get_or_insert_segment(segment);
            // Determine if the segment is already a child of the current node
            if let Some(child_route_idx) =
                self.find_child_node_by_segment(parent_route_idx, segment_idx)
            {
                parent_route_idx = child_route_idx.clone();
                continue;
            }
            // Insert a new child node and proceed
            parent_route_idx = self.insert_new_node_as_child(parent_route_idx, segment_idx);
        }
        Ok(parent_route_idx)
    }

    pub fn eval(&self, path: &str) -> Result<PathMatch, MatchError> {
        let chars = path.chars();
        for i in 0..chars.len() {
            if chars[i] == "/" {
                panic!("boundary at {:?}", i);
            }
        }
        // for segment in path.split("/") {}

        Err(MatchError::NotFound)
    }
}

#[derive(Clone, Debug)]
pub struct PathMatch {
    pub route: RouteIdx,
    pub params: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod should {
    use crate::path_router::*;

    #[test]
    fn return_empty_path_error_for_insert() {
        let mut router = PathRouter::new();
        assert_eq!(InsertError::EmptyPath, router.insert("").unwrap_err());
    }

    #[test]
    fn return_root_route_idx_for_slash() {
        let mut router = PathRouter::new();
        assert_eq!(0usize, *router.insert("/").unwrap());
    }

    #[test]
    fn return_second_node_for_first_nested_path() {
        let mut router = PathRouter::new();
        let route_idx = router.insert("/foo").unwrap();
        assert_eq!(1usize, *route_idx);
    }

    #[test]
    fn return_warning_for_trailing_path_after_consume() {
        let mut router = PathRouter::new();
        assert_eq!(
            InsertError::TrailingWildcardPath,
            router.insert("/*foo/bar").unwrap_err()
        );
    }

    #[test]
    fn return_warning_for_trailing_path_after_wildcard() {
        let mut router = PathRouter::new();
        assert_eq!(
            InsertError::TrailingWildcardPath,
            router.insert("/*/bar").unwrap_err()
        );
    }

    #[test]
    fn create_second_peer_node_for_next_root_child() {
        let mut router = PathRouter::new();
        let first = router.insert("/foo").unwrap();
        let second = router.insert("/bar").unwrap();
        assert_eq!(1usize, *first);
        assert_eq!(2usize, *second);
    }

    #[test]
    fn get_root_node_from_base_path() {
        let router = PathRouter::new();
        let path_match = router.eval("/").unwrap();
        assert_eq!(0usize, *path_match.route);
        assert!(path_match.params.is_none());
    }
}

// #[test]
//     fn contain_single_root_node_and_segment_for_new_router() {
//         let router = PathRouter::new();
//         assert_eq!(1, router.segments.len());
//         assert_eq!(1, router.routes.len());
//         assert_eq!(SegmentType::Static("".to_owned()), router.segments[0]);
//         assert_eq!(
//             Node {
//                 segment_idx: SegmentIdx(0),
//                 parent: None,
//                 children: vec![],
//             },
//             router.routes[0]
//         );
//     }

// #[test]
// fn not_change_after_inserting_duplicate_root() {
//     let mut router = PathRouter::new();
//     assert_eq!(RouteIdx(0), router.insert("/").unwrap());
//     assert_eq!(1, router.segments.len());
//     assert_eq!(1, router.routes.len());
//     assert_eq!(SegmentType::Static("".to_owned()), router.segments[0]);
//     assert_eq!(
//         Node {
//             segment_idx: SegmentIdx(0),
//             parent: None,
//             children: vec![],
//         },
//         router.routes[0]
//     );
// }

// #[test]
// fn configure_second_node_for_single_nested_path() {
//     let mut router = PathRouter::new();
//     assert_eq!(RouteIdx(1), router.insert("/foo").unwrap());
//     assert_eq!(2, router.segments.len());
//     assert_eq!(2, router.routes.len());
//     assert_eq!(SegmentType::Static("foo".to_owned()), router.segments[1]);
//     assert_eq!(
//         Node {
//             segment_idx: SegmentIdx(0),
//             parent: None,
//             children: vec![RouteIdx(1)],
//         },
//         router.routes[0]
//     );
//     assert_eq!(
//         Node {
//             segment_idx: SegmentIdx(1),
//             parent: Some(RouteIdx(0)),
//             children: vec![],
//         },
//         router.routes[1]
//     );
// }

// #[test]
// fn configure_third_node_for_single_nested_path() {
//     let mut router = PathRouter::new();
//     assert_eq!(RouteIdx(2), router.insert("/foo/bar").unwrap());
//     assert_eq!(3, router.segments.len());
//     assert_eq!(3, router.routes.len());
//     assert_eq!(SegmentType::Static("bar".to_owned()), router.segments[2]);
//     assert_eq!(
//         Node {
//             segment_idx: SegmentIdx(0),
//             parent: None,
//             children: vec![RouteIdx(1)],
//         },
//         router.routes[0]
//     );
//     assert_eq!(
//         Node {
//             segment_idx: SegmentIdx(1),
//             parent: Some(RouteIdx(0)),
//             children: vec![RouteIdx(2)],
//         },
//         router.routes[1]
//     );
//     assert_eq!(
//         Node {
//             segment_idx: SegmentIdx(2),
//             parent: Some(RouteIdx(1)),
//             children: vec![],
//         },
//         router.routes[2]
//     );
// }

// #[test]
// fn return_error_for_eval_base_path() {
//     let router = PathRouter::new();
//     match router.eval("/").unwrap_err() {
//         MatchError::NotFound => {} // _ => panic!("wrong error")
//     }
// }

// #[test]
// fn return_path_id_for_matching_base_path() {
//     let mut router = PathRouter::new();
//     // panic!("{:?}", router);
//     let root_idx = router.insert("/").unwrap();
//     panic!("{:?} -> {:?}", router, root_idx);
//     let (_matched_path, params) = router.eval("/").unwrap();
//     assert_eq!(root_idx.0, 0, "wrong path returned");
//     assert_eq!(0, params.len(), "too many params returned");
// }

// #[test]
// fn return_valid() {
//     let mut router = PathRouter::new();
//     // panic!("{:?}", router);
//     let root_idx = router.insert("/").unwrap();
//     let foo_idx = router.insert("/foo").unwrap();
//     let bar_idx = router.insert("/foo/:bar").unwrap();
//     panic!("{:?} -> {:?} / {:?} / {:?}", router, root_idx, foo_idx, bar_idx);
// }
