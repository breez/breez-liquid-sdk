use crate::model::*;
use crate::persist::Persister;

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension, Row};

impl Persister {
    pub(crate) fn insert_or_update_ongoing_swap_in(&self, swap_in: OngoingSwapIn) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT OR REPLACE INTO ongoing_send_swaps (
                id,
                invoice,
                payer_amount_sat,
                txid
            )
            VALUES (?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            swap_in.id,
            swap_in.invoice,
            swap_in.payer_amount_sat,
            swap_in.txid,
        ))?;

        Ok(())
    }

    fn list_ongoing_swap_in_query(where_clauses: Vec<&str>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                id,
                invoice,
                payer_amount_sat,
                txid,
                created_at
            FROM ongoing_send_swaps
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_ongoing_swap_in(
        con: &Connection,
        id: &str,
    ) -> rusqlite::Result<Option<OngoingSwapIn>> {
        let query = Self::list_ongoing_swap_in_query(vec!["id = ?1"]);
        con.query_row(&query, [id], Self::sql_row_to_ongoing_swap_in)
            .optional()
    }

    fn sql_row_to_ongoing_swap_in(row: &Row) -> rusqlite::Result<OngoingSwapIn> {
        Ok(OngoingSwapIn {
            id: row.get(0)?,
            invoice: row.get(1)?,
            payer_amount_sat: row.get(2)?,
            txid: row.get(3)?,
        })
    }

    pub(crate) fn list_ongoing_send(
        &self,
        con: &Connection,
        where_clauses: Vec<&str>,
    ) -> rusqlite::Result<Vec<OngoingSwapIn>> {
        let query = Self::list_ongoing_swap_in_query(where_clauses);
        let ongoing_send = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_ongoing_swap_in)?
            .map(|i| i.unwrap())
            .collect();
        Ok(ongoing_send)
    }
}
