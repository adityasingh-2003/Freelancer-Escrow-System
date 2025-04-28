#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, log};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum EscrowStatus {
    Created,
    Released,
    Refunded,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub struct Escrow {
    pub job_id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub amount: u64,
    pub status: EscrowStatus,
}

#[contracttype]
pub enum EscrowKey {
    Job(u64),
    Count,
}

#[contract]
pub struct FreelancerEscrow;

#[contractimpl]
impl FreelancerEscrow {
    pub fn create_escrow(env: Env, client: Address, freelancer: Address, amount: u64) -> u64 {
        client.require_auth();

        let mut count: u64 = env.storage().instance().get(&EscrowKey::Count).unwrap_or(0);
        count += 1;

        let escrow = Escrow {
            job_id: count,
            client: client.clone(),
            freelancer,
            amount,
            status: EscrowStatus::Created,
        };

        env.storage().instance().set(&EscrowKey::Job(count), &escrow);
        env.storage().instance().set(&EscrowKey::Count, &count);

        log!(&env, "Escrow created with ID: {}", count);
        count
    }

    pub fn release_payment(env: Env, client: Address, job_id: u64) -> bool {
        client.require_auth();

        let mut escrow: Escrow = env.storage().instance().get(&EscrowKey::Job(job_id)).expect("Escrow not found");

        if escrow.client != client || escrow.status != EscrowStatus::Created {
            return false;
        }

        escrow.status = EscrowStatus::Released;
        env.storage().instance().set(&EscrowKey::Job(job_id), &escrow);
        log!(&env, "Payment released for job {}", job_id);
        true
    }

    pub fn refund_client(env: Env, freelancer: Address, job_id: u64) -> bool {
        freelancer.require_auth();

        let mut escrow: Escrow = env.storage().instance().get(&EscrowKey::Job(job_id)).expect("Escrow not found");

        if escrow.freelancer != freelancer || escrow.status != EscrowStatus::Created {
            return false;
        }

        escrow.status = EscrowStatus::Refunded;
        env.storage().instance().set(&EscrowKey::Job(job_id), &escrow);
        log!(&env, "Client refunded for job {}", job_id);
        true
    }

    pub fn get_escrow(env: Env, job_id: u64) -> Escrow {
        env.storage().instance().get(&EscrowKey::Job(job_id)).expect("Escrow not found")
    }
}
