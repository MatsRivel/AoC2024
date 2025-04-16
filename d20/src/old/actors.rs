use crate::{direction::Direction, matrix::Matrix, position::Position, tile::Tile};


pub trait Movable{
    fn get_pos(&self)->Position<IntType>;
    fn try_move(&self, move_dir: Direction, matrix: &Matrix::<Tile>)->Option<Actor>;
}

#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]
pub enum Actor{
    Etherial(EtherialActor),
    Physical(PhysicalActor)
}
impl Movable for Actor{
    fn get_pos(&self)->Position<IntType> {
        match self{
            Actor::Etherial(etherial_actor) => etherial_actor.get_pos(),
            Actor::Physical(physical_actor) => physical_actor.get_pos(),
        }
    }

    fn try_move(&self, move_dir: Direction, matrix: &Matrix::<Tile>)->Option<Actor> {
        match self{
            Actor::Etherial(etherial_actor) => etherial_actor.try_move(move_dir,matrix),
            Actor::Physical(physical_actor) => physical_actor.try_move(move_dir,matrix),
        }
    }
}

#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]
pub struct EtherialActor{
    pos: Position<IntType>
}
impl EtherialActor{
    pub fn new(pos: Position<IntType>)->Self{
        Self{pos}
    }
}
impl Movable for EtherialActor{
    fn get_pos(&self)->Position<IntType> {
        self.pos
    }

    fn try_move(&self, move_dir: Direction, matrix: &Matrix::<Tile>)->Option<Actor>{
        let current = self.get_pos();
        let next = current + move_dir.as_vector();
        match matrix.get(next){
            Some(Tile::Wall) =>  Some(Actor::Physical(PhysicalActor::new(next))),
            Some(_) =>  Some(Actor::Etherial(EtherialActor::new(next))),
            None => None,
        }
    }
}

#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]
pub struct PhysicalActor{
    pos: Position<IntType>
}
impl PhysicalActor{
    #[allow(unused)]
    pub fn new(pos: Position<IntType>)->Self{
        Self{pos}
    }
}
impl Movable for PhysicalActor{
    fn get_pos(&self)->Position<IntType> {
        self.pos
    }

    fn try_move(&self, move_dir: Direction, matrix: &Matrix::<Tile>)->Option<Actor>{
        let current = self.get_pos();
        let next = current + move_dir.as_vector();
        match matrix.get(next){
            None | Some(Tile::Wall) =>  None,
            Some(_) =>  Some(Actor::Physical(Self::new(next))),
        }
    }
}
impl From<EtherialActor> for PhysicalActor{
    fn from(value: EtherialActor) -> Self {
        PhysicalActor::new(value.pos)
    }
}
