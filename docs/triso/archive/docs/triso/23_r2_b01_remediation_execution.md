> HISTORICAL ARCHIVE: superseded working document. Do not use as the current project source.

# R2-B01 Remediation Execution Plan

## Exact supervisor source under test

Repository: theodoreOnzGit/outram-park-backend
Commit under review: 8d31482d127e211614ebb3f66b1076e1ed6dea98

The test is injected into a temporary checkout only. No commit is made to the supervisor repository.

## Concrete five-layer path

Uniform kernel birth
→ sample_uniform_in_ball(&mut seed, cell.get_fuel_radius())
→ TrisoCell::new_crp6_geometry()
→ WoSWalker::new(start, Nuclide::Cs137, child_rng)
→ WoSWalker::walk_until_released(&cell, &params)
→ walk_until_released repeatedly calls step_multilayer
→ step_multilayer calls does_transmit at internal interfaces
→ outer OPyC arrival returns HopOutcome::Released
→ walk_until_released returns Some(self.time)

The wrapper maps None to CensoredMaxSteps.

## Exact estimator

For history m:

tau_m = first simulated time at which that history reaches the absorbing outer OPyC boundary.

For N equal-weight histories:

F_hat_N(t) = (1/N) sum_m 1[tau_m <= t].

This is the empirical CDF of first-release times.

Animation-frame state is deliberately not used for this estimator.

## Censoring semantics

Released(Time) means the release event occurred and its time is known.

CensoredMaxSteps means max_steps was reached before release. It is not a known non-release.

For the ordinary empirical CDF, require zero censoring:

count(CensoredMaxSteps) = 0.

Only then is F_hat_N(t) = #{tau_m <= t}/N a complete empirical CDF.

No censored-data estimator is introduced in this remediation.

## Focused tests

1. Initial births are inside the kernel and volume-uniform.
2. The five-layer path uses walk_until_released and therefore step_multilayer.
3. A release returns an actual positive simulated release time.
4. A release with tau_m > t is not counted at time t.
5. Max-step censoring is distinguishable and rejected by the complete-sample CDF helper.
6. The empirical CDF is monotone.
7. A zero-censoring absorbing run gives F_hat_N(infinity)=1.

The tests do not attempt Review-4 interface convergence, first-passage-table sensitivity, or reinsertion convergence.

## Execution and closure rule

The Ray repository contains a workflow that checks out the exact supervisor commit in a temporary CI workspace, injects the focused test, and runs the test against the actual supervisor code.

R2-B01 is not considered closed merely because the wrapper exists. Closure requires:

mathematical observable + concrete code path + explicit censoring semantics + passing focused tests.

Until the CI result is observed, status remains R2-B01 = REMEDIATION PREPARED / NOT YET CLOSED.