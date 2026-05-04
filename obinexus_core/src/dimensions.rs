//! Dimensional Game Theory: Problem Space Strategy
//!
//! Represents problems along three axes:
//! - Offense: Assertive strategies
//! - Defense: Protective strategies
//! - Attack: Challenging strategies
//!
//! This allows the system to reason strategically about problems
//! rather than just pattern matching.

use serde::{Deserialize, Serialize};

/// Strategy dimension in problem space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StrategyType {
    Offense,  // Assertive, forward-moving
    Defense,  // Protective, defensive
    Attack,   // Challenging, probing
}

/// A point in 3D problem space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalPoint {
    /// Position on offense axis (0.0 to 1.0)
    pub offense: f64,

    /// Position on defense axis (0.0 to 1.0)
    pub defense: f64,

    /// Position on attack axis (0.0 to 1.0)
    pub attack: f64,

    /// Name/description of this point
    pub name: String,
}

impl DimensionalPoint {
    pub fn new(name: String, offense: f64, defense: f64, attack: f64) -> Self {
        DimensionalPoint {
            offense: Self::clamp(offense),
            defense: Self::clamp(defense),
            attack: Self::clamp(attack),
            name,
        }
    }

    fn clamp(value: f64) -> f64 {
        value.max(0.0).min(1.0)
    }

    /// Calculate magnitude (distance from origin)
    pub fn magnitude(&self) -> f64 {
        (self.offense.powi(2) + self.defense.powi(2) + self.attack.powi(2)).sqrt()
    }

    /// Distance to another point
    pub fn distance_to(&self, other: &DimensionalPoint) -> f64 {
        let dx = self.offense - other.offense;
        let dy = self.defense - other.defense;
        let dz = self.attack - other.attack;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

/// The 3D problem space
pub struct DimensionalSpace {
    /// Current position in problem space
    current_point: DimensionalPoint,

    /// Problem history
    history: Vec<DimensionalPoint>,

    /// Strategic constraints
    constraints: StrategicConstraints,
}

/// Strategic constraints that limit the solution space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicConstraints {
    /// Minimum total energy (O + D + A must be >= this)
    pub min_total_energy: f64,

    /// Maximum on any single axis
    pub max_single_axis: f64,

    /// Prefer balanced strategies (penalty for extreme differences)
    pub balance_preference: f64,
}

impl Default for StrategicConstraints {
    fn default() -> Self {
        StrategicConstraints {
            min_total_energy: 0.3,
            max_single_axis: 1.0,
            balance_preference: 0.5, // Moderate preference for balance
        }
    }
}

impl DimensionalSpace {
    pub fn new() -> Self {
        DimensionalSpace {
            current_point: DimensionalPoint::new("initial".to_string(), 0.5, 0.5, 0.5),
            history: Vec::new(),
            constraints: StrategicConstraints::default(),
        }
    }

    /// Move to a new strategic position
    pub fn move_to(&mut self, new_point: DimensionalPoint) -> Result<(), String> {
        // Validate constraints
        self.validate_point(&new_point)?;

        // Record history
        self.history.push(self.current_point.clone());

        // Move to new position
        self.current_point = new_point;

        Ok(())
    }

    /// Validate that a point satisfies constraints
    pub fn validate_point(&self, point: &DimensionalPoint) -> Result<(), String> {
        let total_energy = point.offense + point.defense + point.attack;

        if total_energy < self.constraints.min_total_energy {
            return Err(format!(
                "Insufficient energy: {} < {}",
                total_energy, self.constraints.min_total_energy
            ));
        }

        if point.offense > self.constraints.max_single_axis
            || point.defense > self.constraints.max_single_axis
            || point.attack > self.constraints.max_single_axis
        {
            return Err("Single axis exceeds maximum".to_string());
        }

        Ok(())
    }

    /// Get current position
    pub fn current(&self) -> &DimensionalPoint {
        &self.current_point
    }

    /// Get the best strategy for reaching a target point
    pub fn recommend_strategy(&self, target: &DimensionalPoint) -> StrategyRecommendation {
        let distance = self.current_point.distance_to(target);

        // Determine which axis needs the most movement
        let o_diff = (target.offense - self.current_point.offense).abs();
        let d_diff = (target.defense - self.current_point.defense).abs();
        let a_diff = (target.attack - self.current_point.attack).abs();

        let dominant_strategy = if o_diff >= d_diff && o_diff >= a_diff {
            StrategyType::Offense
        } else if d_diff >= a_diff {
            StrategyType::Defense
        } else {
            StrategyType::Attack
        };

        StrategyRecommendation {
            recommended_strategy: dominant_strategy,
            distance_to_target: distance,
            confidence: 1.0 - distance, // Closer = higher confidence
            next_point: DimensionalPoint::new(
                format!("toward_{:?}", dominant_strategy),
                self.current_point.offense
                    + (target.offense - self.current_point.offense) * 0.1,
                self.current_point.defense
                    + (target.defense - self.current_point.defense) * 0.1,
                self.current_point.attack
                    + (target.attack - self.current_point.attack) * 0.1,
            ),
        }
    }

    /// Get position history
    pub fn history(&self) -> &[DimensionalPoint] {
        &self.history
    }

    /// Reset to initial position
    pub fn reset(&mut self) {
        self.history.clear();
        self.current_point = DimensionalPoint::new("reset".to_string(), 0.5, 0.5, 0.5);
    }
}

impl Default for DimensionalSpace {
    fn default() -> Self {
        Self::new()
    }
}

/// Recommendation for how to solve a problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyRecommendation {
    pub recommended_strategy: StrategyType,
    pub distance_to_target: f64,
    pub confidence: f64,
    pub next_point: DimensionalPoint,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensional_point() {
        let point = DimensionalPoint::new("test".to_string(), 0.5, 0.5, 0.5);
        assert_eq!(point.offense, 0.5);
        assert_eq!(point.magnitude(), (0.75_f64).sqrt());
    }

    #[test]
    fn test_distance_calculation() {
        let p1 = DimensionalPoint::new("p1".to_string(), 0.0, 0.0, 0.0);
        let p2 = DimensionalPoint::new("p2".to_string(), 1.0, 0.0, 0.0);

        let dist = p1.distance_to(&p2);
        assert!((dist - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_dimensional_space() {
        let space = DimensionalSpace::new();
        let target = DimensionalPoint::new("target".to_string(), 0.8, 0.3, 0.6);

        let rec = space.recommend_strategy(&target);
        assert_eq!(rec.recommended_strategy, StrategyType::Offense);
    }

    #[test]
    fn test_constraint_validation() {
        let space = DimensionalSpace::new();
        let invalid = DimensionalPoint::new("invalid".to_string(), 0.0, 0.0, 0.0);

        assert!(space.validate_point(&invalid).is_err()); // Too little energy
    }
}
