mod migrations;

use anyhow::Result;
use migrations::current_migrations;
use rusqlite::{params, Connection, Row};
use rusqlite_migration::{Migrations, M};

use crate::swap_out::model::OngoingSwap;

pub(crate) struct Persister {
    main_db_file: String,
}

impl Persister {
    pub fn new(working_dir: String) -> Self {
        let main_db_file = format!("{}/storage.sql", working_dir);
        Persister { main_db_file }
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        let con = Connection::open(self.main_db_file.clone())?;
        Ok(con)
    }

    pub fn init(&self) -> Result<()> {
        self.migrate_main_db()?;
        Ok(())
    }

    fn migrate_main_db(&self) -> Result<()> {
        let migrations = Migrations::new(current_migrations().into_iter().map(M::up).collect());
        let mut conn = self.get_connection()?;
        migrations.to_latest(&mut conn)?;
        Ok(())
    }

    pub fn insert_ongoing_swaps(&self, swaps: &[OngoingSwap]) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
                INSERT INTO ongoing_swaps (
                    id,
                    preimage,
                    redeem_script,
                    blinding_key,
                    requested_amount_sat
                )
                VALUES (?, ?, ?, ?, ?)
            ",
        )?;

        for swap in swaps {
            _ = stmt.execute((
                &swap.id,
                &swap.preimage,
                &swap.redeem_script,
                &swap.blinding_key,
                &swap.requested_amount_sat,
            ))?
        }

        Ok(())
    }

    pub fn resolve_ongoing_swap(&self, id: String) -> Result<()> {
        let con = self.get_connection()?;

        con.prepare("DELETE FROM ongoing_swaps WHERE id = ?")?
            .execute(params![id])?;

        Ok(())
    }

    pub fn list_ongoing_swaps(&self) -> Result<Vec<OngoingSwap>> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare("SELECT * FROM ongoing_swaps")?;

        let swaps: Vec<OngoingSwap> = stmt
            .query_map(params![], |row| self.sql_row_to_swap(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(swaps)
    }

    fn sql_row_to_swap(&self, row: &Row) -> Result<OngoingSwap, rusqlite::Error> {
        Ok(OngoingSwap {
            id: row.get(0)?,
            preimage: row.get(1)?,
            redeem_script: row.get(2)?,
            blinding_key: row.get(3)?,
            requested_amount_sat: row.get(4)?,
        })
    }
}
