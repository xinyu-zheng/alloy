use super::*;

#[test]
fn test_dispatchable() {
    struct S1 {
        x: u64,
    }
    struct S2 {
        y: u64,
    }
    trait T: Send {
        fn f(&self) -> u64;
    }
    impl T for S1 {
        fn f(&self) -> u64 {
            self.x
        }
    }
    impl T for S2 {
        fn f(&self) -> u64 {
            self.y
        }
    }

    let s1 = S1 { x: 1 };
    let s2 = S2 { y: 2 };
    let s1gc: Gc<S1> = Gc::new(s1);
    let s2gc: Gc<S2> = Gc::new(s2);
    assert_eq!(s1gc.f(), 1);
    assert_eq!(s2gc.f(), 2);

    let s1gcd: Gc<dyn T> = s1gc;
    let s2gcd: Gc<dyn T> = s2gc;
    assert_eq!(s1gcd.f(), 1);
    assert_eq!(s2gcd.f(), 2);
}

#[test]
fn test_unsized() {
    let foo: Gc<[i32]> = Gc::new([1, 2, 3]);
    assert_eq!(foo, foo.clone());
}

#[test]
fn test_from_box() {
    let b: Box<u32> = Box::new(123);
    let g: Gc<u32> = Gc::from(b);

    assert_eq!(*g, 123);
}

#[test]
fn test_from_box_trait() {
    use crate::fmt::Display;
    use crate::string::ToString;

    let b: Box<dyn Display> = Box::new(123);
    let g: Gc<dyn Display> = Gc::from(b);

    assert_eq!(g.to_string(), "123");
}

#[test]
fn test_from_box_trait_zero_sized() {
    use crate::fmt::Debug;

    let b: Box<dyn Debug> = Box::new(());
    let g: Gc<dyn Debug> = Gc::from(b);

    assert_eq!(format!("{g:?}"), "()");
}

#[test]
fn test_from_box_slice() {
    let s = vec![1, 2, 3].into_boxed_slice();
    let g: Gc<[u32]> = Gc::from(s);

    assert_eq!(&g[..], [1, 2, 3]);
}

#[test]
fn test_from_box_str() {
    use crate::string::String;

    let s = String::from("foo").into_boxed_str();
    let g: Gc<str> = Gc::from(s);

    assert_eq!(&g[..], "foo");
}

#[test]
fn test_from_vec() {
    let v = vec![1, 2, 3];
    let g: Gc<[u32]> = Gc::from(v);

    assert_eq!(&g[..], [1, 2, 3]);
}
