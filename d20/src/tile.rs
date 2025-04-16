#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile{
    Start,
    End,
    Wall,
    Space
}

impl From<Tile> for char{
    fn from(value: Tile) -> Self {
        match value{
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Wall => '#',
            Tile::Space => '.',
        }
    }
}