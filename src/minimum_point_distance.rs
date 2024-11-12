use std::f64;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

pub(crate) fn minimum_point_distance(){
    let points = vec![
        Point { x: 3, y: 7 },
        Point { x: 30, y: 80 },
        Point { x: 80, y: 320 },
        Point { x: 15, y: 276 },
        Point { x: 84, y: 298 },
        Point { x: 19, y: 29 },
        Point { x: 200, y: 200 },
        Point { x: 191, y: 312 },
    ];

    let mut min_distance = f64::INFINITY;
    let mut min_points = (0, 0);

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance(&points[j]);
            if distance < min_distance {
                min_distance = distance;
                min_points = (i, j);
            }
        }
    }

    draw_grid(min_distance, min_points, points)
}

fn draw_grid(min_distance: f64, min_points: (usize, usize), points: Vec<Point>){
    // Define grid dimensions
    let width = 50;
    let height = 25;

    // Define scaling factors
    let max_x = points.iter().map(|p| p.x).max().unwrap() as f64;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as f64;
    let x_scale = max_x / (width as f64);
    let y_scale = max_y / (height as f64);

    // Initialize an empty grid
    let mut grid = vec![vec![' '; width]; height];

    // Place points on the grid
    for (i, point) in points.iter().enumerate() {
        let x = ((point.x as f64 / x_scale).round() as usize).min(width - 1);
        let y = ((point.y as f64 / y_scale).round() as usize).min(height - 1);
        let y = height - 1 - y; // Flip y for correct orientation

        // Mark the closest pair with 'X', others with '*'
        if i == min_points.0 || i == min_points.1 {
            grid[y][x] = 'X';
        } else {
            grid[y][x] = '*';
        }
    }

    // Print the grid with axis markings
    println!(" y ↑");
    for (i, row) in grid.iter().enumerate() {
        let y_label = ((max_y / height as f64) * (height - 1 - i) as f64).round() as i32;
        print!("{:3} |", y_label); // y-axis label on the left
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }

    // Print x-axis labels at the bottom
    print!("    +"); // Align with y-axis label
    for _ in 0..width {
        print!("-");
    }
    println!();

    print!("     "); // Align x-axis labels
    for i in 0..width {
        if i % (width / 5) == 0 { // Show x-axis labels at intervals
            let x_label = ((max_x / width as f64) * i as f64).round() as i32;
            print!("{:3}", x_label);
        } else {
            print!(" ");
        }
    }
    println!("\n x →");

    println!(
        "\nThe two closest points are P{} and P{} with a distance of {:.2}",
        min_points.0, min_points.1, min_distance
    );
}