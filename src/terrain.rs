use grid_2d::{Coord, Grid, Size};
use rand::Rng;
use crate::world::NpcType;
use rand::{seq::IteratorRandom, seq::SliceRandom};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TerrainTile {
    Player,
    Floor,
    Wall,
    Npc(NpcType),
}

struct Room {
    top_left: Coord,
    size: Size
}

impl Room {
    // Returns a randomly sized room at a random position
    fn choose<R: Rng>(bounds: Size, rng: &mut R) -> Self{
        let width = rng.gen_range(5..11);
        let height = rng.gen_range(5..9);
        let size = Size::new(width, height);
        let top_left_bounds = bounds - size;
        let left = rng.gen_range(0..top_left_bounds.width());
        let top = rng.gen_range(0..top_left_bounds.height());
        let top_left = Coord::new(left as i32, top as i32);
        Self{ top_left, size }
    }

    // Place `n` randomly chosen NPCs at random positions within the room
    fn place_npcs<R: Rng>(&self, n: usize, grid: &mut Grid<Option<TerrainTile>>, rng: &mut R) {
        for coord in self
            .coords()
            .filter(|&coord| grid.get_checked(coord).unwrap() == TerrainTile::Floor)
            .choose_multiple(rng, n)
        {
            let npc_type = if rng.gen_range(0..100) < 80 {
                NpcType::Orc
            } else {
                NpcType::Troll
            };
            *grid.get_checked_mut(coord) = Some(TerrainTile::Npc(npc_type));
        }
    }

    // Returns coord at the center aof the room.
    fn center(&self) -> Coord{
        self.top_left + self.size.to_coord().unwrap() / 2
    }

    // Returns iterator over all the coordinates in ther room in row major order
    fn coords<'a>(&'a self) -> impl 'a + Iterator<Item = Coord> {
        self.size
            .coord_iter_row_major()
            .map(move |coord| self.top_left + coord)
    }

    // Returns true if and only if each cell of grid overlapping this room is none
    fn only_intersects_empty(&self, grid: &Grid<Option<TerrainTile>>) -> bool {
        self.coords().all(|coord| grid.get_checked(coord).is_none())
    }

    // Updates `grid`, setting each cell overlapping this room to `Some(TerrainTile::Floor)`.
    // The top and left sides of the room are set to `Some(TerrainTile::Wall)` instead.
    // This prevents a pair of rooms being placed immediately adjacent to one another.
    fn carve_out(&self, grid: &mut Grid<Option<TerrainTile>>) {
        for coord in self.coords() {
            let cell = grid.get_checked_mut(coord);
            if coord.x == self.top_left.x || coord.y == self.top_left.y {
                *cell = Some(TerrainTile::Wall);
            } else {
                *cell = Some(TerrainTile::Floor);
            }
        }
    }
}

fn carve_corridor(start: Coord, end: Coord, grid: &mut Grid<Option<TerrainTile>>) {
    for i in start.x.min(end.x)..= start.x.max(end.x) {
        let cell = grid.get_checked_mut(Coord { x: i, ..start });
        if *cell == None || *cell == Some(TerrainTile::Wall) {
            *cell = Some(TerrainTile::Floor);
        }
    }

    for i in start.y.min(end.y)..= start.y.max(end.y) {
        let cell = grid.get_checked_mut(Coord { y: i, ..end });
        if *cell == None || *cell == Some(TerrainTile::Wall) {
            *cell = Some(TerrainTile::Floor);
        }
    }
}

pub fn generate_dungeon<R: Rng>(size: Size, rng: &mut R) -> Grid<TerrainTile> {
    let mut grid = Grid::new_copy(size, None);
    let mut room_centers = Vec::new();

    const NPCS_PER_ROOM_DISTRIBUTION: &[usize] = &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4];

    const NUM_ATTEMPTS: usize = 100;
    for _ in 0..NUM_ATTEMPTS {
        let room = Room::choose(size, rng);

        if room.only_intersects_empty(&grid) {
            room.carve_out(&mut grid);

            let room_center = room.center();
            
            if room_centers.is_empty() {
                *grid.get_checked_mut(room_center) = Some(TerrainTile::Player);
            }

            room_centers.push(room_center);

            let &num_npcs = NPCS_PER_ROOM_DISTRIBUTION.choose(rng).unwrap();
            room.place_npcs(num_npcs, &mut grid, rng);
        }
    }

    for window in room_centers.windows(2) {
        carve_corridor(window[0], window[1], &mut grid);
    }

    grid.map(|t| t.unwrap_or(TerrainTile::Wall))
}