pub(crate) fn current_migrations() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS receive_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            preimage TEXT NOT NULL,
            create_response_json TEXT NOT NULL,
            claim_private_key TEXT NOT NULL,
            invoice TEXT NOT NULL,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            claim_fees_sat INTEGER NOT NULL,
            claim_tx_id TEXT,
            lockup_tx_id TEXT,
            state INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS send_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            invoice TEXT NOT NULL UNIQUE,
            preimage TEXT,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            create_response_json TEXT NOT NULL,
            refund_private_key TEXT NOT NULL,
            lockup_tx_id TEXT,
            refund_tx_id TEXT,
            created_at INTEGER NOT NULL,
            state INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS payment_tx_data (
            tx_id TEXT NOT NULL PRIMARY KEY,
            payment_type INTEGER NOT NULL,
            is_confirmed INTEGER NOT NULL,
            timestamp INTEGER,
            amount_sat INTEGER NOT NULL,
            fees_sat INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS chain_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            direction INTEGER NOT NULL,
            claim_address TEXT NOT NULL,
            lockup_address TEXT NOT NULL,
            timeout_block_height INTEGER NOT NULL,
            preimage TEXT NOT NULL,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            accept_zero_conf INTEGER NOT NULL,
            create_response_json TEXT NOT NULL,
            claim_private_key TEXT NOT NULL,
            refund_private_key TEXT NOT NULL,
            server_lockup_tx_id TEXT,
            user_lockup_tx_id TEXT,
            claim_fees_sat INTEGER NOT NULL,
            claim_tx_id TEXT,
            refund_tx_id TEXT,
            created_at INTEGER NOT NULL,
            state INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS cached_items (
            key TEXT NOT NULL PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;",
        "
        ALTER TABLE receive_swaps ADD COLUMN description TEXT;
        ALTER TABLE send_swaps ADD COLUMN description TEXT;
        ALTER TABLE chain_swaps ADD COLUMN description TEXT;
        ",
        "
        ALTER TABLE receive_swaps ADD COLUMN id_hash TEXT;
        ALTER TABLE send_swaps ADD COLUMN id_hash TEXT;
        ALTER TABLE chain_swaps ADD COLUMN id_hash TEXT;
        ",
    ]
}
