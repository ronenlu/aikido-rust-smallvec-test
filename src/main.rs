use smallvec::{smallvec, SmallVec};

fn main() {
    let v: SmallVec<[u8; 4]> = smallvec![1, 2, 3];
    println!("{:?}", v);
}
