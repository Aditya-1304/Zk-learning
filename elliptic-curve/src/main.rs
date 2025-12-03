use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define our Finite Field parameters
    // Curve: y^2 = x^3 + 3 (mod p)
    let p: i32 = 120; 
    let output_file = "ecc_finite_field.png";

    // 2. Generate the Points
    // In Python, they solved for Y. In Rust, for small P, 
    // we can just check every coordinate (x, y) to see if it satisfies the equation.
    let mut points: Vec<(i32, i32)> = Vec::new();

    for x in 0..p {
        // Calculate RHS: (x^3 + 3) % p
        let rhs = (x.pow(3) + 3) % p;

        // Check every possible Y to see if y^2 % p == rhs
        // This is O(p^2) which is fine for small plotting, bad for crypto.
        for y in 0..p {
            let lhs = (y.pow(2)) % p;
            if lhs == rhs {
                points.push((x, y));
                println!("Found point: ({}, {})", x, y);
            }
        }
    }

    // 3. Setup the Plotting Backend (The "Verbose" part)
    let root = BitMapBackend::new(output_file, (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("y^2 = x^3 + 3 (mod {})", p), ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..p, 0..p)?;

    chart.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    // 4. Draw the Scatter Plot
    chart.draw_series(
        points.iter().map(|(x, y)| {
            EmptyElement::at((*x, *y))    // Position the element
            + Circle::new((0, 0), 5, ShapeStyle::from(&RED).filled()) // Draw a red dot
            + Text::new(
                format!("({},{})", x, y), // Add coordinates text
                (10, 0), 
                ("sans-serif", 15.0).into_font()
            ) 
        })
    )?;

    println!("Plot saved to {}", output_file);
    Ok(())
}