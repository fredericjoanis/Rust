struct ID {
    id: u64,
}

struct Dependency {
    id: ID,
}

trait Task {
    fn process(&self);
    fn get_dependencies(&self) -> Vec<Dependency>;
}

trait Component {}

struct Entity {
    id: ID,
    components: Vec<Box<Component>>,
}

struct PositionComponent {
    x: f64,
    y: f64,
}

struct MeshComponent;
struct AnimComponent;

impl AnimComponent {
    pub fn process(&self, pos: &mut PositionComponent) {
        pos.x += 1.0;
    }
}

struct PhysicsComponent {
    weight: f32,
    gravity: f32,
}

impl PhysicsComponent {
    pub fn process(&self, pos: &mut PositionComponent) {
        pos.y += 2.0;
    }
}

fn process_dispatch() {}

struct TaskScheduler {}

fn main() {
    let mut position_components: Vec<PositionComponent> = vec![];
    let mut mesh_components: Vec<MeshComponent> = vec![];
    let mut anim_components: Vec<AnimComponent> = vec![];
    let mut physics_components: Vec<PhysicsComponent> = vec![];

    let physic_to_add = PhysicsComponent {
        weight: 1.0,
        gravity: 2.0,
    };
    physics_components.push(physic_to_add);

    let mut pos: PositionComponent = PositionComponent { x: 0.0, y: 0.0 };
    let anim_to_add = AnimComponent;
    anim_components.push(anim_to_add);

    physics_components[0].process(&mut pos);
    anim_components[0].process(&mut pos);

    println!("{0} {1}", pos.x, pos.y);

    println!("{0} - {1}", pos.x, pos.y);
}
