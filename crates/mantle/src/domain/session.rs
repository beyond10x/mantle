//! Session lifecycle, the subset of design § 9 the slice records.
//!
//! The record says what Mantle did. Whether the agent is still running is never read from it:
//! `status` asks Substrate, and an outcome Substrate cannot prove stays unknown.

use std::fmt;
use std::str::FromStr;

use anyhow::{Result, bail};

pub use mantle_worker::{AgentKind, AuthenticationMethod};

pub fn agent_name(agent: &AgentKind) -> &'static str {
    match agent {
        AgentKind::V0 => "claude-code",
        AgentKind::V1 => "codex",
    }
}
pub fn auth_name(auth: &AuthenticationMethod) -> &'static str {
    match auth {
        AuthenticationMethod::V1 => "claude-oauth",
        AuthenticationMethod::V0 => "chatgpt-device",
    }
}
pub fn resolve_identity(
    kind: &str,
    auth: Option<&str>,
) -> Result<(AgentKind, AuthenticationMethod)> {
    let (agent, authentication) = match kind {
        "claude-code" => (AgentKind::V0, AuthenticationMethod::V1),
        "codex" => (AgentKind::V1, AuthenticationMethod::V0),
        _ => bail!("unsupported agent kind"),
    };
    if auth.is_some_and(|value| value != auth_name(&authentication)) {
        bail!("authentication method is incompatible with selected agent");
    }
    Ok((agent, authentication))
}
pub fn validate_identity(agent: &AgentKind, auth: &AuthenticationMethod) -> Result<()> {
    resolve_identity(agent_name(agent), Some(auth_name(auth)))?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Materializing,
    Starting,
    Running,
    Stopping,
    Stopped,
    FailedMaterialization,
    FailedAgentStart,
    Retaining,
    Retained,
    Restarting,
    Destroying,
}

impl SessionState {
    pub fn can_move_to(self, next: Self) -> bool {
        use SessionState::*;
        matches!(
            (self, next),
            (Materializing, Starting | FailedMaterialization | Stopping)
                | (Starting, Running | FailedAgentStart | Stopping)
                | (Running | FailedMaterialization | FailedAgentStart, Stopping)
                | (Stopping, Stopped)
                | (Running | FailedAgentStart | Retained, Retaining)
                | (Retaining, Retained)
                | (Retained, Restarting)
                | (Restarting, Running)
                | (
                    Running | Retained | FailedMaterialization | FailedAgentStart | Stopping,
                    Destroying
                )
                | (Destroying, Stopped)
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
            Self::Retaining => "RETAINING",
            Self::Retained => "RETAINED",
            Self::Restarting => "RESTARTING",
            Self::Destroying => "DESTROYING",
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
            "RETAINING" => Self::Retaining,
            "RETAINED" => Self::Retained,
            "RESTARTING" => Self::Restarting,
            "DESTROYING" => Self::Destroying,
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
