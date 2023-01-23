///
/// 

use std::ops::Deref;

use crate::{InsertError, MatchError, SegmentLexer, SegmentType};

// TODO: Evaluate SmallVec/TinyVec for params and maybe node children

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
pub struct PathRouter<'a> {
    pub segments: Vec<SegmentType<'a>>,
    pub routes: Vec<Route>,
}

impl<'a> PathRouter<'a> {
    pub fn new() -> Self {
        PathRouter {
            segments: vec![],
            routes: vec![],
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
    fn get_or_insert_segment(&mut self, segment: SegmentType<'a>) -> SegmentIdx {
        self.get_segment(&segment).unwrap_or_else(|| {
            self.segments.push(segment);
            SegmentIdx(self.segments.len() - 1)
        })
    }

    #[inline]
    fn insert_root_node(&mut self) {
        let segment = SegmentType::Static { path: "" };
        let segment_idx = self.get_or_insert_segment(segment);
        let root = Route {
            segment_idx,
            parent: None,
            children: Vec::new(),
        };
        self.routes.push(root);
    }

    #[inline]
    fn find_child_node_by_segment(
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

    pub fn insert(&mut self, path: &'a str) -> Result<RouteIdx, InsertError> {
        if path.len() == 0 {
            return Err(InsertError::EmptyPath);
        }
        // Need to register the root node if it's not already
        if self.routes.len() == 0 {
            self.insert_root_node();
        }
        let mut parent_route_idx = RouteIdx(0);
        let mut segment_idx = SegmentIdx(0);
        let lexer = SegmentLexer::new(path);
        for item in lexer {
            // Should not define path following *
            match self.segments[*segment_idx] {
                SegmentType::Consume { .. } | SegmentType::Wildcard => {
                    return Err(InsertError::TrailingWildcardPath)
                }
                _ => {}
            }
            let (segment, _span) = item?;
            segment_idx = self.get_or_insert_segment(segment);
            // Setup this segment as the child of the current node
            if let Some(child_route_idx) =
                self.find_child_node_by_segment(parent_route_idx, segment_idx)
            {
                parent_route_idx = child_route_idx.clone();
                continue;
            }
            parent_route_idx = self.insert_new_node_as_child(parent_route_idx, segment_idx);
        }
        Ok(parent_route_idx)
    }

    pub fn eval(&self, path: &str) -> Result<PathMatch, MatchError> {
        let mut index = RouteIdx(0);
        // let _lexer = SegmentLexer::new(path);
        // for item in lexer {
        //     if item.
        // }

        // let _chars = path.chars();
        // for i in 0..chars.len() {
        //     if chars[i] == "/" {
        //         panic!("boundary at {:?}", i);
        //     }
        // }
        // for segment in path.split("/") {}

        Err(MatchError::NotFound)
    }
}

#[derive(Clone, Debug)]
pub struct PathMatch {
    pub route: RouteIdx,
    // pub params: Option<HashMap<String, String>>,
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
        let route_idx = router.insert("/").unwrap();
        assert_eq!(0usize, *route_idx);
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
    fn create_third_peer_node_for_nested_child_of_existing_despite_used_key() {
        let mut router = PathRouter::new();
        router.insert("/foo").unwrap();
        router.insert("/bar").unwrap();
        let third = router.insert("/foo/bar").unwrap();
        assert_eq!(3usize, *third);
    }
}

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