fn main() {
    // We want to calculate Y = a*X + Y
    // Let a = 2.0
    // Let X = [1.0, 2.0, 3.0]
    // Let Y = [4.0, 5.0, 6.0]

    let		mut x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let mut     y: Vec<f64> = vec![4.0, 5.0, 6.0];
    let a: f64 = 2.0;
    let n: i32 = x.len() as i32;
    let incx: i32 = 1; // Stride for x (access every 1st element)
    let incy: i32 = 1; // Stride for y (access every 1st element)

    println!("Before BLAS operation:");
    println!("  Y = {:?}", y);

    // This is the C function call. It's unsafe because we are:
    // 1. Calling a C function (FFI).
    // 2. Passing raw pointers.
    // Rust cannot guarantee memory safety inside this function.
    unsafe {
        cblas::daxpy(
	    n,            // The number of elements
	    a,            // The scalar 'a'
	    x.as_ptr(),   // Pointer to the start of X
            incx,         // Stride of X
	    y.as_mut_ptr(), // Mutable pointer to the start of Y
            incy,         // Stride of Y
	);
    }

    println!("\nAfter BLAS operation (Y = 2.0*X + Y):");
    println!("  Y = {:?}", y);

    // Expected result:
    // Y[0] = 2.0 * 1.0 + 4.0 = 6.0
    // Y[1] = 2.0 * 2.0 + 5.0 = 9.0
    // Y[2] = 2.0 * 3.0 + 6.0 = 12.0
    assert_eq!(y, vec![6.0, 9.0, 12.0]);
    println!("\nAssertion passed! BLAS is linked and working.");
}
