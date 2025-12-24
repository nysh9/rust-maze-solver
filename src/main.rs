use std::collections::HashSet;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl Maze {
    /// Generate a new maze using randomized DFS (recursive backtracking)
    pub fn generate(width: usize, height: usize) -> Self {
        // Ensure dimensions are odd for proper wall/path structure
        let actual_width = if width % 2 == 0 { width + 1 } else { width };
        let actual_height = if height % 2 == 0 { height + 1 } else { height };
        
        // Initialize grid with all walls
        let mut grid = vec![vec!['#'; actual_width]; actual_height];
        
        // Set starting position (must be odd coordinates for path cells)
        let start_x = 1;
        let start_y = 1;
        
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        
        // Start the recursive backtracking algorithm
        stack.push((start_x, start_y));
        visited.insert((start_x, start_y));
        grid[start_y][start_x] = ' ';
        
        // Randomized DFS algorithm
        while let Some((x, y)) = stack.pop() {
            let neighbors = Self::get_unvisited_neighbors(x, y, &visited, actual_width, actual_height);
            
            if !neighbors.is_empty() {
                // Push current cell back onto stack
                stack.push((x, y));
                
                // Choose a random neighbor
                let mut rng = rand::thread_rng();
                let (nx, ny) = neighbors[rng.gen_range(0..neighbors.len())];
                
                // Remove wall between current and chosen neighbor
                let wall_x = (x + nx) / 2;
                let wall_y = (y + ny) / 2;
                grid[wall_y][wall_x] = ' ';
                
                // Mark neighbor as visited and add to path
                visited.insert((nx, ny));
                grid[ny][nx] = ' ';
                stack.push((nx, ny));
            }
        }
        
        // Set entry and exit points
        grid[1][0] = ' ';
        grid[actual_height - 2][actual_width - 1] = ' ';
        
        Self {
            width: actual_width,
            height: actual_height,
            grid,
        }
    }
    
    /// Get unvisited neighbors that are two cells away (for proper wall removal)
    fn get_unvisited_neighbors(
        x: usize,
        y: usize,
        visited: &HashSet<(usize, usize)>,
        width: usize,
        height: usize,
    ) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        
        // Check all four directions (up, right, down, left)
        let directions = [(0, -2), (2, 0), (0, 2), (-2, 0)];
        
        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            // Check bounds and if not visited
            if nx > 0 && nx < (width - 1) as i32 && ny > 0 && ny < (height - 1) as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited.contains(&(nx, ny)) {
                    neighbors.push((nx, ny));
                }
            }
        }
        
        neighbors
    }
    
    /// Display the maze
    pub fn display(&self) {
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
    
    /// Get a reference to the grid
    pub fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
    
    /// Get width
    pub fn width(&self) -> usize {
        self.width
    }
    
    /// Get height
    pub fn height(&self) -> usize {
        self.height
    }
}

fn main() {
    // Generate and display a maze
    let maze = Maze::generate(21, 21);
    maze.display();
    
    println!("\nMaze dimensions: {}x{}", maze.width(), maze.height());
}
