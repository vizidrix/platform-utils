//! 
//! Derived from https://github.com/gotham-rs/gotham/tree/main/gotham/src/state

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault};

mod type_id_hasher;
use type_id_hasher::{TypeIdHasher};

/// Simplified type signature over underlying HashMap
pub type TypeIdMap<T> = HashMap<TypeId, Box<T>, BuildHasherDefault<TypeIdHasher>>;

/// Provides storage for request state, and stores one item of each type. The types used for
/// storage must implement the [`StateData`] trait to allow its storage, which is usually done
/// by adding `#[derive(StateData)]` on the type in question.
///
/// # Examples
///
/// ```rust
/// use typemap::TypeMap;
///
/// struct MyStruct {
///     value: i32,
/// }
/// # fn main() {
/// #   TypeMap::with_new(|map| {
/// #
/// map.put(MyStruct { value: 1 });
/// assert_eq!(map.borrow::<MyStruct>().value, 1);
/// #
/// #   });
/// # }
/// ```
#[derive(Debug, Default)]
pub struct TypeMap {
    // inner: HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<TypeIdHasher>>,
    inner: TypeIdMap<dyn Any>,
}

impl TypeMap {
    /// Creates a new, empty `State` container. This is for internal Gotham use, because the
    /// ability to create a new `State` container would allow for libraries and applications to
    /// incorrectly discard important internal data.
    // pub fn new() -> TypeMap {
    //     Self {
    //         inner: HashMap::default(),
    //         // inner: HashMap::with_capacity(capacity)
    //     }
    // }

    /// Creates a new, empty `State` and yields it mutably into the provided closure. This is
    /// intended only for use in the documentation tests for `State`, since the `State` container
    /// cannot be constructed otherwise.
    #[doc(hidden)]
    pub fn with_new<F>(f: F)
    where
        F: FnOnce(&mut TypeMap),
    {
        f(&mut TypeMap::default())
    }

    /// Puts a value into the `State` storage. One value of each type is retained. Successive calls
    /// to `put` will overwrite the existing value of the same type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # struct AnotherStruct {
    /// #     value: &'static str
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 1 });
    /// assert_eq!(map.borrow::<MyStruct>().value, 1);
    ///
    /// map.put(AnotherStruct { value: "a string" });
    /// map.put(MyStruct { value: 100 });
    ///
    /// assert_eq!(map.borrow::<AnotherStruct>().value, "a string");
    /// assert_eq!(map.borrow::<MyStruct>().value, 100);
    /// #
    /// #   });
    /// # }
    /// ```
    pub fn put<T: Any>(&mut self, t: T) {
        let type_id = TypeId::of::<T>();
        self.inner.insert(type_id, Box::new(t));
    }

    /// Determines if the current entry exists in `TypeMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # struct AnotherStruct {
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 1 });
    /// assert!(map.has::<MyStruct>());
    /// assert_eq!(map.borrow::<MyStruct>().value, 1);
    ///
    /// assert!(!map.has::<AnotherStruct>());
    /// #
    /// #   });
    /// # }
    /// ```
    pub fn has<T: Any>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.inner.get(&type_id).is_some()
    }

    /// Tries to borrow a value from the `TypeMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # struct AnotherStruct {
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 1 });
    /// assert!(map.try_borrow::<MyStruct>().is_some());
    /// assert_eq!(map.try_borrow::<MyStruct>().unwrap().value, 1);
    ///
    /// assert!(map.try_borrow::<AnotherStruct>().is_none());
    /// #
    /// #   });
    /// # }
    /// ```
    pub fn try_get<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.inner.get(&type_id).and_then(|b| b.downcast_ref::<T>())
    }

    /// Borrows a value from the `TypeMap`.
    ///
    /// # Panics
    ///
    /// If a value of type `T` is not present in `TypeMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 1 });
    /// assert_eq!(map.borrow::<MyStruct>().value, 1);
    /// #
    /// #   });
    /// # }
    /// ```
    pub fn get<T: Any>(&self) -> &T {
        self.try_get()
            .expect("required type is not present in TypeMap container")
    }

    /// Tries to mutably borrow a value from the `TypeMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # struct AnotherStruct {
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 100 });
    ///
    /// if let Some(a) = map.try_get_mut::<MyStruct>() {
    ///     a.value += 10;
    /// }
    ///
    /// assert_eq!(map.get::<MyStruct>().value, 110);
    ///
    /// assert!(map.try_get_mut::<AnotherStruct>().is_none());
    /// #   });
    /// # }
    pub fn try_get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.inner
            .get_mut(&type_id)
            .and_then(|b| b.downcast_mut::<T>())
    }

    /// Mutably borrows a value from the `TypeMap`.
    ///
    /// # Panics
    ///
    /// If a value of type `T` is not present in `TypeMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # struct AnotherStruct {
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 100 });
    ///
    /// {
    ///     let a = map.get_mut::<MyStruct>();
    ///     a.value += 10;
    /// }
    ///
    /// assert_eq!(map.get::<MyStruct>().value, 110);
    ///
    /// assert!(map.try_get_mut::<AnotherStruct>().is_none());
    /// #
    /// #   });
    /// # }
    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.try_get_mut()
            .expect("required type is not present in State container")
    }

    /// Tries to move a value out of the `TypeMap` storage and return ownership.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # struct AnotherStruct {
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 110 });
    ///
    /// assert_eq!(map.try_take::<MyStruct>().unwrap().value, 110);
    ///
    /// assert!(map.try_take::<MyStruct>().is_none());
    /// assert!(map.try_get_mut::<MyStruct>().is_none());
    /// assert!(map.try_get::<MyStruct>().is_none());
    ///
    /// assert!(map.try_take::<AnotherStruct>().is_none());
    /// #
    /// #   });
    /// # }
    pub fn try_take<T: Any>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.inner
            .remove(&type_id)
            .and_then(|b| b.downcast::<T>().ok())
            .map(|b| *b)
    }

    /// Moves a value out of the `TypeMap` storage and returns ownership.
    ///
    /// # Panics
    ///
    /// If a value of type `T` is not present in `TypeMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typemap::TypeMap;
    /// #
    /// # struct MyStruct {
    /// #     value: i32
    /// # }
    /// #
    /// # fn main() {
    /// #   TypeMap::with_new(|map| {
    /// #
    /// map.put(MyStruct { value: 110 });
    ///
    /// assert_eq!(map.take::<MyStruct>().value, 110);
    ///
    /// assert!(map.try_take::<MyStruct>().is_none());
    /// assert!(map.try_borrow_mut::<MyStruct>().is_none());
    /// assert!(map.try_borrow::<MyStruct>().is_none());
    /// #
    /// #   });
    /// # }
    pub fn take<T: Any>(&mut self) -> T {
        self.try_take()
            .expect("required type is not present in State container")
    }
}