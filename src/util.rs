use entities::Entity;

#[derive(Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl <T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x: x,
            y: y
        }
    }
}

#[derive(Clone)]
pub struct Rect<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T
}

impl <T> Rect<T> {
    pub fn new(top: T, bottom: T, left: T, right: T) -> Rect<T> {
        Rect {
            top: top,
            bottom: bottom,
            left: left,
            right: right
        }
    }
}

pub fn intersect<T: Entity, V: Entity>(first: &T, second: &V) -> bool {
    let first_bound = first.renderable().bounding_box();
    let first_pos = first.position();
    let second_bound = second.renderable().bounding_box();
    let second_pos = second.position();

    first_bound.left + first_pos.x < second_bound.right + second_pos.x && 
    first_bound.right + first_pos.x > second_bound.left + second_pos.x &&
    first_bound.top + first_pos.y < second_bound.bottom + second_pos.y && 
    first_bound.bottom + first_pos.y > second_bound.top + second_pos.y
}

pub fn clean_collection<T: Entity, V: Fn(&T) -> bool>(collection: &mut Vec<T>, condition: V) {
    let mut obsolete : Vec<usize> = Vec::new();
    for (i, element) in collection.iter().enumerate() {
        if condition(element) {
            obsolete.push(i);
        }
    }

    obsolete.sort();
    obsolete.reverse();

    for i in obsolete {
        collection.remove(i);
    }
}