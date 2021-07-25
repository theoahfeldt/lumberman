use crate::{
    game::{Branch, Game, PlayerAction},
    game_graphics::{GameObject, GameResources},
    transform,
};
use rand::distributions::Distribution;
use rapier3d::prelude::*;
use statrs::distribution::Normal;
use std::collections::VecDeque;

const LOG_HALF_HEIGHT: f32 = 0.5;
const GROUND_GROUP: u32 = 0b1;
const FLYING_GROUP: u32 = 0b10;
const BASE_GROUP: u32 = 0b100;

#[derive(Debug, Clone)]
struct PhysicsLog {
    handle: RigidBodyHandle,
    branch: Branch,
}

pub struct GamePhysics {
    base_log: PhysicsLog,
    flying_logs: VecDeque<PhysicsLog>,
    rigid_bodies: RigidBodySet,
    colliders: ColliderSet,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    islands: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joints: JointSet,
    ccd_solver: CCDSolver,
}

impl GamePhysics {
    pub fn new() -> Self {
        let mut rigid_bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        /* Create the ground. */
        let half_thickness = 0.1;
        let ground_body = RigidBodyBuilder::new_static()
            .translation(vector![0., -half_thickness, 0.])
            .build();
        let ground_collider = ColliderBuilder::cuboid(100.0, half_thickness, 100.0)
            .collision_groups(InteractionGroups::new(
                GROUND_GROUP,
                BASE_GROUP | FLYING_GROUP,
            ))
            .build();
        let ground_handler = rigid_bodies.insert(ground_body);
        colliders.insert_with_parent(ground_collider, ground_handler, &mut rigid_bodies);

        /* Create the base log */
        let log_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0., LOG_HALF_HEIGHT, 0.])
            .build();
        let log_collider = ColliderBuilder::cylinder(LOG_HALF_HEIGHT, 0.5)
            .collision_groups(InteractionGroups::new(BASE_GROUP, GROUND_GROUP))
            .build();
        let log_handle = rigid_bodies.insert(log_body);
        colliders.insert_with_parent(log_collider, log_handle, &mut rigid_bodies);
        let base_log = PhysicsLog {
            handle: log_handle,
            branch: Branch::None,
        };

        /* Create other structures necessary for the simulation. */
        let gravity: Vector<Real> = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let islands = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let joints = JointSet::new();
        let ccd_solver = CCDSolver::new();

        Self {
            base_log,
            flying_logs: VecDeque::new(),
            rigid_bodies,
            colliders,
            gravity,
            integration_parameters,
            physics_pipeline,
            islands,
            broad_phase,
            narrow_phase,
            joints,
            ccd_solver,
        }
    }

    pub fn reset(&mut self) {
        let body = self.rigid_bodies.get_mut(self.base_log.handle).unwrap();
        body.set_translation(vector![0., LOG_HALF_HEIGHT, 0.], true);
        body.set_linvel(vector![0., 0., 0.], true);
        self.base_log.branch = Branch::None;
        for x in self.flying_logs.clone() {
            self.remove_log(x)
        }
        self.flying_logs.clear();
    }

    fn update_base_log(&mut self, branch: Branch) {
        let body = self.rigid_bodies.get_mut(self.base_log.handle).unwrap();
        body.set_translation(vector![0., 3. * LOG_HALF_HEIGHT, 0.], true);
        body.set_linvel(vector![0., -5., 0.], true);
        self.base_log.branch = branch;
    }

    fn random_velocity(std_dev: f64) -> Vector<Real> {
        let mut r = rand::thread_rng();
        let n = Normal::new(0.0, std_dev).unwrap();
        vector![
            n.sample(&mut r) as f32,
            n.sample(&mut r) as f32,
            n.sample(&mut r) as f32
        ]
    }

    pub fn add_new_flying_log(&mut self, action: PlayerAction, branch: Branch) {
        let v_x = match action {
            PlayerAction::ChopLeft => 1.,
            PlayerAction::ChopRight => -1.,
        };
        let linvel = vector![v_x, 0.5, 0.2] * 7. + Self::random_velocity(0.4);
        let angvel = vector![0., 0., 5. * v_x] + Self::random_velocity(0.6);

        let log_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0., LOG_HALF_HEIGHT + 0.1, 0.])
            .linvel(linvel)
            .angvel(angvel)
            .ccd_enabled(true)
            .build();
        let log_collider = ColliderBuilder::cylinder(LOG_HALF_HEIGHT, 0.5)
            .restitution(0.7)
            .collision_groups(InteractionGroups::new(FLYING_GROUP, GROUND_GROUP))
            .build();
        let log_handle = self.rigid_bodies.insert(log_body);
        self.colliders
            .insert_with_parent(log_collider, log_handle, &mut self.rigid_bodies);
        let log = PhysicsLog {
            handle: log_handle,
            branch,
        };
        self.flying_logs.push_back(log);
    }

    fn remove_log(&mut self, log: PhysicsLog) {
        self.rigid_bodies.remove(
            log.handle,
            &mut self.islands,
            &mut self.colliders,
            &mut self.joints,
        );
    }

    // Should be called after game.update()
    pub fn update(&mut self, game: &Game, action: PlayerAction) {
        let old_branch = self.base_log.branch;
        let &new_branch = game.tree.front().unwrap();
        self.add_new_flying_log(action, old_branch);
        self.update_base_log(new_branch);

        for x in self.flying_logs.clone() {
            if self
                .rigid_bodies
                .get(x.handle)
                .unwrap()
                .translation()
                .norm()
                > 5.
            {
                self.remove_log(x)
            }
        }
        let set = &self.rigid_bodies;
        self.flying_logs.retain(|x| set.contains(x.handle));
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );
        //let ball_body = &self.rigid_body_set[self.base_log.handle];
        //println!("Ball altitude: {}", ball_body.translation().y);
    }

    pub fn make_scene(&self, game: &Game, resources: &GameResources) -> Vec<GameObject> {
        let base = self
            .rigid_bodies
            .get(self.base_log.handle)
            .unwrap()
            .translation()
            .y;
        let tree = game.tree.clone();
        let tree = tree.iter().enumerate().map(|(i, val)| GameObject {
            model: match val {
                Branch::None => resources.log.clone(),
                Branch::Left => resources.branch_left.clone(),
                Branch::Right => resources.branch_right.clone(),
            },
            transform: transform::translation3(0., base + i as f32, 0.),
        });
        let flying = self.flying_logs.iter().map(|log| {
            let body = self.rigid_bodies.get(log.handle).unwrap();
            GameObject {
                model: match log.branch {
                    Branch::None => resources.log.clone(),
                    Branch::Left => resources.branch_left.clone(),
                    Branch::Right => resources.branch_right.clone(),
                },
                transform: body.position().to_homogeneous(),
            }
        });
        tree.chain(flying).collect()
    }
}
