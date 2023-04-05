#[allow(unused_variables)]

use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the binary of the memory
        let x_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(&self.x as *const f64 as *const u8, std::mem::size_of::<f64>())
        };
        let y_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(&self.y as *const f64 as *const u8, std::mem::size_of::<f64>())
        };
        
        let mut binary_string = String::new( );
        for byte in x_bytes
        {
            binary_string.push_str( &format!( "{:b}", byte ).to_string( ) );
        }
        for byte in y_bytes
        {
            binary_string.push_str( &format!( "{:b}", byte ).to_string( ) );
        }
        
        write!(f, "{}", binary_string )
    }
}

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}i",
               self.real,
               if self.imaginary < 0.0 { "-" } else { "+" },
               self.imaginary.abs() )
    }
}

#[allow(dead_code)]
fn display() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    println!("What does Point2D look like in binary: {:b}?", point);
    
    let complex = Complex { real: 3.0, imaginary: 1.0 };
    println!("Complex: {}", complex );
}