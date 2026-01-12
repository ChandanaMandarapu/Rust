use std::collections::HashMap;

pub struct Account {
    id: u64,
    balance: u64,
    nonce: u64,
}

pub struct Ledger {
    accounts: HashMap<u64, Account>,
}

pub struct Transaction<'a> {
    from: &'a Account,
    to: &'a Account,
    amount: u64,
    signature: &'a [u8],
}

pub struct SignedTransaction<'a> {
    txn: Transaction<'a>,
    signer_pubkey: &'a [u8],
}

impl Ledger {
    pub fn new() -> Self {
        Self { accounts: HashMap::new() }
    }
    
    pub fn get_account<'a>(&'a self, id: u64) -> Option<&'a Account> {
        self.accounts.get(&id)
    }
    
    pub fn get_account_mut<'a>(&'a mut self, id: u64) -> Option<&'a mut Account> {
        self.accounts.get_mut(&id)
    }
}

impl<'a> Transaction<'a> {
    pub fn new(from: &'a Account, to: &'a Account, amount: u64, signature: &'a [u8]) -> Self {
        Self { from, to, amount, signature }
    }
    
    pub fn validate(&self) -> bool {
        self.from.balance >= self.amount && self.from.id != self.to.id
    }
}

pub struct Block<'a> {
    transactions: Vec<Transaction<'a>>,
    prev_hash: &'a [u8],
}

impl<'a> Block<'a> {
    pub fn verify(&self) -> bool {
        for txn in &self.transactions {
            if !txn.validate() {
                return false;
            }
        }
        true
    }
}

pub struct ChainState<'a> {
    ledger: &'a Ledger,
    blocks: Vec<Block<'a>>,
}

impl<'a> ChainState<'a> {
    pub fn get_balance(&self, id: u64) -> Option<u64> {
        self.ledger.get_account(id).map(|a| a.balance)
    }
}

pub struct TxValidator<'a> {
    banned_ids: &'a [u64],
}

impl<'a> TxValidator<'a> {
    pub fn check(&self, txn: &Transaction<'a>) -> Result<(), &'static str> {
        if self.banned_ids.contains(&txn.from.id) || self.banned_ids.contains(&txn.to.id) {
            Err("Banned")
        } else {
            Ok(())
        }
    }
}

pub struct MerkleProof<'a> {
    hashes: Vec<&'a [u8]>,
    root: &'a [u8],
}

impl<'a> MerkleProof<'a> {
    pub fn verify(&self, leaf: &[u8]) -> bool {
        self.root.len() > 0 && leaf.len() > 0
    }
}

pub struct SmartContract<'a> {
    code: &'a [u8],
    storage: HashMap<&'a [u8], &'a [u8]>,
}

impl<'a> SmartContract<'a> {
    pub fn execute(&self, _ctx: &Transaction<'a>) -> Result<(), &'a str> {
        Ok(())
    }
}

pub struct ContractRuntime<'a> {
    contracts: HashMap<u64, SmartContract<'a>>,
}

pub struct TxBuilder<'a> {
    from: Option<&'a Account>,
    to: Option<&'a Account>,
    amount: u64,
}

impl<'a> TxBuilder<'a> {
    pub fn from(&mut self, account: &'a Account) -> &mut Self {
        self.from = Some(account);
        self
    }
    
    pub fn to(&mut self, account: &'a Account) -> &mut Self {
        self.to = Some(account);
        self
    }
    
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = amount;
        self
    }
    
    pub fn build(self, signature: &'a [u8]) -> Option<Transaction<'a>> {
        if let (Some(f), Some(t)) = (self.from, self.to) {
            Some(Transaction {
                from: f, to: t, amount: self.amount, signature
            })
        } else {
            None
        }
    }
}

pub struct MultiSigWallet<'a> {
    owners: Vec<&'a [u8]>,
    threshold: usize,
}

impl<'a> MultiSigWallet<'a> {
    pub fn verify_signature(&self, sigs: &[&'a [u8]]) -> bool {
        sigs.len() >= self.threshold
    }
}

pub struct AuditLog<'a> {
    entries: Vec<&'a Transaction<'a>>,
}

impl<'a> AuditLog<'a> {
    pub fn add(&mut self, txn: &'a Transaction<'a>) {
        self.entries.push(txn);
    }
}

pub struct ConsensusEngine<'a> {
    validators: Vec<&'a Account>,
}

impl<'a> ConsensusEngine<'a> {
    pub fn elect_leader(&self, seed: u64) -> Option<&'a Account> {
        let idx = (seed as usize) % self.validators.len();
        self.validators.get(idx).copied()
    }
}

pub struct Snapshot<'a> {
    state: &'a ChainState<'a>,
    timestamp: u64,
}

pub struct ForkChoice<'a> {
    chain_a: &'a ChainState<'a>,
    chain_b: &'a ChainState<'a>,
}

impl<'a> ForkChoice<'a> {
    pub fn choose(&self) -> &'a ChainState<'a> {
        if self.chain_a.blocks.len() > self.chain_b.blocks.len() {
            self.chain_a
        } else {
            self.chain_b
        }
    }
}

pub struct TokenMetadata<'a> {
    name: &'a str,
    symbol: &'a str,
}

pub struct TokenAccount<'a> {
    owner: &'a Account,
    mint: &'a TokenMetadata<'a>,
    amount: u64,
}

pub struct Loan<'a> {
    borrower: &'a Account,
    lender: &'a Account,
    collateral: &'a TokenAccount<'a>,
}

pub struct OraclePrice<'a> {
    asset: &'a str,
    price: u64,
    source_sig: &'a [u8],
}

pub struct PriceFeed<'a> {
    data: Vec<OraclePrice<'a>>,
}

impl<'a> PriceFeed<'a> {
    pub fn get_latest(&self, asset: &str) -> Option<&OraclePrice<'a>> {
        self.data.iter().rev().find(|p| p.asset == asset)
    }
}

pub struct EphemeralKey<'a> {
    key: &'a [u8],
    expiry: u64,
}

pub trait SignatureVerifier<'a> {
    fn verify(&self, msg: &'a [u8], sig: &'a [u8]) -> bool;
}

pub struct Ed25519Verifier;
impl<'a> SignatureVerifier<'a> for Ed25519Verifier {
    fn verify(&self, _msg: &'a [u8], _sig: &'a [u8]) -> bool {
        true
    }
}

fn main() {
    let _l = Ledger::new();
    println!("Ledger");
}
