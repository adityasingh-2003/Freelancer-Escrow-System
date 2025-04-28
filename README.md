# Freelancer Escrow System

## 📌 Project Title
**72. Freelancer Escrow System**

## 📝 Project Description
A decentralized escrow platform built on Stellar Soroban, allowing clients to safely lock payments for freelance jobs and release them upon successful delivery. This ensures trustless, secure, and transparent transactions between freelancers and clients.

## 🌍 Project Vision
To empower freelancers and clients to engage in fair, secure, and dispute-resistant remote work transactions without needing a third-party intermediary.

## ✨ Key Features
- **Create Escrow**: Clients can initiate an escrow contract for a specific job with a defined freelancer and amount.
- **Release Payment**: Clients can approve the job and release funds to the freelancer.
- **Refund Client**: Freelancers can cancel and refund the client if the job is not completed.
- **View Escrow Details**: Anyone can view the current state and metadata of a job escrow.

## 📜 Contract Details

Contract Address: CCDXXNKIGYIMW7L66FLMXKZCVIVO6HGCNTEUZQQ46GGEPM67UIS4V62S

| Function | Description |
|----------|-------------|
| `create_escrow(env, client, freelancer, amount)` | Creates a new escrow contract for a job. |
| `release_payment(env, client, job_id)` | Releases payment from escrow to freelancer. |
| `refund_client(env, freelancer, job_id)` | Cancels the job and refunds the client. |
| `get_escrow(env, job_id)` | Returns escrow details for a specific job. |

---

🛠 Built using [Soroban SDK](https://soroban.stellar.org/docs).  
🤝 Enabling trust in the decentralized freelance economy.
