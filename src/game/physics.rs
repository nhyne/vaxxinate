use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub struct PhysicsWorld {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometric_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    force_set: DefaultForceGeneratorSet<f64>,
    joint_constraint_set: DefaultJointConstraintSet<f64>,
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0));
        let geometric_world = DefaultGeometricalWorld::new();
        let body_set: DefaultBodySet<f64> = DefaultBodySet::new();
        let collider_set: DefaultColliderSet<f64> = DefaultColliderSet::new();
        let force_set: DefaultForceGeneratorSet<f64> = DefaultForceGeneratorSet::new();
        let joint_constraint_set: DefaultJointConstraintSet<f64> = DefaultJointConstraintSet::new();

        PhysicsWorld {
            mechanical_world,
            geometric_world,
            body_set,
            collider_set,
            force_set,
            joint_constraint_set,
        }
    }

    pub fn body_set(&self) -> &DefaultBodySet<f64> {
        &self.body_set
    }

    pub fn body_set_mut(&mut self) -> &mut DefaultBodySet<f64> {
        &mut self.body_set
    }

    pub fn body_collider_sets_mut(
        &mut self,
    ) -> (&mut DefaultBodySet<f64>, &mut DefaultColliderSet<f64>) {
        (&mut self.body_set, &mut self.collider_set)
    }

    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometric_world,
            &mut self.body_set,
            &mut self.collider_set,
            &mut self.joint_constraint_set,
            &mut self.force_set,
        )
    }

    //    pub fn insert(&mut self, )
}
