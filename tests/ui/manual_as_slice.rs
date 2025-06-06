#![feature(new_range_api)]
#![allow(clippy::redundant_slicing)]
#![warn(clippy::manual_as_slice)]

mod nested_1 {
    pub(crate) mod nested_2 {
        pub(crate) const SOME_VALUE: [u8; 4] = [1, 2, 3, 4];
    }
}

fn main() {
    let array: [u8; 4] = [0; 4];
    let slice: &[u8] = &array[..];
    //~^ manual_as_slice

    let mut array: [u8; 4] = [0; 4];
    let mut slice: &[u8] = &mut array[..];
    //~^ manual_as_slice

    let slice: &[u8] = &nested_1::nested_2::SOME_VALUE[..];
    //~^ manual_as_slice

    let slice = b"foo";
    let slice: &[u8] = &slice[..];
    //~^ manual_as_slice

    let slice = &[1, 2, 3, 4];
    let slice: &[u8] = &slice[..];
    //~^ manual_as_slice

    macro_rules! perform_the_slice {
        ($a:expr) => {
            &$a[..]
            //~^ manual_as_slice
        };
    }

    perform_the_slice!([1, 2, 3]);

    let slice: &[u8] = &vec![1, 2, 3][..];
    //~^ manual_as_slice

    let slice = &"foo"[..];

    struct Count;

    impl<R: std::range::RangeBounds<()>> std::ops::Index<R> for Count {
        type Output = ();

        fn index(&self, _: R) -> &Self::Output {
            &()
        }
    }

    let count = &Count[..];
}
