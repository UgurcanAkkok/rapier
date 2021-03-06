use super::{PositionSolver, VelocitySolver};
use crate::counters::Counters;
use crate::dynamics::{IntegrationParameters, JointGraphEdge, JointIndex, RigidBodySet};
use crate::geometry::{ContactManifold, ContactManifoldIndex};

pub struct IslandSolver {
    velocity_solver: VelocitySolver,
    position_solver: PositionSolver,
}

impl IslandSolver {
    pub fn new() -> Self {
        Self {
            velocity_solver: VelocitySolver::new(),
            position_solver: PositionSolver::new(),
        }
    }

    pub fn solve_island(
        &mut self,
        island_id: usize,
        counters: &mut Counters,
        params: &IntegrationParameters,
        bodies: &mut RigidBodySet,
        manifolds: &mut [&mut ContactManifold],
        manifold_indices: &[ContactManifoldIndex],
        joints: &mut [JointGraphEdge],
        joint_indices: &[JointIndex],
    ) {
        if manifold_indices.len() != 0 || joint_indices.len() != 0 {
            counters.solver.velocity_assembly_time.resume();
            self.velocity_solver.init_constraints(
                island_id,
                params,
                bodies,
                manifolds,
                &manifold_indices,
                joints,
                &joint_indices,
            );
            counters.solver.velocity_assembly_time.pause();

            counters.solver.velocity_resolution_time.resume();
            self.velocity_solver
                .solve_constraints(island_id, params, bodies, manifolds, joints);
            counters.solver.velocity_resolution_time.pause();

            counters.solver.position_assembly_time.resume();
            self.position_solver.init_constraints(
                island_id,
                params,
                bodies,
                manifolds,
                &manifold_indices,
                joints,
                &joint_indices,
            );
            counters.solver.position_assembly_time.pause();
        }

        counters.solver.velocity_update_time.resume();
        bodies
            .foreach_active_island_body_mut_internal(island_id, |_, rb| rb.integrate(params.dt()));
        counters.solver.velocity_update_time.pause();

        if manifold_indices.len() != 0 || joint_indices.len() != 0 {
            counters.solver.position_resolution_time.resume();
            self.position_solver
                .solve_constraints(island_id, params, bodies);
            counters.solver.position_resolution_time.pause();
        }
    }
}
