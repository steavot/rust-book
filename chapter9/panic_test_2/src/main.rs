fn main() {
    let a: [usize; 5] = [1,2,3,4,5];
    a[99];
}

// this fails to build
// so rust will stop you indexing an array out of range
// at compile time but not a vector. Makes sense.

