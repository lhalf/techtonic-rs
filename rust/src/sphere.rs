const CUBE_SIDE_COUNT: usize = 6;

struct Sphere {
    cells: Vec<Cell>,
}

#[derive(Clone)]
struct Cell {
    height: u8,
}

struct Index(usize);

impl Sphere {
    fn new(cube_length: usize) -> Sphere {
        Sphere {
            cells: vec![Cell { height: 0 }; cube_length * cube_length * CUBE_SIDE_COUNT],
        }
    }

    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }
}

impl std::ops::Index<Index> for Sphere {
    type Output = Cell;

    fn index(&self, index: Index) -> &Self::Output {
        &self.cells[index.0]
    }
}

impl std::ops::IndexMut<Index> for Sphere {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.cells[index.0]
    }
}

#[cfg(test)]
mod test {
    use super::Index;
    use super::Sphere;

    #[test]
    fn sphere_with_cube_length_1_has_6_cells() {
        let sphere = Sphere::new(1);
        assert_eq!(sphere.cells().count(), 6)
    }

    #[test]
    fn sphere_with_cube_length_100_has_60000_cells() {
        let sphere = Sphere::new(100);
        assert_eq!(sphere.cells().count(), 60000)
    }

    #[test]
    fn get_cell_correctly_returns_set_cell_value() {
        let mut sphere = Sphere::new(100);
        sphere[Index(5)].height = 147;

        assert_eq!(sphere[Index(5)].height, 147)
    }
}
