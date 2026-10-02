//! Session lifecycle, the subset of design § 9 the slice records.
//!
//! The record says what Mantle did. Whether the agent is still running is never read from it:
//! `status` asks Substrate, and an outcome Substrate cannot prove stays unknown.

use std::fmt;
use std::str::FromStr;

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Materializing,
    Starting,
    Running,
    Stopping,
    Stopped,
    FailedMaterialization,
    FailedAgentStart,
}

impl SessionState {
    pub fn can_move_to(self, next: Self) -> bool {
        use SessionState::{
            FailedAgentStart, FailedMaterialization, Materializing, Running, Starting, Stopped,
            Stopping,
        };
        matches!(
            (self, next),
            (Materializing, Starting | FailedMaterialization | Stopping)
                | (Starting, Running | FailedAgentStart | Stopping)
                | (Running | FailedMaterialization | FailedAgentStart, Stopping)
                | (Stopping, Stopped)
        )
    }

    pub fn checked_move(self, next: Self) -> Result<Self> {
        if self.can_move_to(next) {
            Ok(next)
        } else {
            bail!("a session cannot move from {self} to {next}")
        }
    }
}

impl fmt::Display for SessionState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Materializing => "MATERIALIZING",
            Self::Starting => "STARTING",
            Self::Running => "RUNNING",
            Self::Stopping => "STOPPING",
            Self::Stopped => "STOPPED",
            Self::FailedMaterialization => "FAILED_MATERIALIZATION",
            Self::FailedAgentStart => "FAILED_AGENT_START",
        })
    }
}

impl FromStr for SessionState {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self> {
        Ok(match text {
            "MATERIALIZING" => Self::Materializing,
            "STARTING" => Self::Starting,
            "RUNNING" => Self::Running,
            "STOPPING" => Self::Stopping,
            "STOPPED" => Self::Stopped,
            "FAILED_MATERIALIZATION" => Self::FailedMaterialization,
            "FAILED_AGENT_START" => Self::FailedAgentStart,
            other => bail!("unknown session state {other:?} in the state database"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SessionState::*;
    use super::*;

    const ALL: [SessionState; 7] = [
        Materializing,
        Starting,
        Running,
        Stopping,
        Stopped,
        FailedMaterialization,
        FailedAgentStart,
    ];

    #[test]
    fn the_happy_path_is_legal() {
        let mut state = Materializing;
        for next in [Starting, Running, Stopping, Stopped] {
            state = state.checked_move(next).expect("legal move");
        }
    }

    #[test]
    fn stopped_is_final_and_failures_only_stop() {
        for next in ALL {
            assert!(!Stopped.can_move_to(next), "STOPPED -> {next}");
        }
        for failed in [FailedMaterialization, FailedAgentStart] {
            for next in ALL {
                assert_eq!(
                    failed.can_move_to(next),
                    next == Stopping,
                    "{failed} -> {next}"
                );
            }
        }
    }

    #[test]
    fn running_is_never_reached_without_starting() {
        for from in ALL {
            if from != Starting {
                assert!(!from.can_move_to(Running), "{from} -> RUNNING");
            }
        }
    }

    #[test]
    fn names_round_trip() {
        for state in ALL {
            assert_eq!(
                state.to_string().parse::<SessionState>().expect("parses"),
                state
            );
        }
    }
}
