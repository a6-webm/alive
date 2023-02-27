type Tick = u64;
type PId = u32;

enum GndTile {
    Water,
    Grass,
}

enum Tile {
    Empty,
    Pin(PId),
}

struct Pin {
    id: PId,
    loc: (i32, i32),
}

struct World {
    gnd: Vec<Vec<GndTile>>,
    w: Vec<Vec<Tile>>,
}

fn main() {
    let world: Vec<Vec<&dyn Tiler>> = Vec::new();
}
