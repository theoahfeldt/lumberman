use crate::{
    game::{Branch, Game, PlayerAction},
    game_graphics::{GameObject, GameResources},
    transform::Transform,
};
use rapier3d::{na::Translation3, prelude::*};
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
    rigid_body_set: RigidBodySet,
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
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

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
        let ground_handler = rigid_body_set.insert(ground_body);
        collider_set.insert_with_parent(ground_collider, ground_handler, &mut rigid_body_set);

        /* Create the base log */
        let log_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0., LOG_HALF_HEIGHT, 0.])
            .build();
        let log_collider = ColliderBuilder::cylinder(LOG_HALF_HEIGHT, 0.5)
            .collision_groups(InteractionGroups::new(BASE_GROUP, GROUND_GROUP))
            .build();
        let log_handle = rigid_body_set.insert(log_body);
        collider_set.insert_with_parent(log_collider, log_handle, &mut rigid_body_set);
        let base_log = PhysicsLog {
            handle: log_handle,
            branch: Branch::None,
        };

        /* Create other structures necessary for the simulation. */
        let gravity: Vector<Real> = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let joint_set = JointSet::new();
        let ccd_solver = CCDSolver::new();

        Self {
            base_log,
            flying_logs: VecDeque::new(),
            rigid_body_set,
            colliders: collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            islands: island_manager,
            broad_phase,
            narrow_phase,
            joints: joint_set,
            ccd_solver,
        }
    }

    fn add_new_base_log(&mut self, branch: Branch) {
        let log_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0., 3. * LOG_HALF_HEIGHT, 0.])
            .linvel(vector![0., -5., 0.])
            .ccd_enabled(true)
            //.gravity_scale(3.)
            .build();
        let log_collider = ColliderBuilder::cylinder(LOG_HALF_HEIGHT, 0.5)
            .collision_groups(InteractionGroups::new(BASE_GROUP, GROUND_GROUP))
            .build();
        let base_log = self.rigid_body_set.insert(log_body);
        self.colliders
            .insert_with_parent(log_collider, base_log, &mut self.rigid_body_set);
        self.base_log = PhysicsLog {
            handle: base_log,
            branch,
        };
    }

    pub fn chuck_base_log(&mut self, action: PlayerAction) {
        let v_x = match action {
            PlayerAction::ChopLeft => 1.,
            PlayerAction::ChopRight => -1.,
        };
        let v = vector![v_x, 0.5, 0.] * 6.;
        let body = self.rigid_body_set.get_mut(self.base_log.handle).unwrap();

        body.set_translation(vector![0., LOG_HALF_HEIGHT, 0.], true);
        body.apply_impulse(v, true);
        body.enable_ccd(false);
        body.set_gravity_scale(1., true);
        body.set_angvel(vector![0., 0., 2. * v_x], true);

        for &c in body.colliders() {
            self.colliders
                .get_mut(c)
                .unwrap()
                .set_collision_groups(InteractionGroups::new(FLYING_GROUP, GROUND_GROUP))
        }
        self.flying_logs.push_back(self.base_log.clone());
    }

    fn remove_log(&mut self, log: PhysicsLog) {
        self.rigid_body_set.remove(
            log.handle,
            &mut self.islands,
            &mut self.colliders,
            &mut self.joints,
        );
    }

    // Should be called after game.update()
    pub fn update(&mut self, game: &Game, action: PlayerAction) {
        let lowest_branch = game.tree.front().unwrap();
        self.chuck_base_log(action);
        self.add_new_base_log(*lowest_branch);

        for x in self.flying_logs.clone() {
            if self
                .rigid_body_set
                .get(x.handle)
                .unwrap()
                .translation()
                .norm()
                > 5.
            {
                self.remove_log(x)
            }
        }
        let set = &self.rigid_body_set;
        self.flying_logs.retain(|x| set.contains(x.handle));
        if self.flying_logs.len() > 10 {
            self.flying_logs.pop_front();
        }
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
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
            .rigid_body_set
            .get(self.base_log.handle)
            .unwrap()
            .translation()
            .y;
        let tree = game.tree.clone();
        let tree = tree.iter().enumerate().map(|(i, val)| {
            let model = match val {
                Branch::None => resources.log.clone(),
                Branch::Left => resources.branch_left.clone(),
                Branch::Right => resources.branch_right.clone(),
            };
            let rotation = None;
            let transform = Transform {
                scale: None,
                rotation,
                translation: Some(Translation3::new(0., base + i as f32, 0.)),
            };
            GameObject { model, transform }
        });
        let flying = self.flying_logs.iter().map(|log| {
            let body = self.rigid_body_set.get(log.handle).unwrap();
            let isometry = body.position();
            let transform = Transform {
                scale: None,
                rotation: Some(isometry.rotation),
                translation: Some(isometry.translation),
            };
            let model = match log.branch {
                Branch::None => resources.log.clone(),
                Branch::Left => resources.branch_left.clone(),
                Branch::Right => resources.branch_right.clone(),
            };
            GameObject { model, transform }
        });
        tree.chain(flying).collect()
    }
}
