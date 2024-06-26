/// Errors that can occur while interacting with the voting contract.
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum VotingError {
    /// Caller is not the admin of the voting.
    NotAdmin,
    /// Caller has already voted.
    AlreadyVoted,
    /// Not authorized to vote.
    NotAuthorized,
    /// Voting hasn't started yet or has already ended.
    VotingNotActive,
    /// Voting cannot be ended (either it hasn't started yet, the deadline hasn't been reached yet,
    /// or it has already ended).
    CannotEndVoting,
}
